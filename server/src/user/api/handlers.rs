use axum::{
    Extension,
    extract::{Multipart, State},
    response::Response,
    routing::{delete, get, put},
};

use crate::{
    app::AppDeps,
    auth::AuthenticatedUser,
    platform::http::{
        error::{ApiError, codes},
        extract::{ExcludeSocketParticipants, read_multipart_file},
        response,
    },
    user::services::avatar::{
        ClearUserAvatarError, SetUserAvatarError, SetUserAvatarInput, clear_user_avatar,
        set_user_avatar,
    },
};

async fn get_current_user(Extension(user): Extension<AuthenticatedUser>) -> Response {
    response::ok(user)
}

async fn put_user_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    multipart: Multipart,
) -> Result<Response, ApiError> {
    let max_file_bytes = deps
        .config
        .policies
        .files
        .user_avatar
        .max_file_size
        .to_bytes() as usize;

    let (original_name, content_type, content) =
        read_multipart_file(multipart, max_file_bytes).await?;

    let updated_user = set_user_avatar(
        &deps,
        &user.uid,
        SetUserAvatarInput {
            content_type,
            original_name,
            content,
            exclude_participants: exclude_participant.into_iter().collect(),
        },
    )
    .await
    .map_err(|err| match err {
        SetUserAvatarError::NotFound => ApiError::not_found(codes::USER_NOT_FOUND),
        SetUserAvatarError::Replace(replace_err) => ApiError::from(replace_err),
        SetUserAvatarError::InternalError(_) => ApiError::internal(codes::USER_UPDATE_FAILED),
    })?;

    Ok(response::ok(updated_user))
}

async fn delete_user_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    let updated_user = clear_user_avatar(
        &deps,
        &user.uid,
        exclude_participant.into_iter().collect(),
    )
    .await
    .map_err(|err| match err {
        ClearUserAvatarError::NotFound => ApiError::not_found(codes::USER_NOT_FOUND),
        ClearUserAvatarError::InternalError(_) => ApiError::internal(codes::USER_UPDATE_FAILED),
    })?;

    Ok(response::ok(updated_user))
}

pub fn routes() -> axum::Router<AppDeps> {
    axum::Router::new()
        .route("/me", get(get_current_user))
        .route("/me/avatar", put(put_user_avatar))
        .route("/me/avatar", delete(delete_user_avatar))
}
