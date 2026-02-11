mod db_config;
mod jwt_config;

pub use db_config::DbConfig;

use serde::Deserialize;

use jwt_config::JwtConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub host: String,

    #[serde(default)]
    pub port: u16,

    #[serde(default)]
    pub database: DbConfig,

    #[serde(default)]
    pub jwt: JwtConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 3003,
            database: DbConfig::default(),
            jwt: JwtConfig::default(),
        }
    }
}

impl Config {
    pub fn from_file() -> Self {
        let config_str = std::fs::read_to_string("config.yml")
            .or_else(|_| std::fs::read_to_string("config.yaml"))
            .expect("Failed to read config file");

        serde_yaml::from_str(&config_str).expect("Failed to parse config file")
    }

    pub fn socket_addr(&self) -> (&str, u16) {
        (self.host.as_str(), self.port)
    }
}
