mod api;
mod db;
mod events;
mod services;

pub use api::mount;
pub use db::character::{Character, LoadCharacterError, load_character as get_by_uid};
pub use events::subscribe;

use crate::app::AppDeps;

pub async fn is_owner(deps: &AppDeps, character_uid: &str, user_uid: &str) -> bool {
    db::character::load_character(&deps.db, character_uid)
        .await
        .is_ok_and(|character| character.creator_uid == user_uid)
}

pub async fn is_visible_to_user(deps: &AppDeps, character_uid: &str, user_uid: &str) -> bool {
    db::character::is_visible_to_user(&deps.db, character_uid, user_uid)
        .await
        .unwrap_or(false)
}
