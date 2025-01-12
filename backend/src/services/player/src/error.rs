use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerServiceError {
    #[error("Validation Error: {0}")]
    ValidationError(String),
    #[error("Server Error: {0}")]
    InternalError(String),
    #[error("Database Error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, Serialize)]
pub struct PlayerServiceErrorResponse {
    pub message: String,
}

impl IntoResponse for PlayerServiceError {
    fn into_response(self) -> Response {
        let (status_code, body) = match self {
            Self::ValidationError(err_message) => {
                let error_response = PlayerServiceErrorResponse {
                    message: err_message.to_string(),
                };

                (
                    StatusCode::BAD_REQUEST,
                    serde_json::to_string(&error_response).unwrap(),
                )
            }
            Self::InternalError(err_message) => {
                let error_response = PlayerServiceErrorResponse {
                    message: err_message.to_string(),
                };

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    serde_json::to_string(&error_response).unwrap(),
                )
            }
            Self::DatabaseError(err_message) => {
                let error_response = PlayerServiceErrorResponse {
                    message: err_message.to_string(),
                };

                (
                    StatusCode::EXPECTATION_FAILED,
                    serde_json::to_string(&error_response).unwrap(),
                )
            }
        };

        Response::builder()
            .status(status_code)
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}
