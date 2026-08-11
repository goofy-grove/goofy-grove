use crate::{
    app::AppDeps,
    chat::events::chat_update_event::ChatUpdatedEventHandler,
    platform::events::{EventSubscriber, InMemoryEventBus},
};

pub mod chat_update_event;

pub fn subscribe(bus: &InMemoryEventBus, deps: &AppDeps) {
    bus.subscribe(ChatUpdatedEventHandler::new(deps.socket.clone()));
}
