use crate::{app::AppDeps, character::db::character};

pub use crate::character::db::character::Character;

pub async fn is_owner(deps: &AppDeps, character_uid: &str, user_uid: &str) -> bool {
    character::load_character(&deps.db, character_uid, user_uid)
        .await
        .is_ok()
}
