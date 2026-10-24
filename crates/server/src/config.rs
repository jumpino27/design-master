use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::{
    path::{Component, Path, PathBuf},
    sync::{Arc, RwLock},
};
use uuid::Uuid;

pub const DEFAULT_PLANNER_MODEL: &str = "z-ai/glm-5.3-flash";
pub const DEFAULT_IMAGE_MODEL: &str = "openai/gpt-image-2.5-sunburst";
pub const DEFAULT_QUALITY: &str = "medium";
pub const MAX_RASTER_UPLOAD_BYTES: usize = 20 * 1024 * 1024; // 20 MiB
pub const MAX_RASTER_MEGAPIXELS: u64 = 60_000_000; // 60 MP

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KeyStorageKind {
    Keyring,
    Session,
    Env,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStatus {
    pub configured: bool,
    pub storage_kind: KeyStorageKind,
    pub masked_hint: Option<String>,
    pub keyring_available: bool,
    pub persistence_note: String,
}

#[derive(Clone)]
pub struct SecretManager {
    session_key: Arc<RwLock<Option<(String, KeyStorageKind)>>>,
    keyring_checked: Arc<RwLock<bool>>,
    disable_keyring: bool,
    service_name: String,
}

impl SecretManager {
    pub fn new(disable_keyring: bool) -> Self {
        Self {
            session_key: Arc::new(RwLock::new(None)),
            keyring_checked: Arc::new(RwLock::new(false)),
            disable_keyring,
            service_name: "design-gen-local".to_string(),
        }
    }

    pub fn mask_key(key: &str) -> String {
        let trimmed = key.trim();
        if trimmed.len() <= 8 {
            "••••••••".to_string()
        } else {
            let prefix = &trimmed[..std::cmp::min(6, trimmed.len())];
            let suffix = &trimmed[trimmed.len() - 4..];
            format!("{}••••{}", prefix, suffix)
        }
    }

    pub fn get_key(&self) -> Option<(String, KeyStorageKind)> {
        // 1. Check in-memory cached key first so we never repeatedly hit OS Keychain on Tokio threads
        if let Ok(guard) = self.session_key.read() {
            if let Some((ref k, ref kind)) = *guard {
                if !k.trim().is_empty() {
                    return Some((k.clone(), kind.clone()));
                }
            }
        }

        // 2. Check environment variable next (non-blocking)
        if let Ok(env_key) = std::env::var("OPENROUTER_API_KEY") {
            let trimmed = env_key.trim().to_string();
            if !trimmed.is_empty() && !trimmed.contains("placeholder") {
                return Some((trimmed, KeyStorageKind::Env));
            }
        }

        // 3. Probe OS keyring at most once with a strict 600ms timeout so a macOS Keychain modal after binary recompile never blocks Tokio workers
        let already_checked = self.keyring_checked.read().map(|g| *g).unwrap_or(false);
        if !self.disable_keyring && !already_checked {
            if let Ok(mut cg) = self.keyring_checked.write() {
                *cg = true;
            }
            let svc = self.service_name.clone();
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let res = keyring::Entry::new(&svc, "openrouter_api_key")
                    .ok()
                    .and_then(|e| e.get_password().ok())
                    .filter(|pw| !pw.trim().is_empty());
                let _ = tx.send(res);
            });
            if let Ok(Some(pw)) = rx.recv_timeout(std::time::Duration::from_millis(600)) {
                if let Ok(mut guard) = self.session_key.write() {
                    *guard = Some((pw.clone(), KeyStorageKind::Keyring));
                }
                return Some((pw, KeyStorageKind::Keyring));
            }
        }

        None
    }

    pub fn set_key(&self, raw_key: &str, prefer_session_only: bool) -> AppResult<KeyStatus> {
        let trimmed = raw_key.trim().to_string();
        if trimmed.is_empty() {
            return Err(AppError::Validation("API key cannot be empty.".into()));
        }

        if let Ok(mut cg) = self.keyring_checked.write() {
            *cg = true;
        }

        if !prefer_session_only && !self.disable_keyring {
            let svc = self.service_name.clone();
            let key_clone = trimmed.clone();
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let mut ok = false;
                if let Ok(entry) = keyring::Entry::new(&svc, "openrouter_api_key") {
                    if entry.set_password(&key_clone).is_ok() {
                        if let Ok(readback) = entry.get_password() {
                            if readback == key_clone {
                                ok = true;
                            }
                        }
                    }
                }
                let _ = tx.send(ok);
            });

            if let Ok(true) = rx.recv_timeout(std::time::Duration::from_millis(800)) {
                if let Ok(mut guard) = self.session_key.write() {
                    *guard = Some((trimmed, KeyStorageKind::Keyring));
                }
                return Ok(self.status());
            }
        }

        // Fall back to session-only memory (never plaintext disk)
        if let Ok(mut guard) = self.session_key.write() {
            *guard = Some((trimmed, KeyStorageKind::Session));
        }
        Ok(self.status())
    }

    pub fn clear_key(&self) -> AppResult<KeyStatus> {
        if let Ok(mut guard) = self.session_key.write() {
            *guard = None;
        }
        if let Ok(mut cg) = self.keyring_checked.write() {
            *cg = true;
        }
        if !self.disable_keyring {
            let svc = self.service_name.clone();
            std::thread::spawn(move || {
                if let Ok(entry) = keyring::Entry::new(&svc, "openrouter_api_key") {
                    let _ = entry.delete_credential();
                }
            });
        }
        Ok(self.status())
    }

    pub fn status(&self) -> KeyStatus {
        let keyring_available = !self.disable_keyring
            && keyring::Entry::new(&self.service_name, "openrouter_api_key").is_ok();
        match self.get_key() {
            Some((k, kind)) => {
                let note = match kind {
                    KeyStorageKind::Keyring => {
                        "Stored securely in OS-native keyring.".to_string()
                    }
                    KeyStorageKind::Session => {
                        "Stored in memory for this session only. You will need to supply the key again after restarting Design Gen.".to_string()
                    }
                    KeyStorageKind::Env => {
                        "Loaded from OPENROUTER_API_KEY environment variable.".to_string()
                    }
                    KeyStorageKind::None => String::new(),
                };
                KeyStatus {
                    configured: true,
                    storage_kind: kind,
                    masked_hint: Some(Self::mask_key(&k)),
                    keyring_available,
                    persistence_note: note,
                }
            }
            None => KeyStatus {
                configured: false,
                storage_kind: KeyStorageKind::None,
                masked_hint: None,
                keyring_available,
                persistence_note: "No API key configured. Browsing local projects works offline; add an OpenRouter key to generate designs.".to_string(),
            },
        }
    }
}

