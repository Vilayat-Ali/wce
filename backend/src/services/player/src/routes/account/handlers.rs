use super::validation::PlayerBuilder;
use crate::{error::PlayerServiceError, utils::jwt, AppContext, PlayerJWTPayload};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use common::response::SuccessDataResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub github_username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignupSuccessResponse {
    access_token: String,
    refresh_token: String,
    player: PlayerJWTPayload,
}

pub async fn signup_player(
    State(ctx): State<AppContext>,
    Json(payload): Json<SignupPayload>,
) -> Result<impl IntoResponse, PlayerServiceError> {
    let ctx = ctx.clone();
    let context = ctx.lock().await;

    let SignupPayload {
        first_name,
        last_name,
        email,
        password,
        github_username,
    } = payload;

    let validated_player = PlayerBuilder::new()
        .set_first_name(first_name.unwrap_or_default())?
        .set_last_name(last_name.unwrap_or_default())?
        .set_email(email.unwrap_or_default())?
        .set_github_username(github_username.unwrap_or_default())?
        .set_password(password.unwrap_or_default())?
        .build();

    tracing::debug!("Payload data for user has been parsed and validated successfully");

    let new_player = sqlx::query!(
        r#"
            INSERT INTO players (
                first_name,
                last_name,
                email,
                github_username,
                password
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            )
            RETURNING id, first_name, last_name, email, github_username, created_at;
        "#,
        validated_player.first_name,
        validated_player.last_name,
        validated_player.email,
        validated_player.github_username,
        validated_player.password
    )
    .fetch_one(&context.pg_pool)
    .await
    .map_err(|e| {
        tracing::error!("{:#?}", e.as_database_error());
        PlayerServiceError::PayloadValidationError("Failed to add user to the database".into())
    })?;

    tracing::info!("Player has been saved to database. Generating JWT tokens");

    let jwt_payload = PlayerJWTPayload {
        id: new_player.id,
        first_name: validated_player.first_name.clone(),
        last_name: validated_player.last_name.clone(),
        email: validated_player.email.clone(),
        github_username: validated_player.github_username.clone(),
    };

    let access_token = jwt::generate_jwt_token(&jwt_payload, "secret").map_err(|e| {
        tracing::error!("Error generating JWT token: {:?}", e);
        return PlayerServiceError::InternalError("Failed to generate JWT token".into());
    });

    let access_token = access_token.unwrap();

    // Returning success with the created player ID
    Ok(Json(SuccessDataResponse {
        status: StatusCode::CREATED.as_u16(),
        message: "Player Created Successfully".into(),
        data: SignupSuccessResponse {
            access_token: access_token.clone(),
            refresh_token: access_token,
            player: jwt_payload,
        },
    }))
}
