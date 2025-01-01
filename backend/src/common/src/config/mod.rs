pub mod r#type;
use std::{fs::File, io::Read};

pub fn load_config() -> Result<r#type::Config, Box<dyn std::error::Error>> {
    let mut config_file = File::open("./Config.toml")?;
    let mut buff = String::new();
    config_file.read_to_string(&mut buff)?;
    let config_data: r#type::Config = toml::from_str(&buff)?;

    Ok(config_data)
}
