use crate::{
    app::AppDeps,
    chat::events::{
        character_added::CharacterAddedEventHandler,
        character_removed::CharacterRemovedEventHandler, created::ChatCreatedEventHandler,
        deleted::ChatDeletedEventHandler, member_added::MemberAddedEventHandler,
        member_removed::MemberRemovedEventHandler, updated::ChatUpdatedEventHandler,
    },
    platform::events::{EventSubscriber, InMemoryEventBus},
};

pub mod character_added;
pub mod character_removed;
pub mod created;
pub mod deleted;
pub mod member_added;
pub mod member_removed;
pub mod updated;

pub fn subscribe(bus: &InMemoryEventBus, deps: &AppDeps) {
    bus.subscribe(ChatCreatedEventHandler::new(deps.socket.clone()));
    bus.subscribe(ChatUpdatedEventHandler::new(deps.socket.clone()));
    bus.subscribe(ChatDeletedEventHandler::new(deps.socket.clone()));
    bus.subscribe(MemberAddedEventHandler::new(deps.socket.clone()));
    bus.subscribe(MemberRemovedEventHandler::new(deps.socket.clone()));
    bus.subscribe(CharacterAddedEventHandler::new(deps.socket.clone()));
    bus.subscribe(CharacterRemovedEventHandler::new(deps.socket.clone()));
}
