use std::pin::Pin;

use serde_json::json;
use socketioxide::SocketIo;
use tracing::info;

use crate::{platform::events::EventHandler, user::events::types::UserUpdatedEvent};

pub struct UserUpdatedEventHandler {
    socket: SocketIo,
}

impl UserUpdatedEventHandler {
    pub fn new(socket: SocketIo) -> Self {
        Self { socket }
    }
}

impl EventHandler<UserUpdatedEvent> for UserUpdatedEventHandler {
    fn handle(&self, event: &UserUpdatedEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let user_id = event.user.uid.clone();
        let json = json!({
            "id": user_id,
            "username": event.user.name,
            "avatar_uid": event.user.avatar_uid,
        });
        let socket = self.socket.clone();
        let exclude_participants = event.exclude_participants.clone();

        Box::pin(async move {
            info!(target: "application::event_bus", ?user_id, ?exclude_participants, "Emitting user:updated event");

            socket
                .of("/v1")
                .unwrap()
                .within(format!("user:{user_id}"))
                .except(exclude_participants)
                .emit("user:updated", &json)
                .await
                .ok();
        })
    }
}
