use crate::{
    app::AppDeps,
    file::{
        FileScope, OrphanAvatarError, ReplaceAvatarError, ReplaceAvatarInput,
        orphan_avatar_if_present, replace_avatar,
    },
    platform::events::EventPublisher,
    user::{
        db::user::{User, load_user_by_uid, save_user},
        events::types::UserUpdatedEvent,
    },
};

#[derive(Debug, Clone, thiserror::Error)]
pub enum SetUserAvatarError {
    #[error("Not found")]
    NotFound,

    #[error(transparent)]
    Replace(#[from] ReplaceAvatarError),

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ClearUserAvatarError {
    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct SetUserAvatarInput {
    pub content_type: String,
    pub original_name: String,
    pub content: Vec<u8>,
    pub exclude_participants: Vec<String>,
}

pub async fn set_user_avatar(
    deps: &AppDeps,
    user_uid: &str,
    input: SetUserAvatarInput,
) -> Result<User, SetUserAvatarError> {
    let SetUserAvatarInput {
        content_type,
        original_name,
        content,
        exclude_participants,
    } = input;

    let (user, credentials) = load_user_by_uid(&deps.db, user_uid)
        .await
        .map_err(|_| SetUserAvatarError::NotFound)?;

    let new_avatar_uid = replace_avatar(
        deps,
        ReplaceAvatarInput {
            content_type,
            original_name,
            scope: FileScope::UserAvatar {
                user_uid: user.uid.clone(),
            },
            content,
            current_avatar_uid: user.avatar_uid.clone(),
        },
        user_uid,
    )
    .await?;

    let updated = User {
        uid: user.uid,
        name: user.name,
        avatar_uid: Some(new_avatar_uid),
    };

    let saved = save_user(&deps.db, updated, credentials)
        .await
        .map_err(|err| SetUserAvatarError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(UserUpdatedEvent {
            user: saved.clone(),
            exclude_participants,
        })
        .await;

    Ok(saved)
}

pub async fn clear_user_avatar(
    deps: &AppDeps,
    user_uid: &str,
    exclude_participants: Vec<String>,
) -> Result<User, ClearUserAvatarError> {
    let (user, credentials) = load_user_by_uid(&deps.db, user_uid)
        .await
        .map_err(|_| ClearUserAvatarError::NotFound)?;

    if let Err(OrphanAvatarError::InternalError(message)) =
        orphan_avatar_if_present(deps, user.avatar_uid.clone()).await
    {
        return Err(ClearUserAvatarError::InternalError(message));
    }

    let updated = User {
        uid: user.uid,
        name: user.name,
        avatar_uid: None,
    };

    let saved = save_user(&deps.db, updated, credentials)
        .await
        .map_err(|err| ClearUserAvatarError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(UserUpdatedEvent {
            user: saved.clone(),
            exclude_participants,
        })
        .await;

    Ok(saved)
}
