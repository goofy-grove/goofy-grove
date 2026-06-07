use crate::{platform::events::Event, user::db::user::User};

#[derive(Debug, Clone)]
pub struct UserUpdatedEvent {
    pub user: User,
    pub exclude_participants: Vec<String>,
}

impl Event for UserUpdatedEvent {}
