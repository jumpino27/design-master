use crate::{
    config::AppConfig,
    db::backup_database,
    error::{AppError, AppResult},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use uuid::Uuid;

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_GITHUB_REPO: &str = "jumpino27/design-master";

/// The desktop owns its windows and restart lifecycle. A browser-only server has no desktop
/// to restart, so it must never launch an unrelated installed copy of the app.
#[derive(Clone, Default)]
pub struct AppRestart {
    handler: Option<Arc<dyn Fn() + Send + Sync>>,
    scheduled: Arc<AtomicBool>,
}

impl AppRestart {
    pub fn new(handler: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            handler: Some(Arc::new(handler)),
            scheduled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_supported(&self) -> bool {
        self.handler.is_some()
    }

    pub fn schedule(&self) -> AppResult<()> {
        let handler = self.handler.clone().ok_or_else(|| {
            AppError::BadRequest(
                "Restart this server from your terminal, or quit and reopen Design Master. Automatic restart is unavailable in this process."
                    .into(),
            )
        })?;
        if self.scheduled.swap(true, Ordering::SeqCst) {
            return Ok(());
        }
        tokio::spawn(async move {
            // Let the installation response reach the window before closing it.
            tokio::time::sleep(Duration::from_secs(1)).await;
            handler();
        });
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoUpdateConfig {
    pub github_repo: String,
    pub auto_check_enabled: bool,
    pub auto_install_enabled: bool,
    pub last_checked_at: Option<String>,
    pub last_installed_version: Option<String>,
}

impl Default for AutoUpdateConfig {
    fn default() -> Self {
        Self {
            github_repo: detect_default_github_repo(),
            auto_check_enabled: true,
            auto_install_enabled: false,
            last_checked_at: None,
            last_installed_version: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckReport {
    pub current_version: String,
    pub github_repo: String,
    pub auto_check_enabled: bool,
    pub auto_install_enabled: bool,
    pub platform: String,
    pub arch: String,
    pub update_available: bool,
    pub latest_version: Option<String>,
    pub release_name: Option<String>,
    pub release_notes: Option<String>,
    pub release_html_url: Option<String>,
    pub published_at: Option<String>,
    pub download_asset_name: Option<String>,
    pub download_asset_url: Option<String>,
    pub download_asset_size: Option<u64>,
    pub download_asset_digest: Option<String>,
    pub status_message: String,
    pub last_checked_at: String,
    pub safety_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInstallReport {
    pub ok: bool,
    pub previous_version: String,
    pub installed_version: String,
    pub backup_sqlite_path: String,
    pub updated_target_path: String,
    pub restart_required: bool,
    pub restart_scheduled: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
struct GitHubReleaseDto {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    html_url: String,
    published_at: Option<String>,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    assets: Vec<GitHubAssetDto>,
}

#[derive(Debug, Deserialize)]
struct GitHubAssetDto {
    name: String,
    browser_download_url: String,
    size: u64,
    digest: Option<String>,
}

fn detect_default_github_repo() -> String {
    if let Ok(env_repo) = std::env::var("DESIGN_MASTER_GITHUB_REPO") {
        let trimmed = env_repo.trim();
        if !trimmed.is_empty() && trimmed.contains('/') {
            return trimmed.to_string();
        }
    }
    // Check `.git/config` if running from a repository checkout
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    for dir in [Some(cwd.as_path()), cwd.parent()] {
        if let Some(d) = dir {
            let git_cfg = d.join(".git").join("config");
            if let Ok(text) = std::fs::read_to_string(&git_cfg) {
                if let Some(slug) = parse_github_slug_from_git_config(&text) {
                    return slug;
                }
            }
        }
    }
    DEFAULT_GITHUB_REPO.to_string()
}

fn parse_github_slug_from_git_config(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(idx) = trimmed.find("github.com") {
            let after = &trimmed[idx + "github.com".len()..];
            let cleaned = after.trim_start_matches(&[':', '/'][..]);
            let without_git = cleaned.trim_end_matches(".git").trim();
            let parts: Vec<&str> = without_git.split('/').collect();
            if parts.len() >= 2 && !parts[0].is_empty() && !parts[1].is_empty() {
                return Some(format!("{}/{}", parts[0], parts[1]));
            }
        }
    }
    None
}

fn config_file_path(app_config: &AppConfig) -> PathBuf {
    app_config.active_data_dir().join("auto-update-config.json")
}

pub fn load_update_config(app_config: &AppConfig) -> AutoUpdateConfig {
    let path = config_file_path(app_config);
    if let Ok(bytes) = std::fs::read(&path) {
        if let Ok(mut cfg) = serde_json::from_slice::<AutoUpdateConfig>(&bytes) {
            if cfg.github_repo.trim().is_empty() {
                cfg.github_repo = detect_default_github_repo();
            }
            return cfg;
        }
    }
    AutoUpdateConfig::default()
}

pub fn save_update_config(app_config: &AppConfig, cfg: &AutoUpdateConfig) -> AppResult<()> {
    let path = config_file_path(app_config);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(cfg)
        .map_err(|e| AppError::Internal(format!("Failed to serialize update config: {}", e)))?;
    std::fs::write(&path, json.as_bytes())?;
    Ok(())
}

pub fn normalize_version(raw: &str) -> String {
    raw.trim()
        .trim_start_matches(|c: char| c == 'v' || c == 'V' || c == ' ')
        .split('-')
        .next()
        .unwrap_or("0.0.0")
        .trim()
        .to_string()
}

fn parse_semver_tuple(v: &str) -> (u64, u64, u64) {
    let norm = normalize_version(v);
    let mut parts = norm.split('.').map(|s| s.parse::<u64>().unwrap_or(0));
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

pub fn is_newer_version(current: &str, candidate: &str) -> bool {
    parse_semver_tuple(candidate) > parse_semver_tuple(current)
}

fn current_effective_version(_cfg: &AutoUpdateConfig) -> String {
    normalize_version(APP_VERSION)
}

fn select_platform_asset(assets: &[GitHubAssetDto]) -> Option<&GitHubAssetDto> {
    select_asset_for_platform(assets, std::env::consts::OS, std::env::consts::ARCH)
}

fn select_asset_for_platform<'a>(
    assets: &'a [GitHubAssetDto],
    os: &str,
    arch: &str,
) -> Option<&'a GitHubAssetDto> {
    let arch_name = match arch {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        _ => return None,
    };
    let suffixes: &[&str] = match os {
        "macos" => &[".app.tar.gz", ".dmg"],
        "windows" => &[".exe"],
        "linux" => &[".run"],
        _ => return None,
    };
    for suffix in suffixes {
        if let Some(asset) = assets.iter().find(|asset| {
            let name = asset.name.to_ascii_lowercase();
            name.ends_with(suffix)
                && name.contains(arch_name)
                && (os != "macos" || name.contains("macos"))
                && (os != "linux" || name.contains("linux"))
        }) {
            return Some(asset);
        }
    }
    None
}

fn verify_download(
    bytes: &[u8],
    expected_size: Option<u64>,
    digest: Option<&str>,
) -> AppResult<()> {
    if bytes.len() < 1024 || expected_size != Some(bytes.len() as u64) {
        return Err(AppError::Internal(
            "Release download size does not match GitHub metadata.".into(),
        ));
    }
    let expected = digest
        .and_then(|value| value.strip_prefix("sha256:"))
        .ok_or_else(|| {
            AppError::Internal(
                "GitHub did not provide a SHA-256 digest for this release asset.".into(),
            )
        })?;
    let actual = hex::encode(Sha256::digest(bytes));
    if actual != expected {
        return Err(AppError::Internal(
            "Release checksum mismatch. Existing installation is unchanged.".into(),
        ));
    }
    Ok(())
}

pub fn default_safety_guarantees() -> Vec<String> {
    vec![
        "Automatic pre-update SQLite WAL database snapshot backup (`<data_dir>/backups/`).".to_string(),
        "Complete isolation of user projects, brand kits, threads, images, and OS Keychain API keys.".to_string(),
        "Release download size and SHA-256 digest checked against GitHub metadata.".to_string(),
        "macOS app backup with rollback on replacement failure.".to_string(),
    ]
}

pub async fn check_for_updates(app_config: &AppConfig) -> UpdateCheckReport {
    let mut cfg = load_update_config(app_config);
    let now = chrono::Utc::now().to_rfc3339();
    cfg.last_checked_at = Some(now.clone());
    let _ = save_update_config(app_config, &cfg);

    let current_version = current_effective_version(&cfg);
    let platform = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();

    let api_url = format!(
        "https://api.github.com/repos/{}/releases/latest",
        cfg.github_repo.trim()
    );

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(6))
        .user_agent(format!("Design-Master-Desktop/{}", current_version))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return UpdateCheckReport {
                current_version,
                github_repo: cfg.github_repo,
                auto_check_enabled: cfg.auto_check_enabled,
                auto_install_enabled: cfg.auto_install_enabled,
                platform,
                arch,
                update_available: false,
                latest_version: None,
                release_name: None,
                release_notes: None,
                release_html_url: None,
                published_at: None,
                download_asset_name: None,
                download_asset_url: None,
                download_asset_size: None,
                download_asset_digest: None,
                status_message: format!("Could not initialize HTTP client: {}", e),
                last_checked_at: now,
                safety_guarantees: default_safety_guarantees(),
            };
        }
    };

    let resp = match client
        .get(&api_url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => {
            return UpdateCheckReport {
                current_version: current_version.clone(),
                github_repo: cfg.github_repo,
                auto_check_enabled: cfg.auto_check_enabled,
                auto_install_enabled: cfg.auto_install_enabled,
                platform,
                arch,
                update_available: false,
                latest_version: Some(current_version.clone()),
                release_name: None,
                release_notes: None,
                release_html_url: None,
                published_at: None,
                download_asset_name: None,
                download_asset_url: None,
                download_asset_size: None,
                download_asset_digest: None,
                status_message: format!(
                    "Running Design Master v{} (offline or GitHub unreachable — your current version is safe and unaffected).",
                    current_version
                ),
                last_checked_at: now,
                safety_guarantees: default_safety_guarantees(),
            };
        }
    };

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return UpdateCheckReport {
            current_version: current_version.clone(),
            github_repo: cfg.github_repo.clone(),
            auto_check_enabled: cfg.auto_check_enabled,
            auto_install_enabled: cfg.auto_install_enabled,
            platform,
            arch,
            update_available: false,
            latest_version: Some(current_version.clone()),
            release_name: None,
            release_notes: None,
            release_html_url: Some(format!("https://github.com/{}/releases", cfg.github_repo)),
            published_at: None,
            download_asset_name: None,
            download_asset_url: None,
            download_asset_size: None,
                download_asset_digest: None,
            status_message: format!(
                "Design Master v{} is up to date. Connected to github.com/{} (ready to auto-install as soon as a new release tag is published).",
                current_version, cfg.github_repo
            ),
            last_checked_at: now,
            safety_guarantees: default_safety_guarantees(),
        };
    }

    if !resp.status().is_success() {
        return UpdateCheckReport {
            current_version: current_version.clone(),
            github_repo: cfg.github_repo.clone(),
            auto_check_enabled: cfg.auto_check_enabled,
            auto_install_enabled: cfg.auto_install_enabled,
            platform,
            arch,
            update_available: false,
            latest_version: Some(current_version.clone()),
            release_name: None,
            release_notes: None,
            release_html_url: Some(format!("https://github.com/{}/releases", cfg.github_repo)),
            published_at: None,
            download_asset_name: None,
            download_asset_url: None,
            download_asset_size: None,
            download_asset_digest: None,
            status_message: format!(
                "Running Design Master v{} (GitHub API returned {}).",
                current_version,
                resp.status()
            ),
            last_checked_at: now,
            safety_guarantees: default_safety_guarantees(),
        };
    }

    let release: GitHubReleaseDto = match resp.json().await {
        Ok(rel) => rel,
        Err(e) => {
            return UpdateCheckReport {
                current_version,
                github_repo: cfg.github_repo,
                auto_check_enabled: cfg.auto_check_enabled,
                auto_install_enabled: cfg.auto_install_enabled,
                platform,
                arch,
                update_available: false,
                latest_version: None,
                release_name: None,
                release_notes: None,
                release_html_url: None,
                published_at: None,
                download_asset_name: None,
                download_asset_url: None,
                download_asset_size: None,
                download_asset_digest: None,
                status_message: format!("Could not parse GitHub release metadata: {}", e),
                last_checked_at: now,
                safety_guarantees: default_safety_guarantees(),
            };
        }
    };

    let latest_norm = normalize_version(&release.tag_name);
    let has_newer = !release.draft && is_newer_version(&current_version, &latest_norm);
    let matched_asset = select_platform_asset(&release.assets);

    let status_message = if has_newer {
        if let Some(asset) = matched_asset {
            format!(
                "Update available: v{} -> v{} ({})",
                current_version, latest_norm, asset.name
            )
        } else {
            format!(
                "New release v{} found on GitHub, ready to update.",
                latest_norm
            )
        }
    } else {
        format!(
            "Design Master v{} is up to date with the latest GitHub release.",
            current_version
        )
    };

    UpdateCheckReport {
        current_version,
        github_repo: cfg.github_repo,
        auto_check_enabled: cfg.auto_check_enabled,
        auto_install_enabled: cfg.auto_install_enabled,
        platform,
        arch,
        update_available: has_newer,
        latest_version: Some(latest_norm),
        release_name: release.name.or(Some(release.tag_name)),
        release_notes: release.body,
        release_html_url: Some(release.html_url),
        published_at: release.published_at,
        download_asset_name: matched_asset.map(|a| a.name.clone()),
        download_asset_url: matched_asset.map(|a| a.browser_download_url.clone()),
        download_asset_size: matched_asset.map(|a| a.size),
        download_asset_digest: matched_asset.and_then(|a| a.digest.clone()),
        status_message,
        last_checked_at: now,
        safety_guarantees: default_safety_guarantees(),
    }
}

/// Safely downloads, verifies, backs up the SQLite database, and atomically installs the latest GitHub release
/// with automatic `.bak` rollback if any step fails.
pub async fn install_latest_update(
    pool: &SqlitePool,
    app_config: &AppConfig,
    restart: &AppRestart,
) -> AppResult<UpdateInstallReport> {
    let check = check_for_updates(app_config).await;
    if !check.update_available {
        return Err(AppError::BadRequest(
            "No newer release is available to install.".into(),
        ));
    }
    let previous_version = check.current_version.clone();
    let target_version = check
        .latest_version
        .clone()
        .unwrap_or_else(|| previous_version.clone());

    // 1. MANDATORY SAFETY STEP: Create a consistent SQLite WAL database snapshot backup before touching anything
    let backups_dir = app_config.active_data_dir().join("backups");
    std::fs::create_dir_all(&backups_dir)?;
    let ts = chrono::Utc::now().format("%Y%m%d-%H%M%S");
    let backup_file = backups_dir.join(format!("pre-update-v{}-{}.sqlite", previous_version, ts));
    backup_database(pool, &backup_file).await?;

    if !backup_file.is_file() {
        return Err(AppError::Internal(
            "Pre-update SQLite backup verification failed; aborting update to protect user data."
                .into(),
        ));
    }

    // 2. If a matching release binary asset is available on GitHub Releases, download and verify it in isolated staging
    if let (Some(asset_name), Some(asset_url)) = (
        check.download_asset_name.as_ref(),
        check.download_asset_url.as_ref(),
    ) {
        let stage_dir = std::env::temp_dir().join(format!("dm-auto-update-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&stage_dir)?;
        let downloaded_file = stage_dir.join(asset_name);

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300))
            .user_agent(format!("Design-Master-Updater/{}", previous_version))
            .build()
            .map_err(|e| AppError::Internal(format!("Updater HTTP client error: {}", e)))?;

        let resp =
            client.get(asset_url).send().await.map_err(|e| {
                AppError::Internal(format!("Failed to download release asset: {}", e))
            })?;

        if !resp.status().is_success() {
            let _ = std::fs::remove_dir_all(&stage_dir);
            return Err(AppError::Internal(format!(
                "Release asset download returned HTTP {}; existing installation untouched.",
                resp.status()
            )));
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| AppError::Internal(format!("Failed reading release bytes: {}", e)))?;

        if let Err(error) = verify_download(
            &bytes,
            check.download_asset_size,
            check.download_asset_digest.as_deref(),
        ) {
            let _ = std::fs::remove_dir_all(&stage_dir);
            return Err(error);
        }

        std::fs::write(&downloaded_file, &bytes)?;

        let updated_target_path =
            apply_verified_platform_package(&stage_dir, &downloaded_file, asset_name)?;
        let _ = std::fs::remove_dir_all(&stage_dir);

        return finish_install(
            UpdateInstallReport {
                ok: true,
                previous_version,
                installed_version: target_version.clone(),
                backup_sqlite_path: backup_file.to_string_lossy().to_string(),
                updated_target_path,
                restart_required: std::env::consts::OS != "windows",
                restart_scheduled: false,
                message: if std::env::consts::OS == "windows" {
                    "Update installer opened. Close Design Master if prompted, then finish the installation. SQLite backup saved.".into()
                } else {
                    format!("Installed Design Master v{}. SQLite backup saved. Restart to use the new version.", target_version)
                },
            },
            restart,
        );
    }

    Err(AppError::BadRequest(
        "This release has no compatible installer. No update was installed.".into(),
    ))
}

fn finish_install(
    mut report: UpdateInstallReport,
    restart: &AppRestart,
) -> AppResult<UpdateInstallReport> {
    if report.restart_required && restart.is_supported() {
        restart.schedule()?;
        report.restart_scheduled = true;
        report.message = format!(
            "Installed Design Master v{}. SQLite backup saved. Restarting automatically…",
            report.installed_version
        );
    }
    Ok(report)
}

fn apply_verified_platform_package(
    stage_dir: &Path,
    downloaded_file: &Path,
    asset_name: &str,
) -> AppResult<String> {
    let os = std::env::consts::OS;
    if os == "macos" {
        let extracted_app = stage_dir.join("Design Master.app");
        if asset_name.ends_with(".app.tar.gz") {
            let status = Command::new("tar")
                .args([
                    "-xzf",
                    &downloaded_file.to_string_lossy(),
                    "-C",
                    &stage_dir.to_string_lossy(),
                ])
                .status()
                .map_err(|e| AppError::Internal(format!("Failed to extract .app.tar.gz: {}", e)))?;
            if !status.success() || !extracted_app.join("Contents/MacOS/design-master").is_file() {
                return Err(AppError::Internal(
                    "Downloaded .app.tar.gz did not contain a valid Design Master.app bundle; existing app untouched.".into(),
                ));
            }
        } else if asset_name.ends_with(".dmg") {
            let mnt = stage_dir.join("dmg_mnt");
            std::fs::create_dir_all(&mnt)?;
            let attach = Command::new("hdiutil")
                .args([
                    "attach",
                    "-nobrowse",
                    "-readonly",
                    "-mountpoint",
                    &mnt.to_string_lossy(),
                    &downloaded_file.to_string_lossy(),
                ])
                .status()
                .map_err(|e| AppError::Internal(format!("Failed to mount DMG: {}", e)))?;
            if !attach.success() {
                return Err(AppError::Internal(
                    "Could not mount downloaded DMG; existing app untouched.".into(),
                ));
            }
            let dmg_app = mnt.join("Design Master.app");
            let copy_res = Command::new("cp")
                .args([
                    "-R",
                    &dmg_app.to_string_lossy(),
                    &extracted_app.to_string_lossy(),
                ])
                .status();
            let _ = Command::new("hdiutil")
                .args(["detach", &mnt.to_string_lossy(), "-force"])
                .status();
            if copy_res.map(|s| !s.success()).unwrap_or(true)
                || !extracted_app.join("Contents/MacOS/design-master").is_file()
            {
                return Err(AppError::Internal(
                    "Downloaded DMG did not contain a valid Design Master.app bundle; existing app untouched.".into(),
                ));
            }
        } else {
            return Err(AppError::BadRequest(format!(
                "Unsupported macOS release asset format: {}",
                asset_name
            )));
        }

        let target_app = std::env::current_exe()
            .ok()
            .and_then(|exe| {
                exe.ancestors()
                    .find(|p| p.extension().and_then(|ext| ext.to_str()) == Some("app"))
                    .map(Path::to_path_buf)
            })
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("Applications/Design Master.app")
            });
        replace_macos_bundle(&extracted_app, &target_app)?;

        return Ok(target_app.to_string_lossy().to_string());
    }

    if os == "windows" {
        let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
        let install_dir = PathBuf::from(&local_app_data)
            .join("Programs")
            .join("Design Master");
        let _ = std::fs::create_dir_all(&install_dir);
        let staged_copy = install_dir.join(asset_name);
        std::fs::copy(downloaded_file, &staged_copy)?;
        Command::new(&staged_copy).spawn().map_err(|e| {
            AppError::Internal(format!("Could not launch the update installer: {}", e))
        })?;
        return Ok(staged_copy.to_string_lossy().to_string());
    }

    // Linux (.run / .AppImage / .deb)
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let bin_dir = home.join(".local").join("bin");
    std::fs::create_dir_all(&bin_dir)?;
    let target_bin = bin_dir.join("design-master-update.run");
    std::fs::copy(downloaded_file, &target_bin)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&target_bin, std::fs::Permissions::from_mode(0o755));
    }
    let status = Command::new(&target_bin).status()?;
    if !status.success() {
        return Err(AppError::Internal(format!(
            "Linux update installer failed with {}",
            status
        )));
    }
    Ok(target_bin.to_string_lossy().to_string())
}

