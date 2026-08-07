mod api;
mod db;
mod events;
mod services;

pub use api::mount;
pub use events::subscribe;

use crate::app::AppDeps;

pub async fn is_owner(deps: &AppDeps, persona_uid: &str, user_uid: &str) -> bool {
    db::persona::load_persona(&deps.db, persona_uid, user_uid)
        .await
        .is_ok()
}
