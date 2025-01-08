pub mod r#type;
use std::{fs::File, io::Read};

pub fn load_config() -> Result<r#type::Config, Box<dyn std::error::Error>> {
    let mut config_file = File::open("./Config.toml")?;
    let mut buff = String::new();
    config_file.read_to_string(&mut buff)?;
    let config_data: r#type::Config = toml::from_str(&buff)?;
    tracing::info!("Config.toml has been parsed successfully!");

    std::env::set_var("RUST_LOG", config_data.player.trace_level.clone());
    tracing::info!(
        "Trace `{}` has been set in environment variable",
        config_data.player.trace_level.clone()
    );

    Ok(config_data)
}
