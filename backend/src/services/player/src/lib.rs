pub mod db;
pub mod error;
pub mod middlewares;
pub mod routes;
pub mod utils;

use common::config::r#type::Config;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub pg_pool: Pool<Postgres>,
    pub player: Option<PlayerJWTPayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerJWTPayload {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub github_username: String,
    exp: usize,
    iat: usize,
}

pub type AppContext = Arc<Mutex<AppState>>;
