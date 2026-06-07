use crate::{app::AppDeps, persona::db::persona};

pub async fn is_owner(deps: &AppDeps, persona_id: &str, user_id: &str) -> bool {
    persona::load_persona(&deps.db, persona_id, user_id)
        .await
        .is_ok()
}
