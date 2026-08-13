use std::pin::Pin;

use serde_json::json;
use socketioxide::SocketIo;
use tracing::info;

use crate::{
    chat::db::ChatMember,
    platform::{
        events::{Event, EventHandler},
        socketio::SOCKET_NAMESPACE,
    },
};

#[derive(Debug, Clone)]
pub struct MemberRemovedEvent {
    pub member: ChatMember,
    pub exclude_participants: Vec<String>,
}

impl Event for MemberRemovedEvent {}

pub struct MemberRemovedEventHandler {
    socket: SocketIo,
}

impl MemberRemovedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<MemberRemovedEvent> for MemberRemovedEventHandler {
    fn handle(&self, event: MemberRemovedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let exclude_participants = event.exclude_participants;
            let chat_uid = event.member.chat_uid.clone();

            info!(target: "application::event_bus", ?chat_uid, ?exclude_participants, "Emitting chat:member_removed, chat:removed events");

            socket
                .clone()
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("user:{}", event.member.user.uid))
                .leave(format!("chat:{}", event.member.chat_uid))
                .await
                .ok();

            socket
                .clone()
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("chat:{}", event.member.chat_uid))
                .except(exclude_participants.clone())
                .emit("chat:member_removed", &event.member)
                .await
                .ok();

            socket
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("user:{}", event.member.user.uid))
                .except(exclude_participants)
                .emit("chat:removed", &json!({ "uid": event.member.chat_uid }))
                .await
                .ok();
        })
    }
}
