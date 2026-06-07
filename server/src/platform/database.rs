pub mod entities;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

use crate::platform::config::Config;

pub async fn init_db(config: &Config) -> DatabaseConnection {
    info!(target: "application::db", "Initializing database connection...");

    let connection = Database::connect(config.database.to_connection_string())
        .await
        .expect("Failed to connect database");

    info!(target: "application::db", "Running database migrations");

    Migrator::up(&connection, None)
        .await
        .expect("Failed to run database migrations");

    connection
}
