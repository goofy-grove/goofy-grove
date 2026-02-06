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
        database_name: String,
    },
    #[serde(rename = "sqlite")]
    Sqlite { file_path: String },
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig::Postgres {
            host: "localhost".to_string(),
            port: 5432,
            username: "user".to_string(),
            password: "password".to_string(),
            database_name: "goofy_grove".to_string(),
        }
    }
}

impl DbConfig {
    pub fn to_connection_string(&self) -> String {
        match self {
            DbConfig::Postgres {
                host,
                port,
                username,
                password,
                database_name,
            } => format!(
                "postgres://{}:{}@{}:{}/{}",
                username, password, host, port, database_name
            ),
            DbConfig::Sqlite { file_path } => {
                if file_path == ":memory:" {
                    "sqlite::memory:".to_string()
                } else {
                    format!("sqlite://{}", file_path)
                }
            }
        }
    }
}
