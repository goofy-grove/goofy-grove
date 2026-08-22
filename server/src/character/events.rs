pub mod handlers;
pub mod types;

use crate::{app::AppDeps, platform::events::EventSubscriber};

pub fn subscribe(deps: &AppDeps) {
    deps.event_bus
        .subscribe(handlers::CharacterCreatedEventHandler::new(
            deps.socket.clone(),
        ));
    deps.event_bus
        .subscribe(handlers::CharacterUpdatedEventHandler::new(
            deps.socket.clone(),
        ));
    deps.event_bus
        .subscribe(handlers::CharacterDeletedEventHandler::new(
            deps.socket.clone(),
        ));
}
