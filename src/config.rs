use config::ConfigError;
use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config
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