#[derive(Clone)]
pub struct AppConfig {
    pub data_dir: PathBuf,
    pub dynamic_data_dir: Option<Arc<RwLock<PathBuf>>>,
    pub bind_addr: String,
    pub static_dir: PathBuf,
    pub csrf_token: String,
    pub secrets: SecretManager,
    pub max_upload_bytes: usize,
    pub max_megapixels: u64,
    pub env_test_adapter: bool,
}

impl AppConfig {
    pub fn from_env() -> AppResult<Self> {
        let data_dir = if let Ok(custom) = std::env::var("DESIGN_GEN_DATA_DIR") {
            let trimmed = custom.trim();
            if !trimmed.is_empty() {
                PathBuf::from(trimmed)
            } else {
                Self::default_os_data_dir()
            }
        } else {
            Self::default_os_data_dir()
        };

        let initial_active_dir =
            Self::read_bootstrap_data_dir(&data_dir).unwrap_or_else(|| data_dir.clone());

        std::fs::create_dir_all(&data_dir)?;
        std::fs::create_dir_all(&initial_active_dir)?;
        std::fs::create_dir_all(initial_active_dir.join("projects"))?;

        let bind_addr =
            std::env::var("DESIGN_GEN_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:4318".to_string());

        let static_dir = std::env::var("DESIGN_GEN_STATIC_DIR")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(Self::default_static_dir);

        let disable_keyring = std::env::var("DESIGN_GEN_DISABLE_KEYRING")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let env_test_adapter = std::env::var("DESIGN_GEN_TEST_ADAPTER")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let csrf_token =
            std::env::var("DESIGN_GEN_CSRF_TOKEN").unwrap_or_else(|_| Uuid::new_v4().to_string());

        Ok(Self {
            data_dir,
            dynamic_data_dir: Some(Arc::new(RwLock::new(initial_active_dir))),
            bind_addr,
            static_dir,
            csrf_token,
            secrets: SecretManager::new(disable_keyring),
            max_upload_bytes: MAX_RASTER_UPLOAD_BYTES,
            max_megapixels: MAX_RASTER_MEGAPIXELS,
            env_test_adapter,
        })
    }

