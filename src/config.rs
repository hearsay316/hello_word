use config::ConfigError;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server: ServerConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut  cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}