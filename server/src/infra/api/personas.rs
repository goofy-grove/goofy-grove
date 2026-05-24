use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    response::Response,
    routing::{delete, get, patch, post},
};
use gg_core::{
    application::persona::{
        GetPersonasService, PersonaCreateService, PersonaDeleteService, PersonaUpdateService,
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
        response::{self, ToJson},
    },
    config::Config,
    db::PersonaRepository,
    event_bus::InMemoryEventBus,
    id_generator::UuidGenerator,
};

#[derive(Debug, Clone)]
pub struct PersonaState<
    Q: GetPersonasQuery,
    C: CreatePersonaUseCase,
    U: UpdatePersonaUseCase,
    D: DeletePersonaUseCase,
> {
    get_personas_query: Q,
    create_persona_use_case: C,
    update_persona_use_case: U,
    delete_persona_use_case: D,
}

impl ToJson for Persona {
    fn to_json(self) -> serde_json::Value {
        json!({
            "uid": self.uid.inner(),
            "name": self.name.inner(),
            "description": self.description.inner(),
            "creator_uid": self.creator_id.inner(),
        })
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
        >,
    >,
    Json(request): Json<PersonaCreateRequest>,
) -> Response {
    let persona_name = match PersonaName::try_new(request.name) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid persona name"]),
    };
    let persona_description = PersonaDescription::new(request.description);
    let command = CreatePersonaCommand {
        name: persona_name,
        creator_id: user.uid.clone(),
        description: persona_description,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match persona_state
        .create_persona_use_case
        .create_persona(command)
        .await
    {
        Ok(persona) => response::created(persona),
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
        >,
    >,
    Json(request): Json<PersonaUpdateRequest>,
) -> Response {
    if request.name.is_none() && request.description.is_none() {
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
    let command = UpdatePersonaCommand {
        id: persona_id,
        name: persona_name,
        description: persona_description,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match persona_state
        .update_persona_use_case
        .update_persona(command, user.uid.clone())
        .await
    {
        Ok(persona) => response::ok(persona),
        Err(UpdatePersonaError::NotFound) => response::not_found(&["Persona not found"]),
        Err(err) => {
            error!(target: "application::api::patch_persona", ?err, "Failed to patch persona:");

            response::internal_error(&["Failed to patch persona"])
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
    let personas_state = PersonaState {
        get_personas_query: GetPersonasService::new(PersonaRepository::new(connection.clone())),
        create_persona_use_case: PersonaCreateService::new(
            PersonaRepository::new(connection.clone()),
            UuidGenerator,
            event_bus.clone(),
        ),
        update_persona_use_case: PersonaUpdateService::new(
            PersonaRepository::new(connection.clone()),
            PersonaRepository::new(connection.clone()),
            event_bus.clone(),
        ),
        delete_persona_use_case: PersonaDeleteService::new(
            PersonaRepository::new(connection.clone()),
            PersonaRepository::new(connection.clone()),
            event_bus.clone(),
        ),
    };

    Router::new()
        .route("/", get(get_all_user_personas))
        .route("/", post(create_persona))
        .route("/{id}", patch(patch_persona))
        .route("/{id}", delete(delete_persona))
        .with_state(personas_state)
        .with_auth(create_auth_state(config, connection))
}