    pub fn active_data_dir(&self) -> PathBuf {
        if let Some(ref lock) = self.dynamic_data_dir {
            if let Ok(guard) = lock.read() {
                if !guard.as_os_str().is_empty() {
                    return guard.clone();
                }
            }
        }
        self.data_dir.clone()
    }

    pub fn set_active_data_dir(&self, new_dir: PathBuf) {
        if let Some(ref lock) = self.dynamic_data_dir {
            if let Ok(mut guard) = lock.write() {
                *guard = new_dir;
            }
        }
    }

    pub fn default_os_data_dir() -> PathBuf {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        // 1. If running in repo root
        if cwd.join("Cargo.toml").is_file() && cwd.join("web").is_dir() {
            return cwd.join("data");
        }
        // 2. If running inside `src-tauri/` or `crates/server/`
        if let Some(parent) = cwd.parent() {
            if parent.join("Cargo.toml").is_file() && parent.join("web").is_dir() {
                return parent.join("data");
            }
            if let Some(grandparent) = parent.parent() {
                if grandparent.join("Cargo.toml").is_file() && grandparent.join("web").is_dir() {
                    return grandparent.join("data");
                }
            }
        }
        // 3. Check relative to executable path if built inside workspace `target/{debug,release}`
        if let Ok(exe) = std::env::current_exe() {
            let mut cur = exe.parent();
            for _ in 0..5 {
                if let Some(dir) = cur {
                    if dir.join("Cargo.toml").is_file() && dir.join("web").is_dir() {
                        return dir.join("data");
                    }
                    cur = dir.parent();
                }
            }
        }
        // 4. If `cwd/data/design-gen.sqlite` already exists (e.g. portable folder), use `cwd/data`;
        //    otherwise use the standard OS user application data directory for packaged installers.
        if cwd != Path::new("/") && cwd.join("data").join("design-gen.sqlite").is_file() {
            return cwd.join("data");
        }
        dirs::data_local_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("DesignMaster")
            .join("data")
    }

