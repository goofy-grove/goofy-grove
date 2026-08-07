mod api;
mod db;
mod events;
mod services;

pub use api::mount;
pub use db::character::Character;
pub use events::subscribe;

use crate::app::AppDeps;

pub async fn is_owner(deps: &AppDeps, character_uid: &str, user_uid: &str) -> bool {
    db::character::load_character(&deps.db, character_uid, user_uid)
        .await
        .is_ok()
}
