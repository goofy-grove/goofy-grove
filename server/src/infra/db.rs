pub mod entities;
mod repositories;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use crate::infra::config::Config;

pub use repositories::*;

pub async fn init_db(config: &Config) -> DatabaseConnection {
    log::info!(target: "application::db", "Initializing database connection...");

    let connection = Database::connect(config.database.to_connection_string())
        .await
        .expect("Failed to connect database");

    log::info!(target: "application::db", "Running database migrations");
    Migrator::up(&connection, None)
        .await
        .expect("Failed to run database migrations");

    connection
}
