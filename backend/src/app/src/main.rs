use axum::Router;
use common::{config::load_config, logger::AppLogger, tracing};
use std::{env, error};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let config = load_config()?;
    AppLogger::setup_logger();

    tracing::info!("Config.toml has been parsed successfully!");

    env::set_var("RUST_LOG", &config.player.trace_level);
    tracing::info!(
        "Log tracing level is set to '{}'. Check env variable RUST_LOG",
        &config.player.trace_level
    );

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.proxy.port)).await?;
    let app_router = Router::new();

    tracing::info!(
        "Proxy server running on port {}",
        listener.local_addr().unwrap().port()
    );
    axum::serve(listener, app_router).await?;
    Ok(())
}
