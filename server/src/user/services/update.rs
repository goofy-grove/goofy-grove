use crate::{
    app::AppDeps,
    file::public::{ApplyAvatarPatchError, FileScope, apply_avatar_uid_patch},
    platform::{events::EventPublisher, types::PatchField},
    user::{
        db::user::{User, load_user_by_id, save_user},
        events::types::UserUpdatedEvent,
    },
};

#[derive(Debug, Clone, thiserror::Error)]
pub enum UpdateUserError {
    #[error("Not found")]
    NotFound,

    #[error("File not found")]
    FileNotFound,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub async fn update_user(
    deps: &AppDeps,
    user_id: &str,
    avatar_uid: PatchField<String>,
    exclude_participants: Vec<String>,
) -> Result<User, UpdateUserError> {
    let user = load_user_by_id(&deps.db, user_id)
        .await
        .map_err(|_| UpdateUserError::NotFound)?;

    let next_avatar_uid = apply_avatar_uid_patch(
        deps,
        user.avatar_uid.clone(),
        avatar_uid,
        &FileScope::UserAvatar {
            user_id: user.uid.clone(),
        },
    )
    .await
    .map_err(|err| match err {
        ApplyAvatarPatchError::FileNotFound => UpdateUserError::FileNotFound,
        ApplyAvatarPatchError::InvalidFileStatus => {
            UpdateUserError::ValidationError("Invalid file status".into())
        }
        ApplyAvatarPatchError::InvalidFileScope => {
            UpdateUserError::ValidationError("Invalid file scope".into())
        }
        ApplyAvatarPatchError::InternalError(message) => UpdateUserError::InternalError(message),
    })?;

    let updated = User {
        uid: user.uid,
        name: user.name,
        password: user.password,
        avatar_uid: next_avatar_uid,
    };

    let saved = save_user(&deps.db, updated)
        .await
        .map_err(|err| UpdateUserError::InternalError(err.to_string()))?;

    deps.event_bus
        .publish(UserUpdatedEvent {
            user: saved.clone(),
            exclude_participants,
        })
        .await;

    Ok(saved)
}
