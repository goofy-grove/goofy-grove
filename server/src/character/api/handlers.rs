use axum::{
    Extension,
    extract::{Multipart, Path, State},
    response::Response,
    routing::{delete, get, patch, post, put},
};
use serde::Deserialize;
use tracing::error;

use crate::{
    app::AppDeps,
    auth::AuthenticatedUser,
    character::services::{
        self,
        avatar::{
            ClearCharacterAvatarError, ClearCharacterAvatarInput, SetCharacterAvatarError,
            SetCharacterAvatarInput,
        },
        create::{CreateCharacterError, CreateCharacterInput},
        delete::{DeleteCharacterError, DeleteCharacterInput},
        update::{UpdateCharacterError, UpdateCharacterInput},
    },
    platform::{
        http::{
            error::{ApiError, codes},
            extract::{ExcludeSocketParticipants, ValidatedJson, read_multipart_file},
            response::{self, Empty},
        },
    },
};

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterCreateRequest {
    name: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterUpdateRequest {
    name: Option<String>,
    description: Option<String>,
}

async fn get_all_user_characters(
    Extension(user): Extension<AuthenticatedUser>,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    let characters = services::get::get_characters(&deps, &user.uid)
        .await
        .map_err(|err| {
            error!(target: "character::api::get_all_user_characters", ?err, "Failed to get characters");

            ApiError::internal(codes::CHARACTER_LIST_FAILED)
        })?;

    Ok(response::ok(characters))
}

async fn create_character(
    Extension(user): Extension<AuthenticatedUser>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<CharacterCreateRequest>,
) -> Result<Response, ApiError> {
    if request.name.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHARACTER_INVALID_NAME));
    }

    let input = CreateCharacterInput {
        name: request.name,
        description: request.description,
        creator_uid: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let character = services::create::create_character(&deps, input)
        .await
        .map_err(|err| match err {
            CreateCharacterError::InternalError(_) => {
                error!(target: "character::api::create_character", ?err, "Failed to create character");

                ApiError::internal(codes::CHARACTER_CREATE_FAILED)
            }
        })?;

    Ok(response::created(character))
}

async fn patch_character(
    Extension(user): Extension<AuthenticatedUser>,
    Path(character_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<CharacterUpdateRequest>,
) -> Result<Response, ApiError> {
    if request.name.is_none() && request.description.is_none() {
        return Err(ApiError::bad_request(codes::CHARACTER_NO_FIELDS_PROVIDED));
    }

    if character_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHARACTER_INVALID_UID));
    }

    if let Some(name) = &request.name
        && name.trim().is_empty()
    {
        return Err(ApiError::bad_request(codes::CHARACTER_INVALID_NAME));
    }

    let input = UpdateCharacterInput {
        character_uid,
        user_uid: user.uid.clone(),
        name: request.name,
        description: request.description,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let character = services::update::update_character(&deps, input)
        .await
        .map_err(|err| match err {
            UpdateCharacterError::NotFound => ApiError::not_found(codes::CHARACTER_NOT_FOUND),
            UpdateCharacterError::InternalError(_) => {
                error!(target: "character::api::patch_character", ?err, "Failed to patch character");
                ApiError::internal(codes::CHARACTER_UPDATE_FAILED)
            }
        })?;

    Ok(response::ok(character))
}

async fn put_character_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    Path(character_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    multipart: Multipart,
) -> Result<Response, ApiError> {
    if character_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHARACTER_INVALID_UID));
    }

    let max_file_bytes = deps
        .config
        .policies
        .files
        .character_avatar
        .max_file_size
        .to_bytes() as usize;

    let (original_name, content_type, content) =
        read_multipart_file(multipart, max_file_bytes).await?;

    let input = SetCharacterAvatarInput {
        character_uid,
        user_uid: user.uid.clone(),
        content_type,
        original_name,
        content,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let character = services::avatar::set_character_avatar(&deps, input)
        .await
        .map_err(|err| match err {
            SetCharacterAvatarError::NotFound => ApiError::not_found(codes::CHARACTER_NOT_FOUND),
            SetCharacterAvatarError::ReplaceAvatar(replace_err) => {
                error!(target: "character::api::put_character_avatar", ?replace_err, "Failed to replace character avatar");
                ApiError::from(replace_err)
            }
            SetCharacterAvatarError::InternalError(_) => {
                error!(target: "character::api::put_character_avatar", ?err, "Failed to set character avatar");
                ApiError::internal(codes::CHARACTER_UPDATE_FAILED)
            }
        })?;

    Ok(response::ok(character))
}

async fn delete_character_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    Path(character_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    if character_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHARACTER_INVALID_UID));
    }

    let input = ClearCharacterAvatarInput {
        character_uid,
        user_uid: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let character = services::avatar::clear_character_avatar(&deps, input)
        .await
        .map_err(|err| match err {
            ClearCharacterAvatarError::NotFound => ApiError::not_found(codes::CHARACTER_NOT_FOUND),
            ClearCharacterAvatarError::InternalError(_) => {
                error!(target: "character::api::delete_character_avatar", ?err, "Failed to clear character avatar");
                ApiError::internal(codes::CHARACTER_UPDATE_FAILED)
            }
        })?;

    Ok(response::ok(character))
}

async fn delete_character(
    Extension(user): Extension<AuthenticatedUser>,
    Path(character_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    if character_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHARACTER_INVALID_UID));
    }

    let input = DeleteCharacterInput {
        character_uid,
        user_uid: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::delete::delete_character(&deps, input)
        .await
        .map_err(|err| match err {
            DeleteCharacterError::NotFound => ApiError::not_found(codes::CHARACTER_NOT_FOUND),
            DeleteCharacterError::InternalError(_) => {
                error!(target: "character::api::delete_character", ?err, "Failed to delete character");

                ApiError::internal(codes::CHARACTER_DELETE_FAILED)
            }
        })?;

    Ok(response::ok(Empty {}))
}

pub fn routes() -> axum::Router<AppDeps> {
    axum::Router::new()
        .route("/", get(get_all_user_characters))
        .route("/", post(create_character))
        .route("/{character_uid}", patch(patch_character))
        .route("/{character_uid}", delete(delete_character))
        .route("/{character_uid}/avatar", put(put_character_avatar))
        .route("/{character_uid}/avatar", delete(delete_character_avatar))
}
