pub mod handlers;
pub mod types;

use crate::{
    app::AppDeps,
    platform::events::{EventSubscriber, InMemoryEventBus},
    user::events::handlers::UserUpdatedEventHandler,
};

pub fn subscribe(bus: &InMemoryEventBus, deps: &AppDeps) {
    bus.subscribe(UserUpdatedEventHandler::new(deps.socket.clone()));
}
