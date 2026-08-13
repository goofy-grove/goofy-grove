use std::pin::Pin;

use socketioxide::SocketIo;
use tracing::info;

use crate::{
    platform::{events::EventHandler, socketio::SOCKET_NAMESPACE},
    user::events::types::UserUpdatedEvent,
};

pub struct UserUpdatedEventHandler {
    socket: SocketIo,
}

impl UserUpdatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<UserUpdatedEvent> for UserUpdatedEventHandler {
    fn handle(&self, event: UserUpdatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let socket = self.socket.clone();

        Box::pin(async move {
            let user_uid = event.user.uid.clone();
            let exclude_participants = event.exclude_participants;

            info!(target: "application::event_bus", ?user_uid, ?exclude_participants, "Emitting user:updated event");

            socket
                .of(SOCKET_NAMESPACE)
                .unwrap()
                .within(format!("user:{user_uid}"))
                .except(exclude_participants)
                .emit("user:updated", &event.user)
                .await
                .ok();
        })
    }
}
