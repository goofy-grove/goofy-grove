use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub async fn on_connect(socket: SocketRef) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    // socket.emit("auth", &data).ok();

    socket.on(
        "message",
        async |socket: SocketRef, Data::<String>(data)| {
            info!(?data, "Received event:");
            socket.emit("message-back", &data).ok();
        },
    );
}

pub fn create_socketio_layer() -> Result<(SocketIoLayer, SocketIo), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    info!("Socket.IO layer created");

    Ok((layer, io))
}
