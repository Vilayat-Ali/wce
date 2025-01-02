pub mod account;

use crate::AppContext;
use axum::Router;

// routes
use account::get_account_router;

pub fn get_all_routes() -> Router<AppContext> {
    Router::new()
        // account routes
        .nest("/account", get_account_router())
}
