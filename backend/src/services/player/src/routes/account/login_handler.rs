use crate::{
    db::player::*, error::PlayerServiceError, utils::jwt::generate_auth_token, AppContext,
    PlayerJWTPayload,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use common::{bcrypt::BcryptHasher, response::SuccessDataResponse};
use serde::{Deserialize, Serialize};
use validate::{PlayerEmail, PlayerPassword};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn login_player_handler(
    State(ctx): State<AppContext>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, PlayerServiceError> {
    let ctx = ctx.clone();
    let context = ctx.lock().await;

    tracing::debug!("Login Payload received: {:#?}", payload);
    let LoginPayload { email, password } = payload;

    let validated_email = PlayerEmail::try_new(email)?.into_inner();
    let validated_password = PlayerPassword::try_new(password)?.into_inner();

    tracing::debug!("Payload data for user has been parsed and validated successfully");

    let player_model = PlayerModel::new(&context.pg_pool);
    let existing_player = player_model.get_by_email(validated_email).await?;

    tracing::debug!("Player saved in the database");

    let is_password_valid =
        BcryptHasher::verify_hash(validated_password, &existing_player.password)
            .map_err(|e| PlayerServiceError::InternalError(e.to_string()))?;

    if !is_password_valid {
        tracing::debug!("Invalid password provided...");
        return Err(PlayerServiceError::Forbidden);
    }

    let auth_tokens =
        generate_auth_token::<PlayerJWTPayload>(&existing_player.into(), &context.config)?;

    // Returning success with the created player ID
    Ok(Json(SuccessDataResponse {
        status: StatusCode::CREATED.as_u16(),
        message: "Player logged in successfully".into(),
        data: auth_tokens,
    }))
}
