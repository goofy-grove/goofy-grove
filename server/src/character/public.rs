use crate::{app::AppDeps, character::db::character};

pub async fn is_owner(deps: &AppDeps, character_id: &str, user_id: &str) -> bool {
    character::load_character(&deps.db, character_id, user_id)
        .await
        .is_ok()
}
