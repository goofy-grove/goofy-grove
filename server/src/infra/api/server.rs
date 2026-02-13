use std::sync::Arc;

use axum::Router;
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

use crate::infra::{
    api::{auth::create_auth_router, users::create_user_router},
    config::Config,
};

pub fn init_router(config: Arc<Config>, connection: DatabaseConnection) -> Router {
    Router::new()
        .nest(
            "/api/v1/auth",
            create_auth_router(config.clone(), connection.clone()),
        )
        .nest("/api/v1/users", create_user_router(config, connection))
}

pub async fn start_server(config: Arc<Config>, connection: DatabaseConnection) {
    let app = init_router(config.clone(), connection.clone());

    let listener = TcpListener::bind(config.socket_addr()).await.unwrap();

    log::info!(target: "application", "Listening on {}:{}", config.host, config.port);

    axum::serve(listener, app).await.unwrap();
}
