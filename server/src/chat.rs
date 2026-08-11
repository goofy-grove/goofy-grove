use crate::app::AppDeps;

mod db;
mod events;
mod services;

pub use events::subscribe;

pub async fn is_owner(deps: &AppDeps, chat_uid: &str, user_uid: &str) -> bool {
    let chat = db::chat::load_chat(&deps.db, chat_uid).await;

    chat.map(|chat| chat.creator_uid == user_uid)
        .unwrap_or(false)
}

pub async fn is_member(deps: &AppDeps, chat_uid: &str, user_uid: &str) -> bool {
    let chat = db::chat::load_chat(&deps.db, chat_uid).await;

    chat.map(|chat| {
        chat.members
            .iter()
            .any(|member| member.user.uid == user_uid)
    })
    .unwrap_or(false)
}
