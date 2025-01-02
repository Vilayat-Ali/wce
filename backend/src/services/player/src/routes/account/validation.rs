use nutype::nutype;
use serde::Serialize;

use crate::error::PlayerServiceError;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub github_username: String,
    pub password: String,
}

impl Player {
    pub fn get_full_name(&self) -> String {
        format!("{} {}", &self.first_name, &self.last_name)
    }
}

#[nutype(sanitize(trim), validate(not_empty, len_char_max = 25))]
pub struct PlayerFirstName(String);

#[nutype(sanitize(trim), validate(not_empty, len_char_max = 25))]
pub struct PlayerLastName(String);

#[nutype(sanitize(trim), validate(not_empty, len_char_max = 50))]
pub struct PlayerEmail(String);

#[nutype(sanitize(trim), validate(not_empty, len_char_max = 50))]
pub struct PlayerGithubUsername(String);

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_min = 8, len_char_max = 50)
)]
pub struct PlayerPassword(String);

pub struct PlayerBuilder {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub github_username: Option<String>,
    pub password: Option<String>,
}

impl Default for PlayerBuilder {
    fn default() -> Self {
        Self {
            first_name: None,
            last_name: None,
            email: None,
            github_username: None,
            password: None,
        }
    }
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_first_name<S>(&mut self, first_name: S) -> Result<&mut Self, PlayerServiceError>
    where
        S: Into<String>,
    {
        match PlayerFirstName::try_new(first_name.into()) {
            Ok(player_first_name) => {
                self.first_name = Some(player_first_name.into_inner());
                return Ok(self);
            }
            Err(e) => return Err(PlayerServiceError::PayloadValidationError(e.to_string())),
        }
    }

    pub fn set_last_name<S>(&mut self, last_name: S) -> Result<&mut Self, PlayerServiceError>
    where
        S: Into<String>,
    {
        match PlayerLastName::try_new(last_name.into()) {
            Ok(player_last_name) => {
                self.last_name = Some(player_last_name.into_inner());
                return Ok(self);
            }
            Err(e) => return Err(PlayerServiceError::PayloadValidationError(e.to_string())),
        }
    }

    pub fn set_email<S>(&mut self, email: S) -> Result<&mut Self, PlayerServiceError>
    where
        S: Into<String>,
    {
        match PlayerEmail::try_new(email.into()) {
            Ok(player_email) => {
                self.email = Some(player_email.into_inner());
                return Ok(self);
            }
            Err(e) => return Err(PlayerServiceError::PayloadValidationError(e.to_string())),
        }
    }

    pub fn set_github_username<S>(
        &mut self,
        github_username: S,
    ) -> Result<&mut Self, PlayerServiceError>
    where
        S: Into<String>,
    {
        match PlayerGithubUsername::try_new(github_username.into()) {
            Ok(player_github_username) => {
                self.github_username = Some(player_github_username.into_inner());
                return Ok(self);
            }
            Err(e) => return Err(PlayerServiceError::PayloadValidationError(e.to_string())),
        }
    }

    pub fn set_password<S>(&mut self, password: S) -> Result<&mut Self, PlayerServiceError>
    where
        S: Into<String>,
    {
        match PlayerPassword::try_new(password.into()) {
            Ok(player_password) => {
                self.password = Some(player_password.into_inner());
                return Ok(self);
            }
            Err(e) => return Err(PlayerServiceError::PayloadValidationError(e.to_string())),
        }
    }

    pub fn build(&mut self) -> Player {
        Player {
            first_name: self.first_name.clone().unwrap(),
            last_name: self.last_name.clone().unwrap(),
            email: self.email.clone().unwrap(),
            github_username: self.github_username.clone().unwrap(),
            password: self.password.clone().unwrap(),
        }
    }
}
