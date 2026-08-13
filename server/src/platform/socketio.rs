mod middlewares;

use std::sync::Arc;

use keyv::Keyv;
use sea_orm::DatabaseConnection;
use socketioxide::{
    SocketIo, SocketIoBuilder,
    extract::{SocketRef, State},
    handler::ConnectHandler,
    layer::SocketIoLayer,
};
use tracing::info;

use crate::platform::config::Config;

pub const SOCKET_NAMESPACE: &str = "/v1";

pub async fn on_connect(socket: SocketRef) {
    info!(target: "application::socketio", id = ?socket.id, "Socket.IO connected");

    socket.on_disconnect(async |socket: SocketRef, State(keyv): State<Arc<Keyv>>| {
        info!(target: "application::socketio", id = ?socket.id, "Socket.IO disconnected");

        let result = keyv.remove(socket.id.as_str()).await;

        if result.is_err() {
            info!(target: "application::socketio", err = ?result.err(), "Keyv error:");
        }
    });
}

pub fn create_socketio_layer(
    db_connection: DatabaseConnection,
    config: Arc<Config>,
) -> Result<(SocketIoLayer, SocketIo), Box<dyn std::error::Error>> {
    let keyv = Arc::new(Keyv::default());

    let (layer, io) = SocketIoBuilder::new()
        .with_state(db_connection)
        .with_state(config)
        .with_state(keyv)
        .build_layer();

    io.ns(
        SOCKET_NAMESPACE,
        on_connect.with(middlewares::authentication_middleware),
    );

    info!(target: "application::socketio", "Socket.IO layer created");

    Ok((layer, io))
}
