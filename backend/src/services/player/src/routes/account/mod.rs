mod login_handler;
mod signup_handler;

use crate::AppContext;
use axum::routing::{post, Router};

pub fn get_account_router() -> Router<AppContext> {
    Router::new()
        .route("/signup", post(signup_handler::signup_player_handler))
        .route("/login", post(login_handler::login_player_handler))
}
