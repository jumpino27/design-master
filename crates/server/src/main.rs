use design_gen_server::{
    config::AppConfig,
    db::{ensure_seed_example_projects, init_db_pool},
    provider::OpenRouterClient,
    routes::{build_router, AppState},
    worker::{recover_interrupted_jobs, spawn_job_worker},
};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::broadcast};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "design_gen_server=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env()?;
    tracing::info!(
        data_dir = %config.active_data_dir().display(),
        bind_addr = %config.bind_addr,
        static_dir = %config.static_dir.display(),
        "Initializing Design Master local backend"
    );

    // Bind TCP listener FIRST so a duplicate process exits immediately without touching SQLite
    let listener = TcpListener::bind(&config.bind_addr).await?;
    tracing::info!("Design Master listening on http://{}", config.bind_addr);

    let pool = init_db_pool(&config.data_dir).await?;
    if config.seed_examples && !config.env_test_adapter {
        if let Err(e) = ensure_seed_example_projects(&pool, &config).await {
            tracing::warn!("Non-fatal warning seeding example projects: {}", e);
        }
    }
    let provider = Arc::new(OpenRouterClient::new());
    let (event_tx, _) = broadcast::channel(512);
    let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

    // Recover any interrupted or journal-saved jobs from previous process execution
    recover_interrupted_jobs(&pool, &config, &event_tx).await?;

    // Start background SQLite-backed job queue worker
    spawn_job_worker(
        pool.clone(),
        config.clone(),
        provider.clone(),
        event_tx.clone(),
        shutdown_rx,
    );

    let state = AppState {
        pool: pool.clone(),
        config: config.clone(),
        provider,
        event_tx,
        restart: Default::default(),
    };

    let app = build_router(state);

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = tokio::signal::ctrl_c().await;
            tracing::info!("Received shutdown signal, stopping workers...");
            let _ = shutdown_tx.send(());
        })
        .await?;

    pool.close().await;
    Ok(())
}
