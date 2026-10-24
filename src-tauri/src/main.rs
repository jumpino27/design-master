#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::PathBuf, sync::Mutex};
use tauri::{Manager, RunEvent, WebviewUrl, WebviewWindowBuilder};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct EmbeddedServerHandle {
    shutdown_tx: Mutex<Option<broadcast::Sender<()>>>,
}

fn find_bundled_static_dir(resource_dir: Option<PathBuf>) -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(res) = resource_dir {
        candidates.push(res.join("_up_").join("web").join("out"));
        candidates.push(res.join("web").join("out"));
        candidates.push(res.join("web-out"));
        candidates.push(res);
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join("web").join("out"));
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.join("web").join("out"));
        }
    }
    for c in candidates {
        if c.join("index.html").is_file() {
            return Some(c);
        }
    }
    None
}

async fn is_dev_frontend_alive(url: &str) -> bool {
    if let Ok(resp) = reqwest::Client::new()
        .get(url)
        .timeout(std::time::Duration::from_millis(450))
        .send()
        .await
    {
        return resp.status().is_success() || resp.status().is_redirection();
    }
    false
}

fn main() {
    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "design_gen_server=info,design_master_desktop=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init();

    let app = tauri::Builder::default()
        .setup(|app| {
            let resource_dir = app.path().resource_dir().ok();
            let static_dir = find_bundled_static_dir(resource_dir);

            let (bound_addr, shutdown_tx, use_dev_url) =
                tauri::async_runtime::block_on(async move {
                    let (addr, tx) =
                        design_gen_server::start_embedded_server(static_dir, None).await?;
                    let dev_alive = if cfg!(debug_assertions)
                        || std::env::var("DESIGN_MASTER_USE_DEV_SERVER").as_deref() == Ok("1")
                    {
                        is_dev_frontend_alive("http://localhost:3000").await
                    } else {
                        false
                    };
                    Ok::<_, anyhow::Error>((addr, tx, dev_alive))
                })?;

            app.manage(EmbeddedServerHandle {
                shutdown_tx: Mutex::new(shutdown_tx),
            });

            let target_url_str = if use_dev_url {
                "http://localhost:3000".to_string()
            } else {
                format!("http://{}", bound_addr)
            };

            tracing::info!(
                window_url = %target_url_str,
                backend_addr = %bound_addr,
                "Opening Design Master native Tauri desktop window"
            );

            let parsed_url: tauri::Url = target_url_str.parse()?;
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(parsed_url))
                .title("Design Master (DM) — AI Design Workbench")
                .inner_size(1520.0, 940.0)
                .min_inner_size(1100.0, 720.0)
                .resizable(true)
                .center()
                .build()?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building Design Master Tauri application");

    app.run(|app_handle, event| {
        if let RunEvent::Exit = event {
            if let Some(state) = app_handle.try_state::<EmbeddedServerHandle>() {
                if let Ok(mut guard) = state.shutdown_tx.lock() {
                    if let Some(tx) = guard.take() {
                        let _ = tx.send(());
                    }
                }
            }
        }
    });
}
