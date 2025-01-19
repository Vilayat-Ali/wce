use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub build: BuildConfig,
    pub player: PlayerServiceConfig,
    pub game: GameServiceConfig,
    pub editor: EditorServiceConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuildConfig {
    pub singleton: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerServiceConfig {
    pub port: usize,
    pub trace_level: String,
    pub db_url: String,
    pub access_token_secret: String,
    pub refresh_token_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameServiceConfig {
    pub port: usize,
    pub trace_level: String,
    pub db_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditorServiceConfig {
    pub port: usize,
    pub trace_level: String,
    pub remote_execution_container: u8,
    pub db_url: Option<String>,
}
