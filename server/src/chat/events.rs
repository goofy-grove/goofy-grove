use crate::{
    app::AppDeps,
    chat::events::{
        character_added::CharacterAddedEventHandler,
        character_removed::CharacterRemovedEventHandler, created::ChatCreatedEventHandler,
        deleted::ChatDeletedEventHandler, member_added::MemberAddedEventHandler,
        member_removed::MemberRemovedEventHandler, updated::ChatUpdatedEventHandler,
    },
    platform::events::EventSubscriber,
};

pub mod character_added;
pub mod character_removed;
pub mod created;
pub mod deleted;
pub mod member_added;
pub mod member_removed;
pub mod updated;

pub fn subscribe(deps: &AppDeps) {
    deps.event_bus
        .subscribe(ChatCreatedEventHandler::new(deps.socket.clone()));
    deps.event_bus
        .subscribe(ChatUpdatedEventHandler::new(deps.socket.clone()));
    deps.event_bus
        .subscribe(ChatDeletedEventHandler::new(deps.socket.clone()));
    deps.event_bus
        .subscribe(MemberAddedEventHandler::new(deps.socket.clone()));
    deps.event_bus
        .subscribe(MemberRemovedEventHandler::new(deps.socket.clone()));
    deps.event_bus
        .subscribe(CharacterAddedEventHandler::new(deps.socket.clone()));
    deps.event_bus
        .subscribe(CharacterRemovedEventHandler::new(deps.socket.clone()));
}
