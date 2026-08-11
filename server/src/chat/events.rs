use crate::{
    app::AppDeps,
    chat::events::{
        created::ChatCreatedEventHandler, deleted::ChatDeletedEventHandler,
        updated::ChatUpdatedEventHandler,
    },
    platform::events::{EventSubscriber, InMemoryEventBus},
};

pub mod created;
pub mod deleted;
pub mod updated;

pub fn subscribe(bus: &InMemoryEventBus, deps: &AppDeps) {
    bus.subscribe(ChatCreatedEventHandler::new(deps.socket.clone()));
    bus.subscribe(ChatUpdatedEventHandler::new(deps.socket.clone()));
    bus.subscribe(ChatDeletedEventHandler::new(deps.socket.clone()));
}
