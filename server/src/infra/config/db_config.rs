use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum DbConfig {
    #[serde(rename = "postgres")]
    Postgres {
        host: String,
        port: u16,
        username: String,
        password: String,
    },
    #[serde(rename = "sqlite")]
    Sqlite {
        file_path: String,
    },
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig::Postgres {
            host: "localhost".to_string(),
            port: 5432,
            username: "user".to_string(),
            password: "password".to_string(),
        }
    }
}