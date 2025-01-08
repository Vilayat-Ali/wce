use axum::Router;
use common::{config::load_config, logger::AppLogger};
use player::{routes::get_all_routes, AppState};
use sqlx::postgres::PgPoolOptions;
use std::{error, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let config = load_config()?;
    AppLogger::setup_logger();

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.player.db_url)
        .await?;
    tracing::info!("Connection to postgres database established");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.player.port)).await?;
    let app_router = Router::new()
        // logging and tracing middleware
        .layer(AppLogger::get_trace_layer())
        // entry endpoint
        .nest("/player/api", get_all_routes())
        // context store
        .with_state(Arc::new(Mutex::new(AppState {
            config,
            pg_pool,
            player: None,
        })));

    tracing::info!(
        "Server running on port {}",
        listener.local_addr().unwrap().port()
    );
    axum::serve(listener, app_router).await?;
    Ok(())
}
