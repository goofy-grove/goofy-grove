use std::sync::Arc;

use axum::Router;
use sea_orm::DatabaseConnection;
use socketioxide::layer::SocketIoLayer;
use tokio::net::TcpListener;

use crate::infra::{
    api::{auth::create_auth_router, persons::create_person_router, users::create_user_router},
    config::Config,
    socketio::create_socketio_layer,
};

pub fn init_router(
    config: Arc<Config>,
    connection: DatabaseConnection,
    socketio_layer: SocketIoLayer,
) -> Router {
    Router::new()
        .layer(socketio_layer)
        .nest(
            "/api/v1/auth",
            create_auth_router(config.clone(), connection.clone()),
        )
        .nest(
            "/api/v1/users",
            create_user_router(config.clone(), connection.clone()),
        )
        .nest("/api/v1/persons", create_person_router(config, connection))
}

pub async fn start_server(
    config: Arc<Config>,
    connection: DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let socketio_layer = create_socketio_layer()?;
    let app = init_router(config.clone(), connection.clone(), socketio_layer);

    let listener = TcpListener::bind(config.socket_addr()).await?;

    log::info!(target: "application", "Listening on {}:{}", config.host, config.port);

    axum::serve(listener, app).await?;

    Ok(())
}
