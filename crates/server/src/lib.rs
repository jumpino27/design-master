pub mod compiler;
pub mod config;
pub mod context_builder;
pub mod db;
pub mod error;
pub mod handoff;
pub mod image_util;
pub mod models;
pub mod provider;
pub mod routes;
pub mod style_catalog;
pub mod updater;
pub mod worker;

use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{net::TcpListener, sync::broadcast};

/// Starts the Design Master (`design-gen-server`) backend in-process on a local loopback port
/// (defaulting to `127.0.0.1:4318`, or reusing `127.0.0.1:4318` if already running, or binding
/// an OS-assigned loopback port `127.0.0.1:0` if 4318 is occupied by another service).
pub async fn start_embedded_server(
    custom_static_dir: Option<PathBuf>,
    custom_data_dir: Option<PathBuf>,
) -> anyhow::Result<(SocketAddr, Option<broadcast::Sender<()>>)> {
    let mut config = config::AppConfig::from_env()?;
    if let Some(sd) = custom_static_dir {
        if sd.join("index.html").is_file() {
            config.static_dir = sd;
        }
    }
    if let Some(dd) = custom_data_dir {
        std::fs::create_dir_all(&dd)?;
        config.data_dir = dd;
    }

    let listener = match TcpListener::bind(&config.bind_addr).await {
        Ok(l) => l,
        Err(_) => {
            // Check if a Design Master server is already running and healthy on `config.bind_addr`
            let health_url = format!("http://{}/api/health", config.bind_addr);
            if let Ok(resp) = reqwest::Client::new()
                .get(&health_url)
                .timeout(std::time::Duration::from_millis(800))
                .send()
                .await
            {
                if resp.status().is_success() {
                    if let Ok(addr) = config.bind_addr.parse::<SocketAddr>() {
                        tracing::info!(
                            "Reusing existing healthy Design Master backend at http://{}",
                            addr
                        );
                        return Ok((addr, None));
                    }
                }
            }
            // Otherwise bind an OS-assigned free loopback port so the Tauri desktop app always starts
            TcpListener::bind("127.0.0.1:0").await?
        }
    };

    let bound_addr = listener.local_addr()?;
    config.bind_addr = bound_addr.to_string();

    tracing::info!(
        data_dir = %config.active_data_dir().display(),
        bind_addr = %config.bind_addr,
        static_dir = %config.static_dir.display(),
        "Starting embedded Design Master backend inside Tauri"
    );

    let pool = db::init_db_pool(&config.data_dir).await?;
    if !config.env_test_adapter {
        if let Err(e) = db::ensure_seed_example_projects(&pool, &config).await {
            tracing::warn!("Non-fatal warning seeding example projects: {}", e);
        }
    }

    let provider = Arc::new(provider::OpenRouterClient::new());
    let (event_tx, _) = broadcast::channel(512);
    let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

    worker::recover_interrupted_jobs(&pool, &config, &event_tx).await?;
    worker::spawn_job_worker(
        pool.clone(),
        config.clone(),
        provider.clone(),
        event_tx.clone(),
        shutdown_rx,
    );

    // Spawn non-blocking background GitHub Release auto-update check on startup
    {
        let pool_bg = pool.clone();
        let cfg_bg = config.clone();
        tokio::spawn(async move {
            let upd_cfg = updater::load_update_config(&cfg_bg);
            if upd_cfg.auto_check_enabled {
                let report = updater::check_for_updates(&cfg_bg).await;
                if report.update_available && upd_cfg.auto_install_enabled {
                    let _ = updater::install_latest_update(&pool_bg, &cfg_bg).await;
                }
            }
        });
    }

    let state = routes::AppState {
        pool: pool.clone(),
        config: config.clone(),
        provider,
        event_tx,
    };

    let app = routes::build_router(state);
    let shutdown_tx_for_server = shutdown_tx.clone();
    let mut server_shutdown_rx = shutdown_tx_for_server.subscribe();

    tokio::spawn(async move {
        let _ = axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = server_shutdown_rx.recv().await;
            })
            .await;
        pool.close().await;
    });

    Ok((bound_addr, Some(shutdown_tx)))
}
