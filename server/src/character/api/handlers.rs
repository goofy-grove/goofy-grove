use axum::{
    Extension, Json,
    extract::{Path, State},
    response::Response,
    routing::{delete, get, patch, post},
};
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    app::AppDeps,
    auth::public::AuthenticatedUser,
    character::{
        db::character::Character,
        services::{
            self,
            create::CreateCharacterInput,
            delete::{DeleteCharacterError, DeleteCharacterInput},
            update::{UpdateCharacterError, UpdateCharacterInput},
        },
    },
    platform::http::{
        extract::ExcludeSocketParticipants,
        response::{self, ToJson},
    },
};

impl ToJson for Character {
    fn to_json(self) -> serde_json::Value {
        json!({
            "uid": self.uid,
            "name": self.name,
            "description": self.description,
            "creator_uid": self.creator_id,
        })
    }
}

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

#[derive(Debug, Clone)]
pub struct DeleteCharacterResponse;

impl ToJson for DeleteCharacterResponse {
    fn to_json(self) -> serde_json::Value {
        json!({})
    }
}

async fn get_all_user_characters(
    Extension(user): Extension<AuthenticatedUser>,
    State(deps): State<AppDeps>,
) -> Response {
    match services::get::get_characters(&deps, &user.uid).await {
        Ok(characters) => response::ok(characters),
        Err(err) => {
            error!(target: "character::api::get_all_user_characters", ?err, "Failed to get characters");
            response::internal_error(&["Failed to get characters"])
        }
    }
}

async fn create_character(
    Extension(user): Extension<AuthenticatedUser>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    Json(request): Json<CharacterCreateRequest>,
) -> Response {
    if request.name.trim().is_empty() {
        return response::bad_request(&["Invalid character name"]);
    }

    let input = CreateCharacterInput {
        name: request.name,
        description: request.description,
        creator_id: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match services::create::create_character(&deps, input).await {
        Ok(character) => response::created(character),
        Err(err) => {
            error!(target: "character::api::create_character", ?err, "Failed to create character");

            response::internal_error(&["Failed to create character"])
        }
    }
}

async fn patch_character(
    Extension(user): Extension<AuthenticatedUser>,
    Path(character_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    Json(request): Json<CharacterUpdateRequest>,
) -> Response {
    if request.name.is_none() && request.description.is_none() {
        return response::bad_request(&["At least one field should be provided"]);
    }

    if character_id.trim().is_empty() {
        return response::bad_request(&["Invalid character id"]);
    }

    let input = UpdateCharacterInput {
        id: character_id,
        user_id: user.uid.clone(),
        name: request.name,
        description: request.description,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match services::update::update_character(&deps, input).await {
        Ok(character) => response::ok(character),
        Err(UpdateCharacterError::NotFound) => response::not_found(&["Character not found"]),
        Err(err) => {
            error!(target: "character::api::patch_character", ?err, "Failed to patch character");

            response::internal_error(&["Failed to patch character"])
        }
    }
}

async fn delete_character(
    Extension(user): Extension<AuthenticatedUser>,
    Path(character_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Response {
    if character_id.trim().is_empty() {
        return response::bad_request(&["Invalid character id"]);
    }

    let input = DeleteCharacterInput {
        id: character_id,
        user_id: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match services::delete::delete_character(&deps, input).await {
        Ok(()) => response::ok(DeleteCharacterResponse),
        Err(DeleteCharacterError::NotFound) => response::not_found(&["Character not found"]),
        Err(err) => {
            error!(target: "character::api::delete_character", ?err, "Failed to delete character");
            response::internal_error(&["Failed to delete character"])
        }
    }
}

pub fn routes() -> axum::Router<AppDeps> {
    axum::Router::new()
        .route("/", get(get_all_user_characters))
        .route("/", post(create_character))
        .route("/{id}", patch(patch_character))
        .route("/{id}", delete(delete_character))
}
