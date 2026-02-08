pub mod entities;
mod repositories;

use domain::prelude::*;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use crate::{
    application::RegistrationService,
    infra::{config::Config, id_generator::UuidGenerator, security::ArgonPasswordSystem},
};

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

    log::info!(target: "application::db", "Creating master user");
    create_master_user(connection.clone()).await;

    connection
}

pub async fn create_master_user(connection: DatabaseConnection) {
    let user_repository = UserRepository::new(connection);

    if let Err(DomainError::ExternalServiceError(LoadUserByNamePortError::NotFound)) =
        user_repository
            .load_user_by_name(&UserName::new("admin".to_owned()))
            .await
    {
        let registration_service =
            RegistrationService::new(user_repository, ArgonPasswordSystem, UuidGenerator);

        registration_service
            .register(RegistrationCommand::new(
                UserName::new("admin".to_owned()),
                Secret::new("password".to_owned()),
            ))
            .await
            .expect("Failed to create master user");

        log::info!(target: "application", "Created user with name: `admin` and password: `password`");
    }
}
