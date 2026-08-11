use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    app::{deps::AppDeps, router::build_router},
    character, chat, persona,
    platform::{config::Config, events::InMemoryEventBus, socketio::create_socketio_layer},
    user,
};

fn register_event_handlers(event_bus: &InMemoryEventBus, deps: &AppDeps) {
    user::subscribe(event_bus, deps);
    character::subscribe(event_bus, deps);
    persona::subscribe(event_bus, deps);
    chat::subscribe(event_bus, deps);
}

pub async fn start_server(
    config: Arc<Config>,
    connection: DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let (socketio_layer, io) = create_socketio_layer(connection.clone(), config.clone())?;
    let event_bus = InMemoryEventBus::new();

    let deps = AppDeps {
        config: config.clone(),
        db: connection.clone(),
        event_bus: event_bus.clone(),
        socket: io,
    };

    user::create_master_user(&deps).await;

    register_event_handlers(&event_bus, &deps);

    let app = build_router(&deps, socketio_layer);

    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!(target: "application::server", "Listening on http://{}:{}", config.host, config.port);

    axum::serve(listener, app).await?;

    Ok(())
}
