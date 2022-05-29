use axum::Server;
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (host, port, router) = veryrezsi::init().await;
    let address = format!("{}:{}", host, port)
        .parse()
        .expect("Could not parse host and port");
    tracing::info!("Starting server on {address}...");
    let _ = Server::bind(&address)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
