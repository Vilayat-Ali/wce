use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerServiceError {
    #[error("Invalid values in payload. {0}")]
    PayloadValidationError(String),
}

#[derive(Debug, Serialize)]
pub struct PlayerServiceErrorResponse {
    pub message: String,
}

impl IntoResponse for PlayerServiceError {
    fn into_response(self) -> Response {
        let error_response = PlayerServiceErrorResponse {
            message: self.to_string(),
        };
        let body = serde_json::to_string(&error_response).unwrap();

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}
