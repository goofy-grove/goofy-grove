pub use crate::user::db::user::{User, load_user_by_name as get_by_name_db};
pub use crate::user::services::get::get_by_name;

use tracing::info;

use crate::{app::AppDeps, user::services::register};

pub async fn create_master_user(deps: &AppDeps) {
    if get_by_name(deps, "admin").await.is_err() {
        register::register(deps, "admin", "password")
            .await
            .expect("Failed to create master user");

        info!(target: "application", "Created user with name: `admin` and password: `password`");
    }
}
