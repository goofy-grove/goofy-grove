pub mod handlers;
pub mod types;

use crate::{app::AppDeps, platform::events::EventSubscriber};

pub fn subscribe(deps: &AppDeps) {
    deps.event_bus
        .subscribe(handlers::PersonaCreatedEventHandler::new(
            deps.socket.clone(),
        ));
    deps.event_bus
        .subscribe(handlers::PersonaUpdatedEventHandler::new(
            deps.socket.clone(),
        ));
    deps.event_bus
        .subscribe(handlers::PersonaDeletedEventHandler::new(
            deps.socket.clone(),
        ));
}
