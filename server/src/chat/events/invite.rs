use std::pin::Pin;

use socketioxide::SocketIo;
use tracing::info;

use crate::{
    chat::db::Chat,
    platform::events::{Event, EventHandler},
    user::User,
};

#[derive(Debug, Clone)]
pub struct InviteToChatEvent {
    pub chat: Chat,
    pub user: User,
    pub exclude_participants: Vec<String>,
}

impl Event for InviteToChatEvent {}

pub struct InviteToChatEventHandler {
    socket: SocketIo,
}

impl InviteToChatEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<InviteToChatEvent> for InviteToChatEventHandler {
    fn handle(&self, event: InviteToChatEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let chat_uid = event.chat.uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?chat_uid, ?exclude_participants, "Emitting chat:user_invited event");

            socket
                .clone()
                .of("/v1")
                .unwrap()
                .within(format!("user:{}", event.user.uid))
                .join(format!("chat:{chat_uid}"))
                .await
                .unwrap();

            socket
                .of("/v1")
                .unwrap()
                .within(format!("chat:{chat_uid}"))
                .except(exclude_participants)
                .emit("chat:user_invited", &event.chat)
                .await
                .ok();
        })
    }
}