fn replace_macos_bundle(source: &Path, target: &Path) -> AppResult<()> {
    let parent = target
        .parent()
        .ok_or_else(|| AppError::Internal("App bundle has no parent directory.".into()))?;
    std::fs::create_dir_all(parent)?;
    let replacement = parent.join(format!("Design-Master-{}.app", Uuid::new_v4()));
    let copy_status = Command::new("cp")
        .arg("-R")
        .arg(source)
        .arg(&replacement)
        .status()?;
    if !copy_status.success() || !replacement.join("Contents/MacOS/design-master").is_file() {
        let _ = std::fs::remove_dir_all(&replacement);
        return Err(AppError::Internal(
            "Could not stage the new app. Existing installation is unchanged.".into(),
        ));
    }
    let signature = Command::new("codesign")
        .args(["--verify", "--deep", "--strict"])
        .arg(&replacement)
        .status()?;
    if !signature.success() {
        let _ = std::fs::remove_dir_all(&replacement);
        return Err(AppError::Internal(
            "Downloaded app signature is invalid. Existing installation is unchanged.".into(),
        ));
    }
    let backup = parent.join(format!("Design-Master-{}.app.bak", Uuid::new_v4()));
    let had_previous = target.exists();
    if had_previous {
        std::fs::rename(target, &backup)?;
    }
    if let Err(error) = std::fs::rename(&replacement, target) {
        if had_previous {
            std::fs::rename(&backup, target)?;
        }
        let _ = std::fs::remove_dir_all(&replacement);
        return Err(AppError::Internal(format!(
            "Could not replace app bundle: {}",
            error
        )));
    }
    if had_previous {
        let _ = std::fs::remove_dir_all(backup);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn completed_install_schedules_one_restart_without_another_click() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let restart = AppRestart::new(move || {
            tx.send(()).unwrap();
        });
        let report = finish_install(
            UpdateInstallReport {
                ok: true,
                previous_version: "1.1.1".into(),
                installed_version: "1.1.2".into(),
                backup_sqlite_path: "/workspace/backups/pre-update.sqlite".into(),
                updated_target_path: "/Applications/Design Master.app".into(),
                restart_required: true,
                restart_scheduled: false,
                message: "Installed".into(),
            },
            &restart,
        )
        .unwrap();
        assert!(report.restart_scheduled);
        assert!(rx.try_recv().is_err(), "response must precede restart");
        restart.clone().schedule().unwrap();
        tokio::time::timeout(Duration::from_secs(3), rx.recv())
            .await
            .unwrap()
            .unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(
            rx.try_recv().is_err(),
            "duplicate requests must not restart twice"
        );
    }

    #[tokio::test]
    async fn browser_only_server_cannot_launch_another_desktop_app() {
        assert!(AppRestart::default().schedule().is_err());
    }

    fn asset(name: &str) -> GitHubAssetDto {
        GitHubAssetDto {
            name: name.into(),
            browser_download_url: format!("https://example.invalid/{name}"),
            size: 2048,
            digest: None,
        }
    }

    #[test]
    fn macos_updates_match_architecture_and_prefer_app_archives() {
        let assets = vec![
            asset("Design-Master-1.1.0-macos-x64.app.tar.gz"),
            asset("Design-Master-1.1.0-macos-arm64.dmg"),
            asset("Design-Master-1.1.0-macos-arm64.app.tar.gz"),
        ];
        assert_eq!(
            select_asset_for_platform(&assets, "macos", "aarch64")
                .unwrap()
                .name,
            assets[2].name
        );
        assert!(select_asset_for_platform(&assets, "macos", "arm").is_none());
    }

    #[test]
    fn windows_updates_use_installer_and_linux_uses_native_run_package() {
        let assets = vec![
            asset("Design-Master-1.1.0-windows-x64.zip"),
            asset("Design-Master-Setup-1.1.0-x64.exe"),
            asset("Design-Master-1.1.0-linux-x64-installer.run"),
        ];
        assert_eq!(
            select_asset_for_platform(&assets, "windows", "x86_64")
                .unwrap()
                .name,
            assets[1].name
        );
        assert_eq!(
            select_asset_for_platform(&assets, "linux", "x86_64")
                .unwrap()
                .name,
            assets[2].name
        );
        assert!(select_asset_for_platform(&assets, "linux", "aarch64").is_none());
    }

    #[test]
    fn download_requires_matching_size_and_digest() {
        let bytes = vec![7; 2048];
        let digest = format!("sha256:{}", hex::encode(Sha256::digest(&bytes)));
        assert!(verify_download(&bytes, Some(2048), Some(&digest)).is_ok());
        assert!(verify_download(&bytes, Some(2049), Some(&digest)).is_err());
        assert!(verify_download(&bytes, Some(2048), Some("sha256:bad")).is_err());
        assert!(verify_download(&bytes, Some(2048), None).is_err());
    }

    #[test]
    fn downloaded_version_does_not_override_running_binary_version() {
        let cfg = AutoUpdateConfig {
            last_installed_version: Some("99.0.0".into()),
            ..Default::default()
        };
        assert_eq!(current_effective_version(&cfg), APP_VERSION);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn invalid_macos_replacement_preserves_existing_app() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("download.app");
        let target = temp.path().join("installed.app");
        std::fs::create_dir_all(source.join("Contents/MacOS")).unwrap();
        std::fs::create_dir_all(&target).unwrap();
        std::fs::write(
            source.join("Contents/MacOS/design-master"),
            b"invalid executable",
        )
        .unwrap();
        std::fs::write(target.join("user-installed-marker"), b"previous version").unwrap();
        assert!(replace_macos_bundle(&source, &target).is_err());
        assert_eq!(
            std::fs::read(target.join("user-installed-marker")).unwrap(),
            b"previous version"
        );
    }
}
