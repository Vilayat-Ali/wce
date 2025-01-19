use common::config::r#type::Config;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

use crate::error::PlayerServiceError;

impl From<jsonwebtoken::errors::Error> for PlayerServiceError {
    fn from(val: jsonwebtoken::errors::Error) -> Self {
        PlayerServiceError::InternalError(format!("JWTError: {:#?}", val.to_string()))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn generate_auth_token<T>(
    claims: &T,
    config: &Config,
) -> jsonwebtoken::errors::Result<AuthTokens>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let access_token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(config.player.access_token_secret.as_bytes()),
    )?;

    let refresh_token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(config.player.refresh_token_secret.as_bytes()),
    )?;

    Ok(AuthTokens {
        access_token,
        refresh_token,
    })
}
