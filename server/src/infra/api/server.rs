use std::sync::Arc;

use axum::Router;
use gg_core::domain::prelude::EventSubscriber;
use sea_orm::DatabaseConnection;
use socketioxide::{SocketIo, layer::SocketIoLayer};
use tokio::net::TcpListener;

use crate::infra::{
    api::{auth::create_auth_router, persons::create_person_router, users::create_user_router},
    config::Config,
    event_bus::{InMemoryEventBus, PersonCreatedEventHandler},
    socketio::create_socketio_layer,
};

pub fn init_router(
    config: Arc<Config>,
    connection: DatabaseConnection,
    socketio_layer: SocketIoLayer,
    event_bus: InMemoryEventBus,
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
        .nest(
            "/api/v1/persons",
            create_person_router(config, connection, event_bus),
        )
}

pub fn register_event_handlers(event_bus: &mut InMemoryEventBus, socket: SocketIo) {
    event_bus.subscribe(PersonCreatedEventHandler::new(socket));
}

pub async fn start_server(
    config: Arc<Config>,
    connection: DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let (socketio_layer, io) = create_socketio_layer()?;
    let mut event_bus = InMemoryEventBus::new();

    register_event_handlers(&mut event_bus, io);

    let app = init_router(
        config.clone(),
        connection.clone(),
        socketio_layer,
        event_bus.clone(),
    );

    let listener = TcpListener::bind(config.socket_addr()).await?;

    log::info!(target: "application", "Listening on {}:{}", config.host, config.port);

    axum::serve(listener, app).await?;

    Ok(())
}
