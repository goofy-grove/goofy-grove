use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    response::Response,
    routing::{delete, get, patch, post},
};
use gg_core::{
    application::character::{
        CharacterCreateService, CharacterDeleteService, CharacterUpdateService,
        CreateCharacterPrerequisites, DeleteCharacterPrerequisites, GetCharactersService,
        UpdateCharacterPrerequisites,
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
    db::CharacterRepository,
    event_bus::InMemoryEventBus,
    id_generator::UuidGenerator,
};

#[derive(Debug, Clone)]
pub struct CharacterState<
    Q: GetCharactersQuery,
    C: CreateCharacterUseCase,
    U: UpdateCharacterUseCase,
    D: DeleteCharacterUseCase,
> {
    get_characters_query: Q,
    create_character_use_case: C,
    update_character_use_case: U,
    delete_character_use_case: D,
}

impl ToJson for Character {
    fn to_json(self) -> serde_json::Value {
        json!({
            "uid": self.uid.inner(),
            "name": self.name.inner(),
            "description": self.description.inner(),
            "creator_uid": self.creator_id.inner(),
        })
    }
}

pub async fn get_all_user_characters(
    Extension(user): Extension<User>,
    State(character_state): State<
        CharacterState<
            impl GetCharactersQuery,
            impl CreateCharacterUseCase,
            impl UpdateCharacterUseCase,
            impl DeleteCharacterUseCase,
        >,
    >,
) -> Response {
    let User { uid, .. } = user;

    let characters_result = character_state
        .get_characters_query
        .get_characters(&uid)
        .await;

    match characters_result {
        Ok(characters) => response::ok(characters),
        Err(err) => {
            error!(target: "application::api::get_all_user_characters", ?err, "Failed to get characters:");

            response::internal_error(&["Failed to get characters"])
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterCreateRequest {
    name: String,
    description: String,
}

pub async fn create_character(
    Extension(user): Extension<User>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(character_state): State<
        CharacterState<
            impl GetCharactersQuery,
            impl CreateCharacterUseCase,
            impl UpdateCharacterUseCase,
            impl DeleteCharacterUseCase,
        >,
    >,
    Json(request): Json<CharacterCreateRequest>,
) -> Response {
    let character_name = match CharacterName::try_new(request.name) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid character name"]),
    };
    let character_description = CharacterDescription::new(request.description);
    let User { uid, .. } = user;
    let command = CreateCharacterCommand {
        name: character_name,
        creator_id: uid,
        description: character_description,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match character_state
        .create_character_use_case
        .create_character(command)
        .await
    {
        Ok(character) => response::created(character),
        Err(err) => {
            error!(target: "application::api::create_character", ?err, "Failed to create character:");

            response::internal_error(&["Failed to create character"])
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterUpdateRequest {
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeleteCharacterResponse;

impl ToJson for DeleteCharacterResponse {
    fn to_json(self) -> serde_json::Value {
        json!({})
    }
}

pub async fn patch_character(
    Extension(user): Extension<User>,
    Path(character_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(character_state): State<
        CharacterState<
            impl GetCharactersQuery,
            impl CreateCharacterUseCase,
            impl UpdateCharacterUseCase,
            impl DeleteCharacterUseCase,
        >,
    >,
    Json(request): Json<CharacterUpdateRequest>,
) -> Response {
    if request.name.is_none() && request.description.is_none() {
        return response::bad_request(&["At least one field should be provided"]);
    }

    let character_id = match CharacterId::try_new(character_id) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid character id"]),
    };

    let character_name = match request.name {
        Some(value) => match CharacterName::try_new(value) {
            Ok(name) => Some(name),
            Err(_) => return response::bad_request(&["Invalid character name"]),
        },
        None => None,
    };
    let character_description = request.description.map(CharacterDescription::new);
    let User { uid, .. } = user;
    let command = UpdateCharacterCommand {
        id: character_id,
        name: character_name,
        description: character_description,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match character_state
        .update_character_use_case
        .update_character(command, uid)
        .await
    {
        Ok(character) => response::ok(character),
        Err(UpdateCharacterError::NotFound) => response::not_found(&["Character not found"]),
        Err(err) => {
            error!(target: "application::api::patch_character", ?err, "Failed to patch character:");

            response::internal_error(&["Failed to patch character"])
        }
    }
}

pub async fn delete_character(
    Extension(user): Extension<User>,
    Path(character_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(character_state): State<
        CharacterState<
            impl GetCharactersQuery,
            impl CreateCharacterUseCase,
            impl UpdateCharacterUseCase,
            impl DeleteCharacterUseCase,
        >,
    >,
) -> Response {
    let character_id = match CharacterId::try_new(character_id) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid character id"]),
    };
    let User { uid, .. } = user;
    let command = DeleteCharacterCommand {
        id: character_id,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match character_state
        .delete_character_use_case
        .delete_character(command, uid)
        .await
    {
        Ok(()) => response::ok(DeleteCharacterResponse),
        Err(DeleteCharacterError::NotFound) => response::not_found(&["Character not found"]),
        Err(err) => {
            error!(target: "application::api::delete_character", ?err, "Failed to delete character:");
            response::internal_error(&["Failed to delete character"])
        }
    }
}

pub fn create_character_router(
    config: Arc<Config>,
    connection: DatabaseConnection,
    event_bus: InMemoryEventBus,
) -> Router {
    let characters_state = CharacterState {
        get_characters_query: GetCharactersService::new(CharacterRepository::new(
            connection.clone(),
        )),
        create_character_use_case: CharacterCreateService::new(CreateCharacterPrerequisites {
            save_character_port: CharacterRepository::new(connection.clone()),
            uid_generator: UuidGenerator,
            event_publisher: event_bus.clone(),
        }),
        update_character_use_case: CharacterUpdateService::new(UpdateCharacterPrerequisites {
            load_character_port: CharacterRepository::new(connection.clone()),
            save_character_port: CharacterRepository::new(connection.clone()),
            event_publisher: event_bus.clone(),
        }),
        delete_character_use_case: CharacterDeleteService::new(DeleteCharacterPrerequisites {
            load_character_port: CharacterRepository::new(connection.clone()),
            delete_character_port: CharacterRepository::new(connection.clone()),
            event_publisher: event_bus.clone(),
        }),
    };

    Router::new()
        .route("/", get(get_all_user_characters))
        .route("/", post(create_character))
        .route("/{id}", patch(patch_character))
        .route("/{id}", delete(delete_character))
        .with_state(characters_state)
        .with_auth(create_auth_state(config, connection))
}
