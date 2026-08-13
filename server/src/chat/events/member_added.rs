use std::pin::Pin;

use socketioxide::SocketIo;
use tracing::info;

use crate::{
    chat::db::{Chat, ChatMember},
    platform::{
        events::{Event, EventHandler},
        socketio::SOCKET_NAMESPACE,
    },
};

#[derive(Debug, Clone)]
pub struct MemberAddedEvent {
    pub chat: Chat,
    pub member: ChatMember,
    pub exclude_participants: Vec<String>,
}

impl Event for MemberAddedEvent {}

pub struct MemberAddedEventHandler {
    socket: SocketIo,
}

impl MemberAddedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<MemberAddedEvent> for MemberAddedEventHandler {
    fn handle(&self, event: MemberAddedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let chat_uid = event.chat.uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?chat_uid, ?exclude_participants, "Emitting chat:member_added, chat:added events");

            socket
                .clone()
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("chat:{chat_uid}"))
                .except(exclude_participants)
                .emit("chat:member_added", &event.member)
                .await
                .ok();

            socket
                .clone()
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("user:{}", event.member.user.uid))
                .join(format!("chat:{chat_uid}"))
                .await
                .unwrap();

            socket
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("user:{}", event.member.user.uid))
                .emit("chat:added", &event.chat)
                .await
                .unwrap();
        })
    }
}
