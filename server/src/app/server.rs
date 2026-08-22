use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    app::{deps::AppDeps, router::build_router},
    character, chat, messages, persona,
    platform::{config::Config, events::InMemoryEventBus, socketio::create_socketio_layer},
    user,
};

fn register_event_handlers(deps: &AppDeps) {
    user::subscribe(deps);
    character::subscribe(deps);
    persona::subscribe(deps);
    chat::subscribe(deps);
    messages::subscribe(deps);
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
        event_bus,
        socket: io,
    };

    user::create_master_user(&deps).await;

    register_event_handlers(&deps);

    let app = build_router(&deps, socketio_layer);

    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!(target: "application::server", "Listening on http://{}:{}", config.host, config.port);

    axum::serve(listener, app).await?;

    Ok(())
}
