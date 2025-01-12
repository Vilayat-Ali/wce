use crate::{error::PlayerServiceError, utils::jwt, AppContext, PlayerJWTPayload};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use common::{
    bcrypt::BcryptHasher,
    response::{SuccessDataResponse, SuccessResponse},
};
use serde::{Deserialize, Serialize};

pub async fn login_player_handler() -> Result<impl IntoResponse, PlayerServiceError> {
    // Returning success with the created player ID
    Ok(Json(SuccessResponse {
        status: StatusCode::CREATED.as_u16(),
        message: "Player Created Successfully".into(),
    }))
}
