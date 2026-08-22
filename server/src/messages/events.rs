use crate::{
    app::AppDeps, messages::events::message_sended::MessageSendedEventHandler,
    platform::events::EventSubscriber,
};

pub mod message_sended;

pub fn subscribe(deps: &AppDeps) {
    deps.event_bus
        .subscribe(MessageSendedEventHandler::new(deps.socket.clone()))
}
