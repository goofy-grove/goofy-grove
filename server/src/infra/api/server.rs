use std::sync::Arc;

use axum::Router;
use domain::prelude::{
    DomainError, LoadUserByNamePort, LoadUserByNamePortError, RegistrationCommand,
    RegistrationUseCase, Secret, UserIdGenerator, UserName,
};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

use crate::{
    application::{RegistrationService, UserAuthorizationService},
    infra::{
        api::auth::create_auth_router, config::Config, db::UserRepository,
        id_generator::UuidGenerator, security::ArgonPasswordSystem,
    },
};

#[derive(Debug, Clone)]
struct AppState {
    config: Arc<Config>,
}

pub fn init_router(config: Arc<Config>, connection: DatabaseConnection) -> Router {
    Router::new()
        .with_state(AppState { config })
        .nest("/api/v1/auth", create_auth_router(connection))
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

pub async fn start_server(config: Arc<Config>, connection: DatabaseConnection) {
    let app = init_router(config.clone(), connection.clone());

    log::info!(target: "application", "Creating master user");
    create_master_user(connection).await;

    let listener = TcpListener::bind(config.socket_addr()).await.unwrap();

    log::info!(target: "application", "Listening on {}:{}", config.host, config.port);

    axum::serve(listener, app).await.unwrap();
}
