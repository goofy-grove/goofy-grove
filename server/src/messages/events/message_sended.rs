use std::pin::Pin;

use socketioxide::SocketIo;
use tracing::info;

use crate::{
    messages::db::Message,
    platform::{
        events::{Event, EventHandler},
        socketio::SOCKET_NAMESPACE,
    },
};

#[derive(Debug, Clone)]
pub struct MessageSendedEvent {
    pub message: Message,
    pub exclude_participants: Vec<String>,
}

impl Event for MessageSendedEvent {}

pub struct MessageSendedEventHandler {
    socket: SocketIo,
}

impl MessageSendedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<MessageSendedEvent> for MessageSendedEventHandler {
    fn handle(&self, event: MessageSendedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let chat_uid = event.message.chat_uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?chat_uid, ?exclude_participants, "Emitting chat:message_added events");

            socket
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("chat:{}", event.message.chat_uid))
                .except(exclude_participants)
                .emit("chat:message_added", &event.message)
                .await
                .ok();
        })
    }
}
