use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    http::HeaderMap,
    response::Response,
    routing::{get, post},
};
use gg_core::{
    application::persona::{GetPersonasService, PersonaCreateService},
    domain::prelude::*,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::infra::{
    api::{
        auth::{AuthLayerExt, create_auth_state},
        response::{self, ToJson},
    },
    config::Config,
    db::PersonaRepository,
    event_bus::InMemoryEventBus,
    id_generator::UuidGenerator,
};

#[derive(Debug, Clone)]
pub struct PersonaState<Q: GetPersonasQuery, C: CreatePersonaUseCase> {
    get_personas_query: Q,
    create_persona_use_case: C,
}

impl ToJson for Persona {
    fn to_json(self) -> serde_json::Value {
        json!({
            "uid": self.uid().inner(),
            "name": self.name().inner(),
            "description": self.description().inner(),
            "creator_uid": self.creator_id().inner(),
        })
    }
}

pub async fn get_all_user_personas(
    Extension(user): Extension<User>,
    State(persona_state): State<PersonaState<impl GetPersonasQuery, impl CreatePersonaUseCase>>,
) -> Response {
    let personas_result = persona_state
        .get_personas_query
        .get_personas(user.uid())
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
    headers: HeaderMap,
    Extension(user): Extension<User>,
    State(persona_state): State<PersonaState<impl GetPersonasQuery, impl CreatePersonaUseCase>>,
    Json(request): Json<PersonaCreateRequest>,
) -> Response {
    let persona_name = match PersonaName::try_new(request.name) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid persona name"]),
    };
    let persona_description = PersonaDescription::new(request.description);
    // TODO: Make as a separate function
    let exclude_participants = match headers.get("x-socket-id") {
        Some(id) => {
            let id = match id.to_str() {
                Ok(value) => value,
                Err(_) => return response::bad_request(&["Invalid x-socket-id header"]),
            };
            let participant_id = match ParticipantId::try_new(id.to_owned()) {
                Ok(value) => value,
                Err(_) => return response::bad_request(&["Invalid x-socket-id header"]),
            };
            vec![participant_id]
        }
        None => vec![],
    };

    let command = CreatePersonaCommand::new(
        persona_name,
        user.uid().to_owned(),
        persona_description,
        exclude_participants,
    );

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
            event_bus,
        ),
    };

    Router::new()
        .route("/", get(get_all_user_personas))
        .route("/", post(create_persona))
        .with_state(personas_state)
        .with_auth(create_auth_state(config, connection))
}
