use crate::{error::PlayerServiceError, PlayerJWTPayload};
use nutype::nutype;
use serde::{Deserialize, Serialize};
use std::{
    string::String,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSchema {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub rating: i32,
    pub github_username: String,
    pub password: String,
}

impl Into<PlayerJWTPayload> for PlayerSchema {
    fn into(self) -> PlayerJWTPayload {
        let now = SystemTime::now();
        let iat = now.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        let exp = (now + std::time::Duration::new(3600, 0))
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        PlayerJWTPayload {
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            github_username: self.github_username,
            iat,
            exp,
        }
    }
}

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 25),
    derive(Debug, Clone)
)]
pub struct PlayerFirstName(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 25),
    derive(Debug, Clone)
)]
pub struct PlayerLastName(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 50),
    derive(Debug, Clone)
)]
pub struct PlayerEmail(String);

impl From<PlayerEmailError> for PlayerServiceError {
    fn from(value: PlayerEmailError) -> Self {
        Self::ValidationError(value.to_string())
    }
}

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 50),
    derive(Debug, Clone)
)]
pub struct PlayerGithubUsername(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_min = 8, len_char_max = 50),
    derive(Debug, Clone)
)]
pub struct PlayerPassword(String);

impl From<PlayerPasswordError> for PlayerServiceError {
    fn from(value: PlayerPasswordError) -> Self {
        Self::ValidationError(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct CreatePlayer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub github_username: String,
    pub password: String,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerBuilder {
    first_name: Option<PlayerFirstName>,
    last_name: Option<PlayerLastName>,
    email: Option<PlayerEmail>,
    github_username: Option<PlayerGithubUsername>,
    password: Option<PlayerPassword>,
}

impl PlayerBuilder {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn set_first_name<S: Into<String>>(
        &mut self,
        first_name: S,
    ) -> Result<&mut Self, PlayerServiceError> {
        let validated_first_name = PlayerFirstName::try_new(first_name)
            .map_err(|e| PlayerServiceError::ValidationError(e.to_string()))?;
        self.first_name = Some(validated_first_name);
        Ok(self)
    }

    pub fn set_last_name<S: Into<String>>(
        &mut self,
        last_name: S,
    ) -> Result<&mut Self, PlayerServiceError> {
        let validated_last_name = PlayerLastName::try_new(last_name)
            .map_err(|e| PlayerServiceError::ValidationError(e.to_string()))?;
        self.last_name = Some(validated_last_name);
        Ok(self)
    }

    pub fn set_email<S: Into<String>>(
        &mut self,
        email: S,
    ) -> Result<&mut Self, PlayerServiceError> {
        let validated_email = PlayerEmail::try_new(email)
            .map_err(|e| PlayerServiceError::ValidationError(e.to_string()))?;
        self.email = Some(validated_email);
        Ok(self)
    }

    pub fn set_github_username<S: Into<String>>(
        &mut self,
        github_username: S,
    ) -> Result<&mut Self, PlayerServiceError> {
        let validated_github_username = PlayerGithubUsername::try_new(github_username)
            .map_err(|e| PlayerServiceError::ValidationError(e.to_string()))?;
        self.github_username = Some(validated_github_username);
        Ok(self)
    }

    pub fn set_password<S: Into<String>>(
        &mut self,
        password: S,
    ) -> Result<&mut Self, PlayerServiceError> {
        let validated_password = PlayerPassword::try_new(password)
            .map_err(|e| PlayerServiceError::ValidationError(e.to_string()))?;
        self.password = Some(validated_password);
        Ok(self)
    }

    pub fn build(&mut self) -> CreatePlayer {
        let PlayerBuilder {
            first_name,
            last_name,
            email,
            github_username,
            password,
        } = self;

        CreatePlayer {
            first_name: first_name.clone().unwrap().into_inner(),
            last_name: last_name.clone().unwrap().into_inner(),
            email: email.clone().unwrap().into_inner(),
            github_username: github_username.clone().unwrap().into_inner(),
            password: password.clone().unwrap().into_inner(),
        }
    }
}
