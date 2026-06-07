use axum::{
    Extension, Json, Router,
    extract::{Multipart, Path, State},
    response::Response,
    routing::{delete, get, patch, post},
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    app::AppDeps,
    auth::public::AuthenticatedUser,
    file::public::{CreateFileError, CreateFileInput, FileScope, create_file_for_user},
    persona::{
        db::persona::Persona,
        services::{
            self,
            create::{CreatePersonaError, CreatePersonaInput},
            delete::{DeletePersonaError, DeletePersonaInput},
            update::{UpdatePersonaError, UpdatePersonaInput},
        },
    },
    platform::{
        http::{
            extract::ExcludeSocketParticipants,
            multipart::read_multipart_file,
            response::{self, ToJson},
        },
        types::PatchField,
    },
};

impl ToJson for Persona {
    fn to_json(self) -> serde_json::Value {
        json!({
            "uid": self.uid,
            "name": self.name,
            "description": self.description,
            "creator_uid": self.creator_id,
            "avatar_uid": self.avatar_uid,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FileUploadResponse {
    pub uid: String,
}

impl ToJson for FileUploadResponse {
    fn to_json(self) -> serde_json::Value {
        json!({ "uid": self.uid })
    }
}

async fn get_all_user_personas(
    Extension(user): Extension<AuthenticatedUser>,
    State(deps): State<AppDeps>,
) -> Response {
    match services::get::get_personas(&deps, &user.uid).await {
        Ok(personas) => response::ok(personas),
        Err(err) => {
            error!(target: "application::api::get_all_user_personas", ?err, "Failed to get personas:");
            response::internal_error(&["Failed to get personas"])
        }
    }
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
    Json(request): Json<PersonaCreateRequest>,
) -> Response {
    if request.name.trim().is_empty() {
        return response::bad_request(&["Invalid persona name"]);
    }

    let input = CreatePersonaInput {
        name: request.name,
        description: request.description,
        creator_id: user.uid.clone(),
        avatar_uid: request.avatar_uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match services::create::create_persona(&deps, input).await {
        Ok(persona) => response::created(persona),
        Err(CreatePersonaError::FileNotFound) => response::not_found(&["Avatar file not found"]),
        Err(CreatePersonaError::ValidationError(message)) => response::bad_request(&[&message]),
        Err(err) => {
            error!(target: "application::api::create_persona", ?err, "Failed to create persona:");
            response::internal_error(&["Failed to create persona"])
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonaUpdateRequest {
    name: Option<String>,
    description: Option<String>,
    avatar_uid: Option<Option<String>>,
}

#[derive(Debug, Clone)]
pub struct DeletePersonaResponse;

impl ToJson for DeletePersonaResponse {
    fn to_json(self) -> serde_json::Value {
        json!({})
    }
}

async fn patch_persona(
    Extension(user): Extension<AuthenticatedUser>,
    Path(persona_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    Json(request): Json<PersonaUpdateRequest>,
) -> Response {
    if request.name.is_none() && request.description.is_none() && request.avatar_uid.is_none() {
        return response::bad_request(&["At least one field should be provided"]);
    }

    if persona_id.trim().is_empty() {
        return response::bad_request(&["Invalid persona id"]);
    }

    if let Some(name) = &request.name
        && name.trim().is_empty()
    {
        return response::bad_request(&["Invalid persona name"]);
    }

    let avatar_uid = match request.avatar_uid {
        None => PatchField::Unchanged,
        Some(None) => PatchField::Clear,
        Some(Some(value)) => PatchField::Set(value),
    };

    let input = UpdatePersonaInput {
        id: persona_id,
        user_id: user.uid.clone(),
        name: request.name,
        description: request.description,
        avatar_uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match services::update::update_persona(&deps, input).await {
        Ok(persona) => response::ok(persona),
        Err(UpdatePersonaError::NotFound) => response::not_found(&["Persona not found"]),
        Err(UpdatePersonaError::FileNotFound) => response::not_found(&["Avatar file not found"]),
        Err(UpdatePersonaError::ValidationError(message)) => response::bad_request(&[&message]),
        Err(err) => {
            error!(target: "application::api::patch_persona", ?err, "Failed to patch persona:");
            response::internal_error(&["Failed to patch persona"])
        }
    }
}

async fn upload_persona_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    Path(persona_id): Path<String>,
    State(deps): State<AppDeps>,
    multipart: Multipart,
) -> Response {
    if persona_id.trim().is_empty() {
        return response::bad_request(&["Invalid persona id"]);
    }

    let (original_name, content_type, content) = match read_multipart_file(multipart).await {
        Ok(value) => value,
        Err(message) => return response::bad_request(&[&message]),
    };

    let input = CreateFileInput {
        content_type,
        original_name,
        scope: FileScope::PersonaAvatar {
            user_id: user.uid.clone(),
            persona_id: persona_id.clone(),
        },
        content,
    };

    match create_file_for_user(&deps, input, &user.uid).await {
        Ok(file_id) => response::ok(FileUploadResponse { uid: file_id }),
        Err(CreateFileError::AccessDenied) => response::forbidden(&["Access denied"]),
        Err(CreateFileError::PolicyViolation(_)) => {
            response::bad_request(&["File does not match upload policy"])
        }
        Err(CreateFileError::PolicyForScopeNotFound) => {
            response::internal_error(&["Upload policy not configured"])
        }
        Err(err) => {
            error!(target: "application::api::upload_persona_avatar", ?err, "Failed to upload persona avatar");
            response::internal_error(&["Failed to upload avatar"])
        }
    }
}

async fn delete_persona(
    Extension(user): Extension<AuthenticatedUser>,
    Path(persona_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Response {
    if persona_id.trim().is_empty() {
        return response::bad_request(&["Invalid persona id"]);
    }

    let input = DeletePersonaInput {
        id: persona_id,
        user_id: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match services::delete::delete_persona(&deps, input).await {
        Ok(()) => response::ok(DeletePersonaResponse),
        Err(DeletePersonaError::NotFound) => response::not_found(&["Persona not found"]),
        Err(err) => {
            error!(target: "application::api::delete_persona", ?err, "Failed to delete persona:");
            response::internal_error(&["Failed to delete persona"])
        }
    }
}

pub fn routes() -> Router<AppDeps> {
    Router::new()
        .route("/", get(get_all_user_personas))
        .route("/", post(create_persona))
        .route("/{id}", patch(patch_persona))
        .route("/{id}", delete(delete_persona))
        .route("/{id}/avatar", post(upload_persona_avatar))
}
