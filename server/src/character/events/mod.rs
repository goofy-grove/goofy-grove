pub mod handlers;
pub mod types;

use crate::{
    app::AppDeps,
    platform::events::{EventSubscriber, InMemoryEventBus},
};

pub fn subscribe(bus: &InMemoryEventBus, deps: &AppDeps) {
    bus.subscribe(handlers::CharacterCreatedEventHandler::new(
        deps.socket.clone(),
    ));
    bus.subscribe(handlers::CharacterUpdatedEventHandler::new(
        deps.socket.clone(),
    ));
    bus.subscribe(handlers::CharacterDeletedEventHandler::new(
        deps.socket.clone(),
    ));
}
