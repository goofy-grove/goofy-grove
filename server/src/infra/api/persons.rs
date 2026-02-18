use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use gg_core::{
    application::person::{GetPersonsService, PersonCreateService},
    domain::prelude::*,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;

use crate::infra::{
    api::auth::{AuthLayerExt, create_auth_state},
    config::Config,
    db::PersonRepository,
    id_generator::UuidGenerator,
};

#[derive(Debug, Clone)]
pub struct PersonState<Q: GetPersonsQuery, C: CreatePersonUseCase> {
    get_persons_query: Q,
    create_person_use_case: C,
}

pub trait PersonToJson {
    fn to_json(&self) -> serde_json::Value;
}

impl PersonToJson for Person {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "uid": self.uid().value(),
            "name": self.name().value(),
            "description": self.description().value(),
            "creator_uid": self.creator_id().value(),
        })
    }
}

pub async fn get_all_user_persons(
    Extension(user): Extension<User>,
    State(person_state): State<PersonState<impl GetPersonsQuery, impl CreatePersonUseCase>>,
) -> Response {
    let persons_result = person_state.get_persons_query.get_persons(user.uid()).await;

    match persons_result {
        Ok(persons) => Json(
            persons
                .iter()
                .map(PersonToJson::to_json)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(err) => {
            log::error!("Failed to get persons: {:?}", err);
            Json(json!({"error": "Failed to get persons"})).into_response()
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersonCreateRequest {
    name: String,
    description: String,
}

pub async fn create_person(
    Extension(user): Extension<User>,
    State(person_state): State<PersonState<impl GetPersonsQuery, impl CreatePersonUseCase>>,
    Json(request): Json<PersonCreateRequest>,
) -> Response {
    let command = CreatePersonCommand::new(
        PersonName::new(request.name),
        user.uid().to_owned(),
        PersonDescription::new(request.description),
    );

    match person_state
        .create_person_use_case
        .create_person(command)
        .await
    {
        Ok(person) => Json(person.to_json()).into_response(),
        Err(err) => {
            log::error!("Failed to create person: {:?}", err);
            Json(json!({"error": "Failed to create person"})).into_response()
        }
    }
}

pub fn create_person_router(config: Arc<Config>, connection: DatabaseConnection) -> Router {
    let persons_state = PersonState {
        get_persons_query: GetPersonsService::new(PersonRepository::new(connection.clone())),
        create_person_use_case: PersonCreateService::new(
            PersonRepository::new(connection.clone()),
            UuidGenerator,
        ),
    };

    Router::new()
        .route("/", get(get_all_user_persons))
        .route("/", post(create_person))
        .with_state(persons_state)
        .with_auth(create_auth_state(config, connection))
}
