pub mod handlers;
pub mod types;

use crate::{
    app::AppDeps,
    platform::events::{EventSubscriber, InMemoryEventBus},
};

pub fn subscribe(bus: &InMemoryEventBus, deps: &AppDeps) {
    bus.subscribe(handlers::PersonaCreatedEventHandler::new(
        deps.socket.clone(),
    ));
    bus.subscribe(handlers::PersonaUpdatedEventHandler::new(
        deps.socket.clone(),
    ));
    bus.subscribe(handlers::PersonaDeletedEventHandler::new(
        deps.socket.clone(),
    ));
}
