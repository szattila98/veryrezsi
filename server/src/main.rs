use axum::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (addr, app) = veryrezsi::init().await;
    let _ = Server::bind(&addr).serve(app.into_make_service()).await;
    Ok(())
}
