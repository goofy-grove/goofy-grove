pub mod handlers;
pub mod types;

use crate::{
    app::AppDeps, platform::events::EventSubscriber,
    user::events::handlers::UserUpdatedEventHandler,
};

pub fn subscribe(deps: &AppDeps) {
    deps.event_bus
        .subscribe(UserUpdatedEventHandler::new(deps.socket.clone()));
}
