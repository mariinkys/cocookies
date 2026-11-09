use std::net::SocketAddr;

use cocookies::infra::{app::create_app, config::AppConfig, jobs, setup};
use tokio::net::TcpListener;
use tracing::info;

// musl's built-in allocator is notoriously slow under contention; mimalloc is fast and returns
// memory to the OS eagerly, which keeps the container's RSS small.
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> anyhow::Result<()> {
    setup::init_tracing();
    let config = AppConfig::from_env()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        // The blocking pool runs bcrypt, PDF rendering and PDF text extraction. A small cap keeps
        // memory bounded under bursts; extra work simply queues.
        .max_blocking_threads(8)
        .build()?
        .block_on(run(config))
}

async fn run(config: AppConfig) -> anyhow::Result<()> {
    let addr = config.bind_addr;
    let app_state = setup::init_app_state(config).await?;
    jobs::spawn(app_state.clone());

    let app = create_app(app_state);
    let listener = TcpListener::bind(addr).await?;
    info!(%addr, version = env!("CARGO_PKG_VERSION"), "Cocookies backend listening");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    info!("shut down cleanly");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut s) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
