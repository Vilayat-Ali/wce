pub mod db;
pub mod error;
pub mod middlewares;
pub mod routes;
pub mod utils;

use common::config::r#type::Config;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
}

pub type AppContext = Arc<Mutex<AppState>>;
