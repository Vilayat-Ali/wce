use jsonwebtoken::{encode, EncodingKey, Header};

use crate::error::PlayerServiceError;

impl From<jsonwebtoken::errors::Error> for PlayerServiceError {
    fn from(val: jsonwebtoken::errors::Error) -> Self {
        PlayerServiceError::InternalError(format!("JWTError: {:#?}", val.to_string()))
    }
}

pub fn generate_jwt_token<T, S>(claims: &T, secret: S) -> jsonwebtoken::errors::Result<String>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
    S: Into<String>,
{
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.into().as_bytes()),
    )?;

    Ok(token)
}
