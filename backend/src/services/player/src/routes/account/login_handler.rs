use crate::error::PlayerServiceError;
use axum::{http::StatusCode, response::IntoResponse, Json};
use common::response::SuccessResponse;

pub async fn login_player_handler() -> Result<impl IntoResponse, PlayerServiceError> {
    // Returning success with the created player ID
    Ok(Json(SuccessResponse {
        status: StatusCode::CREATED.as_u16(),
        message: "Player Created Successfully".into(),
    }))
}