    pub fn default_static_dir() -> PathBuf {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut candidates = vec![cwd.join("web").join("out")];
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.join("web").join("out"));
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(exe_dir) = exe.parent() {
                // Bundled Tauri resource paths across macOS (.app/Contents/Resources), Windows, and Linux
                candidates.push(exe_dir.join("resources").join("web-out"));
                candidates.push(exe_dir.join("_up_").join("web").join("out"));
                candidates.push(exe_dir.join("web-out"));
                if let Some(parent) = exe_dir.parent() {
                    candidates.push(parent.join("Resources").join("web-out"));
                    candidates.push(
                        parent
                            .join("Resources")
                            .join("_up_")
                            .join("web")
                            .join("out"),
                    );
                    candidates.push(parent.join("lib").join("design-master").join("web-out"));
                    candidates.push(
                        parent
                            .join("lib")
                            .join("design-master")
                            .join("_up_")
                            .join("web")
                            .join("out"),
                    );
                }
                let mut cur = Some(exe_dir);
                for _ in 0..5 {
                    if let Some(d) = cur {
                        candidates.push(d.join("web").join("out"));
                        cur = d.parent();
                    }
                }
            }
        }
        for c in candidates {
            if c.join("index.html").is_file() {
                return c;
            }
        }
        PathBuf::from("web/out")
    }

    pub fn read_bootstrap_data_dir(base_data_dir: &Path) -> Option<PathBuf> {
        let local_bootstrap = base_data_dir.join("design-gen-bootstrap.json");
        if let Ok(bytes) = std::fs::read(&local_bootstrap) {
            if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                if let Some(custom_dir) = val.get("data_dir").and_then(|v| v.as_str()) {
                    let p = PathBuf::from(custom_dir);
                    if p.is_absolute() {
                        return Some(p);
                    }
                }
            }
        }
        None
    }

    pub fn validate_and_prepare_data_dir(candidate: &str) -> AppResult<PathBuf> {
        let trimmed = candidate.trim();
        if trimmed.is_empty() {
            return Err(AppError::Validation(
                "Data directory path cannot be empty.".into(),
            ));
        }
        let path = PathBuf::from(trimmed);
        if !path.is_absolute() {
            return Err(AppError::Validation(
                "Data directory must be an absolute path.".into(),
            ));
        }
        for comp in path.components() {
            if matches!(comp, Component::ParentDir) {
                return Err(AppError::Validation(
                    "Data directory path must not contain '..' traversal segments.".into(),
                ));
            }
        }
        std::fs::create_dir_all(&path).map_err(|e| {
            AppError::Validation(format!(
                "Cannot create or access directory '{}': {}",
                trimmed, e
            ))
        })?;
        // Verify write access with a probe file
        let probe = path.join(".design-gen-write-probe");
        std::fs::write(&probe, b"ok").map_err(|e| {
            AppError::Validation(format!("Directory '{}' is not writable: {}", trimmed, e))
        })?;
        let _ = std::fs::remove_file(&probe);
        Ok(path)
    }

    pub fn persist_and_switch_data_dir(&self, new_dir: &Path) -> AppResult<()> {
        let old_dir = self.active_data_dir();
        std::fs::create_dir_all(new_dir.join("projects"))?;

        // Copy existing projects directory to the new path if changing directories
        if old_dir != new_dir && old_dir.join("projects").exists() {
            let _ = copy_dir_recursive(&old_dir.join("projects"), &new_dir.join("projects"));
        }

        let _ = std::fs::create_dir_all(&self.data_dir);
        let bootstrap_json = serde_json::json!({
            "data_dir": new_dir.to_string_lossy(),
            "updated_at": chrono::Utc::now().to_rfc3339()
        });
        let payload = serde_json::to_string_pretty(&bootstrap_json).unwrap_or_default();
        let _ = std::fs::write(
            self.data_dir.join("design-gen-bootstrap.json"),
            payload.as_bytes(),
        );
        let cwd_data = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("data");
        if cwd_data != self.data_dir {
            let _ = std::fs::create_dir_all(&cwd_data);
            let _ = std::fs::write(
                cwd_data.join("design-gen-bootstrap.json"),
                payload.as_bytes(),
            );
        }

        self.set_active_data_dir(new_dir.to_path_buf());
        Ok(())
    }

    /// Resolves a relative path stored in SQLite safely inside the active data directory.
    /// Prevents any `..` traversal or symlink escapes outside `active_data_dir`.
    pub fn resolve_rel_path(&self, rel_path: &str) -> AppResult<PathBuf> {
        if rel_path.is_empty() {
            return Err(AppError::BadRequest("Empty relative path".into()));
        }
        let rel = Path::new(rel_path);
        if rel.is_absolute() {
            return Err(AppError::Forbidden(
                "Relative database path cannot be absolute.".into(),
            ));
        }
        for comp in rel.components() {
            match comp {
                Component::Normal(_) => {}
                _ => {
                    return Err(AppError::Forbidden(
                        "Path traversal or non-normal path segment rejected.".into(),
                    ))
                }
            }
        }
        let base_dir = self.active_data_dir();
        let joined = base_dir.join(rel);
        // If parent exists, canonicalize to ensure no symlink escape outside canonical base_dir
        if let (Ok(canon_base), Some(parent)) = (base_dir.canonicalize(), joined.parent()) {
            if let Ok(canon_parent) = parent.canonicalize() {
                if !canon_parent.starts_with(&canon_base) {
                    return Err(AppError::Forbidden(
                        "Symlink escape outside configured data directory blocked.".into(),
                    ));
                }
            }
        }
        Ok(joined)
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &dst_path)?;
        } else if !dst_path.exists() {
            let _ = std::fs::copy(entry.path(), dst_path);
        }
    }
    Ok(())
}
