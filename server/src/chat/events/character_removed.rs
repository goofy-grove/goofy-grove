use socketioxide::SocketIo;
use tracing::info;

use crate::{
    chat::db::ChatCharacter,
    platform::{
        events::{Event, EventHandler},
        socketio::SOCKET_NAMESPACE,
    },
};

#[derive(Debug, Clone)]
pub struct CharacterRemovedEvent {
    pub character: ChatCharacter,
    pub exclude_participants: Vec<String>,
}

impl Event for CharacterRemovedEvent {}

pub struct CharacterRemovedEventHandler {
    socket: SocketIo,
}

impl CharacterRemovedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<CharacterRemovedEvent> for CharacterRemovedEventHandler {
    fn handle(
        &self,
        event: CharacterRemovedEvent,
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let chat_uid = event.character.chat_uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?chat_uid, ?exclude_participants, "Emitting chat:character_removed events");

            socket
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("chat:{}", event.character.chat_uid))
                .except(exclude_participants)
                .emit("chat:character_removed", &event.character)
                .await
                .ok();
        })
    }
}
