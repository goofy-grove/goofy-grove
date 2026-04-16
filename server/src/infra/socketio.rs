use serde_json::Value;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
};
use tracing::info;

pub async fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!(ns = socket.ns(), ?socket.id, ?data, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    socket.on(
        "message",
        async |socket: SocketRef, Data::<String>(data)| {
            info!(target: "application::socketio", ?data, "Received event:");
            socket.emit("message-back", &data).ok();
        },
    );
}

pub fn create_socketio_layer() -> Result<(SocketIoLayer, SocketIo), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    info!(target: "application::socketio", "Socket.IO layer created");

    Ok((layer, io))
}
