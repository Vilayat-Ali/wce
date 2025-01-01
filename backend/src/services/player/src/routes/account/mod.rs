mod routes;

use crate::AppContext;
use axum::routing::{post, Router};

pub fn get_account_router() -> Router<AppContext> {
    Router::new()
        .route("/signup", post(routes::signup_player))
        .route("/login", post(routes::login_player))
}
