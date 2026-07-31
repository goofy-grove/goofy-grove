use crate::{
    app::AppDeps,
    user::db::user::{LoadUserError, User, load_user_by_name},
};

pub async fn get_by_name(deps: &AppDeps, name: &str) -> Result<User, LoadUserError> {
    load_user_by_name(&deps.db, name)
        .await
        .map(|(user, _)| user)
}
