pub mod validate;

use crate::{error::PlayerServiceError, utils::jwt::generate_jwt_token, PlayerJWTPayload};

use super::player::validate::{CreatePlayer, PlayerSchema};
use common::{
    bcrypt::BcryptHasher,
    traits::db_trait::{DBModel, ModelFieldValue},
};
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
    type Id = i32;
    type ServiceError = PlayerServiceError;
    type Schema = PlayerSchema;

    type CreatePayload = CreatePlayer;
    type CreateResponse = (String, String);

    async fn create(
        &self,
        data: Self::CreatePayload,
    ) -> Result<Self::CreateResponse, Self::ServiceError> {
        let CreatePlayer {
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

    async fn read_by_id(&self, id: Self::Id) -> Result<Self::Schema, Self::ServiceError>
    where
        Self::Id: std::marker::Send,
    {
        let record = sqlx::query!(
            r#"
                SELECT * FROM players WHERE id = $1;
            "#,
            id
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| PlayerServiceError::DatabaseError(e.to_string()))?;

        Ok(PlayerSchema {
            id: record.id,
            first_name: record.first_name,
            last_name: record.last_name,
            email: record.email,
            password: record.password,
            github_username: record.github_username,
            rating: record.rating.unwrap(),
        })
    }

    async fn update_by_id(
        &self,
        id: Self::Id,
        update_data: std::collections::HashMap<String, ModelFieldValue>,
    ) -> Result<(), Self::ServiceError>
    where
        Self::Id: std::marker::Send,
    {
        let mut query_params: Vec<ModelFieldValue> = Vec::with_capacity(update_data.len());

        let update_query_string = format!(
            r#"
                UPDATE 
                    players
                SET
                    {}
                WHERE id = {};
            "#,
            update_data
                .iter()
                .enumerate()
                .map(|(idx, (field_name, field_val))| {
                    query_params.push(field_val.clone());
                    format!("{} = ${}", field_name, idx + 1)
                })
                .collect::<Vec<String>>()
                .join(", "),
            update_data.len() + 1
        );

        let mut update_query = sqlx::query(&update_query_string);

        for field_val in query_params {
            match field_val {
                ModelFieldValue::IntVal(val) => update_query = update_query.bind(val),
                ModelFieldValue::BoolVal(val) => update_query = update_query.bind(val),
                ModelFieldValue::FloatVal(val) => update_query = update_query.bind(val),
                ModelFieldValue::TextVal(val) => update_query = update_query.bind(val),
            };
        }

        update_query
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| PlayerServiceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_id(&self, id: Self::Id) -> Result<(), Self::ServiceError>
    where
        Self::Id: std::marker::Send,
    {
        sqlx::query!(
            r#"
            DELETE FROM players WHERE id = $1;
        "#,
            id
        )
        .execute(self.pool)
        .await
        .map_err(|e| PlayerServiceError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
