mod handlers;
mod validation;

use crate::AppContext;
use axum::routing::{post, Router};

pub fn get_account_router() -> Router<AppContext> {
    Router::new().route("/signup", post(handlers::signup_player))
    // .route("/login", get(handlers::login_player))
}
