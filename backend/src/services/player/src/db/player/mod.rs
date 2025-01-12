pub mod validate;

use crate::{error::PlayerServiceError, utils::jwt::generate_jwt_token, PlayerJWTPayload};

use super::player::validate::Player;
use common::{bcrypt::BcryptHasher, traits::db_trait::DBModel};
use sqlx::Postgres;

#[derive(Debug, Clone)]
pub struct PlayerModel<'a> {
    pool: &'a sqlx::Pool<Postgres>,
}

impl<'a> PlayerModel<'a> {
    pub fn new(pool: &'a sqlx::Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl<'a> DBModel for PlayerModel<'a> {
    type ServiceError = PlayerServiceError;
    type CreatePayload = Player;
    type CreateResponse = (String, String);

    async fn create(
        &self,
        data: Self::CreatePayload,
    ) -> Result<Self::CreateResponse, Self::ServiceError> {
        let Player {
            first_name,
            last_name,
            email,
            password,
            github_username,
            ..
        } = data;

        let hashed_password = BcryptHasher::hash_string(password).map_err(|_| {
            PlayerServiceError::InternalError("Failed to hash user password".into())
        })?;

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| PlayerServiceError::DatabaseError(e.to_string()))?;

        let record = sqlx::query!(
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
                ) RETURNING id;
            "#,
            first_name,
            last_name,
            email,
            github_username,
            hashed_password
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| PlayerServiceError::DatabaseError(e.to_string()))?;

        let jwt_payload = PlayerJWTPayload {
            first_name,
            last_name,
            email,
            github_username,
            id: record.id,
        };

        let access_token = generate_jwt_token(&jwt_payload, "secret")
            .map_err(|e| PlayerServiceError::InternalError(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| PlayerServiceError::DatabaseError(e.to_string()))?;

        Ok((access_token, String::from("refresh_token")))
    }
}
