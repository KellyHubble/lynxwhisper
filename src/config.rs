use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub mode: String,
    pub model: ModelConfig,
    pub hotkeys: Hotkeys,
    pub automatic: AutomaticConfig,
}

#[derive(Deserialize, Clone)]
pub struct ModelConfig {
    pub path: PathBuf,
}

#[derive(Deserialize, Clone)]
pub struct Hotkeys {
    pub manual_start: String,
    pub manual_stop: String,
    pub automatic_toggle: String,
}

#[derive(Deserialize, Clone)]
pub struct AutomaticConfig {
    pub chunk_interval: u64,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}