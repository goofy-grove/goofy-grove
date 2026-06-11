use crate::{app::AppDeps, persona::db::persona};

pub async fn is_owner(deps: &AppDeps, persona_uid: &str, user_uid: &str) -> bool {
    persona::load_persona(&deps.db, persona_uid, user_uid)
        .await
        .is_ok()
}
