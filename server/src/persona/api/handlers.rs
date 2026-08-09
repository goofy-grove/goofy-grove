use axum::{
    Extension, Router,
    extract::{Multipart, Path, State},
    response::Response,
    routing::{delete, get, patch, post, put},
};
use serde::Deserialize;
use tracing::error;

use crate::{
    app::AppDeps,
    auth::AuthenticatedUser,
    persona::services::{
        self,
        avatar::{
            ClearPersonaAvatarError, ClearPersonaAvatarInput, SetPersonaAvatarError,
            SetPersonaAvatarInput,
        },
        create::{CreatePersonaError, CreatePersonaInput},
        delete::{DeletePersonaError, DeletePersonaInput},
        update::{UpdatePersonaError, UpdatePersonaInput},
    },
    platform::http::{
        error::{ApiError, codes},
        extract::{ExcludeSocketParticipants, ValidatedJson, read_multipart_file},
        response::{self, Empty},
    },
};

async fn get_all_user_personas(
    Extension(user): Extension<AuthenticatedUser>,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    let personas = services::get::get_personas(&deps, &user.uid)
        .await
        .map_err(|err| {
            error!(target: "application::api::get_all_user_personas", ?err, "Failed to get personas:");

            ApiError::internal(codes::PERSONA_LIST_FAILED)
        })?;

    Ok(response::ok(personas))
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonaCreateRequest {
    name: String,
    description: String,
}

async fn create_persona(
    Extension(user): Extension<AuthenticatedUser>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<PersonaCreateRequest>,
) -> Result<Response, ApiError> {
    if request.name.trim().is_empty() {
        return Err(ApiError::bad_request(codes::PERSONA_INVALID_NAME));
    }

    let input = CreatePersonaInput {
        name: request.name,
        description: request.description,
        creator_uid: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let persona = services::create::create_persona(&deps, input)
        .await
        .map_err(|err| match err {
            CreatePersonaError::InternalError(_) => {
                error!(target: "application::api::create_persona", ?err, "Failed to create persona:");

                ApiError::internal(codes::PERSONA_CREATE_FAILED)
            }
        })?;

    Ok(response::created(persona))
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonaUpdateRequest {
    name: Option<String>,
    description: Option<String>,
}

async fn patch_persona(
    Extension(user): Extension<AuthenticatedUser>,
    Path(persona_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<PersonaUpdateRequest>,
) -> Result<Response, ApiError> {
    if request.name.is_none() && request.description.is_none() {
        return Err(ApiError::bad_request(codes::PERSONA_NO_FIELDS_PROVIDED));
    }

    if persona_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::PERSONA_INVALID_UID));
    }

    if let Some(name) = &request.name
        && name.trim().is_empty()
    {
        return Err(ApiError::bad_request(codes::PERSONA_INVALID_NAME));
    }

    let input = UpdatePersonaInput {
        persona_uid,
        user_uid: user.uid.clone(),
        name: request.name,
        description: request.description,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let persona = services::update::update_persona(&deps, input)
        .await
        .map_err(|err| match err {
            UpdatePersonaError::NotFound => ApiError::not_found(codes::PERSONA_NOT_FOUND),
            UpdatePersonaError::InternalError(_) => {
                error!(target: "application::api::patch_persona", ?err, "Failed to patch persona:");
                ApiError::internal(codes::PERSONA_UPDATE_FAILED)
            }
        })?;

    Ok(response::ok(persona))
}

async fn put_persona_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    Path(persona_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    multipart: Multipart,
) -> Result<Response, ApiError> {
    if persona_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::PERSONA_INVALID_UID));
    }

    let max_file_bytes = deps
        .config
        .policies
        .files
        .persona_avatar
        .max_file_size
        .to_bytes() as usize;

    let (original_name, content_type, content) =
        read_multipart_file(multipart, max_file_bytes).await?;

    let input = SetPersonaAvatarInput {
        persona_uid,
        user_uid: user.uid.clone(),
        content_type,
        original_name,
        content,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let persona = services::avatar::set_persona_avatar(&deps, input)
        .await
        .map_err(|err| match err {
            SetPersonaAvatarError::NotFound => ApiError::not_found(codes::PERSONA_NOT_FOUND),
            SetPersonaAvatarError::ReplaceAvatar(err) => {
                error!(target: "application::api::put_persona_avatar", ?err, "Failed to set persona avatar");
                ApiError::from(err)
            }
            SetPersonaAvatarError::InternalError(_) => {
                error!(target: "application::api::put_persona_avatar", ?err, "Failed to set persona avatar");
                ApiError::internal(codes::PERSONA_UPDATE_FAILED)
            }
        })?;

    Ok(response::ok(persona))
}

async fn delete_persona_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    Path(persona_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    if persona_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::PERSONA_INVALID_UID));
    }

    let input = ClearPersonaAvatarInput {
        persona_uid,
        user_uid: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let persona = services::avatar::clear_persona_avatar(&deps, input)
        .await
        .map_err(|err| match err {
            ClearPersonaAvatarError::NotFound => ApiError::not_found(codes::PERSONA_NOT_FOUND),
            ClearPersonaAvatarError::InternalError(_) => {
                error!(target: "application::api::delete_persona_avatar", ?err, "Failed to clear persona avatar");
                ApiError::internal(codes::PERSONA_UPDATE_FAILED)
            }
        })?;

    Ok(response::ok(persona))
}

async fn delete_persona(
    Extension(user): Extension<AuthenticatedUser>,
    Path(persona_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    if persona_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::PERSONA_INVALID_UID));
    }

    let input = DeletePersonaInput {
        persona_uid,
        user_uid: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::delete::delete_persona(&deps, input)
        .await
        .map_err(|err| match err {
            DeletePersonaError::NotFound => ApiError::not_found(codes::PERSONA_NOT_FOUND),
            DeletePersonaError::InternalError(_) => {
                error!(target: "application::api::delete_persona", ?err, "Failed to delete persona:");
                ApiError::internal(codes::PERSONA_DELETE_FAILED)
            }
        })?;

    Ok(response::ok(Empty {}))
}

pub fn routes() -> Router<AppDeps> {
    Router::new()
        .route("/", get(get_all_user_personas))
        .route("/", post(create_persona))
        .route("/{persona_uid}", patch(patch_persona))
        .route("/{persona_uid}", delete(delete_persona))
        .route("/{persona_uid}/avatar", put(put_persona_avatar))
        .route("/{persona_uid}/avatar", delete(delete_persona_avatar))
}
