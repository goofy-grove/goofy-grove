use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Multipart, Path, State},
    response::Response,
    routing::{delete, get, patch, post},
};
use gg_core::{
    application::persona::{
        CreatePersonaPrerequisites, DeletePersonaPrerequisites, GetPersonasService,
        PersonaCreateService, PersonaDeleteService, PersonaUpdateService,
        UpdatePersonaPrerequisites,
    },
    domain::prelude::*,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::infra::{
    api::{
        auth::{AuthLayerExt, create_auth_state},
        extract::ExcludeSocketParticipants,
        multipart::read_multipart_file,
        response::{self, ToJson},
    },
    config::Config,
    db::PersonaRepository,
    event_bus::InMemoryEventBus,
    file::FileServices,
    id_generator::UuidGenerator,
};

#[derive(Clone)]
pub struct PersonaState<
    Q: GetPersonasQuery,
    C: CreatePersonaUseCase,
    U: UpdatePersonaUseCase,
    D: DeletePersonaUseCase,
    F: CreateFileUseCase,
> {
    get_personas_query: Q,
    create_persona_use_case: C,
    update_persona_use_case: U,
    delete_persona_use_case: D,
    create_file_use_case: F,
}

impl ToJson for Persona {
    fn to_json(self) -> serde_json::Value {
        json!({
            "uid": self.uid.inner(),
            "name": self.name.inner(),
            "description": self.description.inner(),
            "creator_uid": self.creator_id.inner(),
            "avatar_uid": self.avatar_uid.map(|value| value.into_inner()),
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

pub async fn get_all_user_personas(
    Extension(user): Extension<User>,
    State(persona_state): State<
        PersonaState<
            impl GetPersonasQuery,
            impl CreatePersonaUseCase,
            impl UpdatePersonaUseCase,
            impl DeletePersonaUseCase,
            impl CreateFileUseCase,
        >,
    >,
) -> Response {
    let personas_result = persona_state
        .get_personas_query
        .get_personas(&user.uid)
        .await;

    match personas_result {
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

pub async fn create_persona(
    Extension(user): Extension<User>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(persona_state): State<
        PersonaState<
            impl GetPersonasQuery,
            impl CreatePersonaUseCase,
            impl UpdatePersonaUseCase,
            impl DeletePersonaUseCase,
            impl CreateFileUseCase,
        >,
    >,
    Json(request): Json<PersonaCreateRequest>,
) -> Response {
    let persona_name = match PersonaName::try_new(request.name) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid persona name"]),
    };
    let persona_description = PersonaDescription::new(request.description);
    let avatar_uid = match request.avatar_uid {
        None => None,
        Some(value) => match FileId::try_new(value) {
            Ok(file_id) => Some(file_id),
            Err(_) => return response::bad_request(&["Invalid avatar uid"]),
        },
    };
    let command = CreatePersonaCommand {
        name: persona_name,
        creator_id: user.uid.clone(),
        description: persona_description,
        avatar_uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match persona_state
        .create_persona_use_case
        .create_persona(command)
        .await
    {
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

pub async fn patch_persona(
    Extension(user): Extension<User>,
    Path(persona_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(persona_state): State<
        PersonaState<
            impl GetPersonasQuery,
            impl CreatePersonaUseCase,
            impl UpdatePersonaUseCase,
            impl DeletePersonaUseCase,
            impl CreateFileUseCase,
        >,
    >,
    Json(request): Json<PersonaUpdateRequest>,
) -> Response {
    if request.name.is_none() && request.description.is_none() && request.avatar_uid.is_none() {
        return response::bad_request(&["At least one field should be provided"]);
    }

    let persona_id = match PersonaId::try_new(persona_id) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid persona id"]),
    };

    let persona_name = match request.name {
        Some(value) => match PersonaName::try_new(value) {
            Ok(name) => Some(name),
            Err(_) => return response::bad_request(&["Invalid persona name"]),
        },
        None => None,
    };
    let persona_description = request.description.map(PersonaDescription::new);
    let avatar_uid = match request.avatar_uid {
        None => PatchField::Unchanged,
        Some(None) => PatchField::Clear,
        Some(Some(value)) => match FileId::try_new(value) {
            Ok(file_id) => PatchField::Set(file_id),
            Err(_) => return response::bad_request(&["Invalid avatar uid"]),
        },
    };

    let command = UpdatePersonaCommand {
        id: persona_id,
        name: persona_name,
        description: persona_description,
        avatar_uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match persona_state
        .update_persona_use_case
        .update_persona(command, user.uid.clone())
        .await
    {
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

pub async fn upload_persona_avatar(
    Extension(user): Extension<User>,
    Path(persona_id): Path<String>,
    State(persona_state): State<
        PersonaState<
            impl GetPersonasQuery,
            impl CreatePersonaUseCase,
            impl UpdatePersonaUseCase,
            impl DeletePersonaUseCase,
            impl CreateFileUseCase,
        >,
    >,
    multipart: Multipart,
) -> Response {
    let persona_id = match PersonaId::try_new(persona_id) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid persona id"]),
    };

    let (original_name, content_type, content) = match read_multipart_file(multipart).await {
        Ok(value) => value,
        Err(message) => return response::bad_request(&[&message]),
    };

    let command = CreateFileCommand {
        content_type,
        original_name,
        scope: FileScope::PersonaAvatar {
            user_id: user.uid.clone(),
            persona_id,
        },
        content,
    };

    match persona_state
        .create_file_use_case
        .create_file(command, user.uid.clone())
        .await
    {
        Ok(file_id) => response::ok(FileUploadResponse {
            uid: file_id.into_inner(),
        }),
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

pub async fn delete_persona(
    Extension(user): Extension<User>,
    Path(persona_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(persona_state): State<
        PersonaState<
            impl GetPersonasQuery,
            impl CreatePersonaUseCase,
            impl UpdatePersonaUseCase,
            impl DeletePersonaUseCase,
            impl CreateFileUseCase,
        >,
    >,
) -> Response {
    let persona_id = match PersonaId::try_new(persona_id) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid persona id"]),
    };
    let command = DeletePersonaCommand {
        id: persona_id,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match persona_state
        .delete_persona_use_case
        .delete_persona(command, user.uid.clone())
        .await
    {
        Ok(()) => response::ok(DeletePersonaResponse),
        Err(DeletePersonaError::NotFound) => response::not_found(&["Persona not found"]),
        Err(err) => {
            error!(target: "application::api::delete_persona", ?err, "Failed to delete persona:");
            response::internal_error(&["Failed to delete persona"])
        }
    }
}

pub fn create_persona_router(
    config: Arc<Config>,
    connection: DatabaseConnection,
    event_bus: InMemoryEventBus,
) -> Router {
    let file_services = FileServices::new(config.clone(), connection.clone());
    let file_repository = file_services.file_repository.clone();

    let personas_state = PersonaState {
        get_personas_query: GetPersonasService::new(PersonaRepository::new(connection.clone())),
        create_persona_use_case: PersonaCreateService::new(CreatePersonaPrerequisites {
            save_persona_port: PersonaRepository::new(connection.clone()),
            uid_generator: UuidGenerator,
            event_publisher: event_bus.clone(),
            load_file_port: file_repository.clone(),
            activate_file_port: file_repository.clone(),
            orphan_file_port: file_repository.clone(),
        }),
        update_persona_use_case: PersonaUpdateService::new(UpdatePersonaPrerequisites {
            load_persona_port: PersonaRepository::new(connection.clone()),
            save_persona_port: PersonaRepository::new(connection.clone()),
            event_publisher: event_bus.clone(),
            load_file_port: file_repository.clone(),
            activate_file_port: file_repository.clone(),
            orphan_file_port: file_repository.clone(),
        }),
        delete_persona_use_case: PersonaDeleteService::new(DeletePersonaPrerequisites {
            load_persona_port: PersonaRepository::new(connection.clone()),
            delete_persona_port: PersonaRepository::new(connection.clone()),
            event_publisher: event_bus.clone(),
            load_file_port: file_repository.clone(),
            orphan_file_port: file_repository,
        }),
        create_file_use_case: file_services.create_file,
    };

    Router::new()
        .route("/", get(get_all_user_personas))
        .route("/", post(create_persona))
        .route("/{id}", patch(patch_persona))
        .route("/{id}", delete(delete_persona))
        .route("/{id}/avatar", post(upload_persona_avatar))
        .with_state(personas_state)
        .with_auth(create_auth_state(config, connection))
}
