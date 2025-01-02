use super::validation::{Player, PlayerBuilder};
use crate::{error::PlayerServiceError, AppContext};
use axum::{extract::State, response::IntoResponse, Json};
use common::tracing;
use serde::Deserialize;

pub async fn signup_player(
    State(ctx): State<AppContext>,
    Json(payload): Json<Player>,
) -> Result<impl IntoResponse, PlayerServiceError> {
    let Player {
        first_name,
        last_name,
        email,
        password,
        github_username,
    } = payload;

    let validated_player = PlayerBuilder::new()
        .set_first_name(first_name)?
        .set_last_name(last_name)?
        .set_email(email)?
        .set_github_username(github_username)?
        .set_password(password)?
        .build();

    tracing::debug!("Payload data for user has been parsed and validated successfully");

    Ok("hello world".into_response())
}

pub async fn login_player(State(ctx): State<AppContext>) -> impl IntoResponse {
    "hello world".into_response()
}
