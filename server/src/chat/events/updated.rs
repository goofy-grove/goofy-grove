use std::pin::Pin;

use socketioxide::SocketIo;
use tracing::info;

use crate::{
    chat::db::ChatInfo,
    platform::{
        events::{Event, EventHandler},
        socketio::SOCKET_NAMESPACE,
    },
};

#[derive(Debug, Clone)]
pub struct ChatUpdatedEvent {
    pub chat: ChatInfo,
    pub exclude_participants: Vec<String>,
}

impl Event for ChatUpdatedEvent {}

pub struct ChatUpdatedEventHandler {
    socket: SocketIo,
}

impl ChatUpdatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<ChatUpdatedEvent> for ChatUpdatedEventHandler {
    fn handle(&self, event: ChatUpdatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let chat_uid = event.chat.uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?chat_uid, ?exclude_participants, "Emitting chat:updated event");

            socket
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("chat:{}", event.chat.uid))
                .except(exclude_participants)
                .emit("chat:updated", &event.chat)
                .await
                .ok();
        })
    }
}
