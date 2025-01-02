use axum::Router;
use common::{config::load_config, logger::AppLogger, tracing};
use player::{routes::get_all_routes, AppState};
use std::{env, error, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

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

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.player.port)).await?;
    let app_router = Router::new()
        // logging and tracing middleware
        .layer(AppLogger::get_trace_layer())
        // entry endpoint
        .nest("/player/api", get_all_routes())
        // context store
        .with_state(Arc::new(Mutex::new(AppState { config })));

    tracing::info!(
        "Server running on port {}",
        listener.local_addr().unwrap().port()
    );
    axum::serve(listener, app_router).await?;
    Ok(())
}
