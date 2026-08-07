pub mod handlers;
pub mod types;

use crate::{
    app::AppDeps,
    platform::events::{EventSubscriber, InMemoryEventBus},
};

// FIXME: Remove mod.rs file and move subscribe function into public.rs file
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
