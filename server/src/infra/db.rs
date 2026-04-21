pub mod entities;
mod repositories;

use gg_core::{application::auth::RegistrationService, domain::prelude::*};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

use crate::infra::{config::Config, id_generator::UuidGenerator, security::ArgonPasswordSystem};

pub use repositories::*;

pub async fn init_db(config: &Config) -> DatabaseConnection {
    info!(target: "application::db", "Initializing database connection...");

    let connection = Database::connect(config.database.to_connection_string())
        .await
        .expect("Failed to connect database");

    info!(target: "application::db", "Running database migrations");
    Migrator::up(&connection, None)
        .await
        .expect("Failed to run database migrations");

    info!(target: "application::db", "Creating master user");
    create_master_user(connection.clone()).await;

    connection
}

pub async fn create_master_user(connection: DatabaseConnection) {
    let user_repository = UserRepository::new(connection);
    // NOTE: static admin username is controlled in code and satisfies Username validation.
    let admin_username = Username::try_new("admin".to_owned()).unwrap();
    // NOTE: static admin password is controlled in code and satisfies Secret validation.
    let admin_password = Secret::try_new("password".to_owned()).unwrap();

    if let Err(LoadUserByNamePortError::NotFound) =
        user_repository.load_user_by_name(&admin_username).await
    {
        let registration_service =
            RegistrationService::new(user_repository, ArgonPasswordSystem, UuidGenerator);

        registration_service
            .register(RegistrationCommand::new(admin_username, admin_password))
            .await
            .expect("Failed to create master user");

        info!(target: "application", "Created user with name: `admin` and password: `password`");
    }
}
