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
mod showcases;
pub mod style_catalog;
pub mod updater;
pub mod worker;

use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{net::TcpListener, sync::broadcast};

/// Starts the Design Master (`design-gen-server`) backend in-process on a local loopback port
/// (defaulting to `127.0.0.1:4318`, or binding an OS-assigned loopback port if it is occupied).
/// Each desktop owns its backend so it cannot attach to an older app during an update.
pub async fn start_embedded_server(
    custom_static_dir: Option<PathBuf>,
    custom_data_dir: Option<PathBuf>,
    restart: updater::AppRestart,
) -> anyhow::Result<(SocketAddr, broadcast::Sender<()>)> {
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
        Err(_) => TcpListener::bind("127.0.0.1:0").await?,
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
    if config.seed_examples && !config.env_test_adapter {
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
        let restart_bg = restart.clone();
        tokio::spawn(async move {
            let upd_cfg = updater::load_update_config(&cfg_bg);
            if upd_cfg.auto_check_enabled {
                let report = updater::check_for_updates(&cfg_bg).await;
                if report.update_available && upd_cfg.auto_install_enabled {
                    if let Err(error) =
                        updater::install_latest_update(&pool_bg, &cfg_bg, &restart_bg).await
                    {
                        tracing::error!(%error, "Automatic update failed");
                    }
                }
            }
        });
    }

    let state = routes::AppState {
        pool: pool.clone(),
        config: config.clone(),
        provider,
        event_tx,
        restart,
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

    Ok((bound_addr, shutdown_tx))
}
