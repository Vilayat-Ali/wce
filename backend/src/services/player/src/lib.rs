pub mod db;
pub mod middlewares;
pub mod routes;
pub mod utils;

use std::sync::Arc;

use common::config::r#type::Config;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
}

pub type AppContext = Arc<Mutex<AppState>>;
