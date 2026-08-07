use axum::{
    Extension, Router,
    extract::{Multipart, Path, State},
    response::Response,
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    app::AppDeps,
    auth::AuthenticatedUser,
    file::{CreateFileInput, FileScope, create_file_for_user},
    persona::services::{
        self,
        create::{CreatePersonaError, CreatePersonaInput},
        delete::{DeletePersonaError, DeletePersonaInput},
        update::{UpdatePersonaError, UpdatePersonaInput},
    },
    platform::{
        http::{
            error::{ApiError, codes},
            extract::{ExcludeSocketParticipants, ValidatedJson, read_multipart_file},
            response::{self, Empty},
        },
        types::PatchField,
    },
};

#[derive(Debug, Clone, Serialize)]
pub struct FileUploadResponse {
    pub uid: String,
}

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
    avatar_uid: Option<String>,
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
        avatar_uid: request.avatar_uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let persona = services::create::create_persona(&deps, input)
        .await
        .map_err(|err| match err {
            CreatePersonaError::FileNotFound => ApiError::not_found(codes::PERSONA_AVATAR_NOT_FOUND),
            CreatePersonaError::InvalidFileStatus => {
                ApiError::bad_request(codes::FILE_INVALID_STATUS)
            }
            CreatePersonaError::InvalidFileScope => {
                ApiError::bad_request(codes::FILE_INVALID_SCOPE)
            }
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
    avatar_uid: Option<Option<String>>,
}

async fn patch_persona(
    Extension(user): Extension<AuthenticatedUser>,
    Path(persona_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<PersonaUpdateRequest>,
) -> Result<Response, ApiError> {
    if request.name.is_none() && request.description.is_none() && request.avatar_uid.is_none() {
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

    let avatar_uid = match request.avatar_uid {
        None => PatchField::Unchanged,
        Some(None) => PatchField::Clear,
        Some(Some(value)) => PatchField::Set(value),
    };

    let input = UpdatePersonaInput {
        persona_uid,
        user_uid: user.uid.clone(),
        name: request.name,
        description: request.description,
        avatar_uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let persona = services::update::update_persona(&deps, input)
        .await
        .map_err(|err| match err {
            UpdatePersonaError::NotFound => ApiError::not_found(codes::PERSONA_NOT_FOUND),
            UpdatePersonaError::AccessDenied => ApiError::forbidden(codes::PERSONA_ACCESS_DENIED),
            UpdatePersonaError::FileNotFound => {
                ApiError::not_found(codes::PERSONA_AVATAR_NOT_FOUND)
            }
            UpdatePersonaError::InvalidFileStatus => {
                ApiError::bad_request(codes::FILE_INVALID_STATUS)
            }
            UpdatePersonaError::InvalidFileScope => {
                ApiError::bad_request(codes::FILE_INVALID_SCOPE)
            }
            UpdatePersonaError::InternalError(_) => {
                error!(target: "application::api::patch_persona", ?err, "Failed to patch persona:");
                ApiError::internal(codes::PERSONA_UPDATE_FAILED)
            }
        })?;

    Ok(response::ok(persona))
}

async fn upload_persona_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    Path(persona_uid): Path<String>,
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

    let input = CreateFileInput {
        content_type,
        original_name,
        scope: FileScope::PersonaAvatar {
            user_uid: user.uid.clone(),
            persona_uid: persona_uid.clone(),
        },
        content,
    };

    let file_uid = create_file_for_user(&deps, input, &user.uid)
        .await
        .map_err(|err| {
            error!(target: "application::api::upload_persona_avatar", ?err, "Failed to upload persona avatar");

            ApiError::from(err)
        })?;

    Ok(response::created(FileUploadResponse { uid: file_uid }))
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
            DeletePersonaError::AccessDenied => ApiError::forbidden(codes::PERSONA_ACCESS_DENIED),
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
        .route("/{persona_uid}/avatar", post(upload_persona_avatar))
}
