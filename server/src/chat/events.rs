use crate::{
    app::AppDeps,
    chat::events::{deleted::ChatDeletedEventHandler, updated::ChatUpdatedEventHandler},
    platform::events::{EventSubscriber, InMemoryEventBus},
};

pub mod deleted;
pub mod updated;

pub fn subscribe(bus: &InMemoryEventBus, deps: &AppDeps) {
    bus.subscribe(ChatUpdatedEventHandler::new(deps.socket.clone()));
    bus.subscribe(ChatDeletedEventHandler::new(deps.socket.clone()));
}
