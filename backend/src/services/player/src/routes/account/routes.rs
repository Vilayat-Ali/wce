use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::AppContext;

#[derive(Debug, Deserialize)]
pub struct SignupPlayerPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub country: String,
    pub password: String,
}

pub async fn signup_player(
    State(ctx): State<AppContext>,
    Json(_payload): Json<SignupPlayerPayload>,
) -> impl IntoResponse {
    let ctx = ctx.clone();
    "signup"
}

pub async fn login_player(
    State(ctx): State<AppContext>,
    Json(payload): Json<SignupPlayerPayload>,
) -> &'static str {
    "login"
}
