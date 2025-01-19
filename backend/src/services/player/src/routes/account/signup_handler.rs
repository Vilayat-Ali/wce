use crate::{db::player::*, error::PlayerServiceError, utils::jwt::AuthTokens, AppContext};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use common::{response::SuccessDataResponse, traits::db_trait::DBModel};
use serde::{Deserialize, Serialize};
use validate::PlayerBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub github_username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
struct SignupSuccessResponse {
    data: AuthTokens,
}

pub async fn signup_player_handler(
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

    let validated_player = PlayerBuilder::init()
        .set_first_name(first_name.unwrap_or_default())?
        .set_last_name(last_name.unwrap_or_default())?
        .set_email(email.unwrap_or_default())?
        .set_github_username(github_username.unwrap_or_default())?
        .set_password(password.unwrap_or_default())?
        .build();

    tracing::debug!("Payload data for user has been parsed and validated successfully");

    let player_model = PlayerModel::new(&context.pg_pool);
    let auth_tokens = player_model
        .create(validated_player, Some(&context.config))
        .await?;

    tracing::debug!("Player saved in the database");

    // Returning success with the created player ID
    Ok(Json(SuccessDataResponse {
        status: StatusCode::CREATED.as_u16(),
        message: "Player Created Successfully".into(),
        data: SignupSuccessResponse { data: auth_tokens },
    }))
}
