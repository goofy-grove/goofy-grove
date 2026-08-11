mod api;
mod db;
mod events;
mod services;

pub use api::mount;
pub use db::user::{User, load_user_by_name as get_by_name_db, load_user_by_uid as get_by_uid};
pub use events::subscribe;
pub use services::get::get_by_name;

use tracing::info;

use crate::app::AppDeps;

pub async fn create_master_user(deps: &AppDeps) {
    if get_by_name(deps, "admin").await.is_err() {
        services::register::register(deps, "admin", "password")
            .await
            .expect("Failed to create master user");

        info!(target: "application", "Created user with name: `admin` and password: `password`");
    }
}
