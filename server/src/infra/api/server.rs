use std::sync::Arc;

use axum::Router;
use gg_core::domain::prelude::EventSubscriber;
use sea_orm::DatabaseConnection;
use socketioxide::{SocketIo, layer::SocketIoLayer};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::infra::{
    api::{
        auth::create_auth_router, characters::create_character_router,
        personas::create_persona_router, users::create_user_router,
    },
    config::Config,
    event_bus::{
        CharacterCreatedEventHandler, CharacterDeletedEventHandler, CharacterUpdatedEventHandler,
        InMemoryEventBus, PersonaCreatedEventHandler, PersonaDeletedEventHandler,
        PersonaUpdatedEventHandler,
    },
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
            "/api/v1/personas",
            create_persona_router(config.clone(), connection.clone(), event_bus.clone()),
        )
        .nest(
            "/api/v1/characters",
            create_character_router(config, connection, event_bus),
        )
        .layer(CorsLayer::very_permissive())
}

// FIXME: move it to another module
pub fn register_event_handlers(event_bus: &mut InMemoryEventBus, socket: SocketIo) {
    event_bus.subscribe(PersonaCreatedEventHandler::new(socket.clone()));
    event_bus.subscribe(PersonaUpdatedEventHandler::new(socket.clone()));
    event_bus.subscribe(PersonaDeletedEventHandler::new(socket.clone()));
    event_bus.subscribe(CharacterCreatedEventHandler::new(socket.clone()));
    event_bus.subscribe(CharacterUpdatedEventHandler::new(socket.clone()));
    event_bus.subscribe(CharacterDeletedEventHandler::new(socket));
}

pub async fn start_server(
    config: Arc<Config>,
    connection: DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let (socketio_layer, io) = create_socketio_layer(connection.clone(), config.clone())?;
    let mut event_bus = InMemoryEventBus::new();

    register_event_handlers(&mut event_bus, io);

    let app = init_router(
        config.clone(),
        connection.clone(),
        socketio_layer,
        event_bus,
    );

    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!(target: "application::server", "Listening on http://{}:{}", config.host, config.port);

    axum::serve(listener, app).await?;

    Ok(())
}
