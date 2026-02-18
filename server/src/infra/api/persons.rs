use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
};
use gg_core::{application::person::GetPersonsService, domain::prelude::*};
use sea_orm::DatabaseConnection;
use serde_json::json;

use crate::infra::{
    api::auth::{AuthLayerExt, create_auth_state},
    config::Config,
    db::PersonRepository,
};

#[derive(Debug, Clone)]
pub struct PersonState<Q: GetPersonsQuery> {
    get_persons_query: Q,
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
    State(person_state): State<PersonState<impl GetPersonsQuery>>,
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

pub fn create_person_router(config: Arc<Config>, connection: DatabaseConnection) -> Router {
    let persons_state = PersonState {
        get_persons_query: GetPersonsService::new(PersonRepository::new(connection.clone())),
    };

    Router::new()
        .route("/", get(get_all_user_persons))
        .with_state(persons_state)
        .with_auth(create_auth_state(config, connection))
}
