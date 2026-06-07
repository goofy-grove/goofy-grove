use axum::{
    Extension,
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
        error::{ApiError, codes},
        extract::{ExcludeSocketParticipants, ValidatedJson},
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
) -> Result<Response, ApiError> {
    let characters = services::get::get_characters(&deps, &user.uid)
        .await
        .map_err(|err| {
            error!(target: "character::api::get_all_user_characters", ?err, "Failed to get characters");

            ApiError::internal(codes::CHARACTER_LIST_FAILED)
        })?;

    Ok(response::ok(characters))
}

async fn create_character(
    Extension(user): Extension<AuthenticatedUser>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<CharacterCreateRequest>,
) -> Result<Response, ApiError> {
    if request.name.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHARACTER_INVALID_NAME));
    }

    let input = CreateCharacterInput {
        name: request.name,
        description: request.description,
        creator_id: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let character = services::create::create_character(&deps, input)
        .await
        .map_err(|err| {
            error!(target: "character::api::create_character", ?err, "Failed to create character");

            ApiError::internal(codes::CHARACTER_CREATE_FAILED)
        })?;

    Ok(response::created(character))
}

async fn patch_character(
    Extension(user): Extension<AuthenticatedUser>,
    Path(character_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<CharacterUpdateRequest>,
) -> Result<Response, ApiError> {
    if request.name.is_none() && request.description.is_none() {
        return Err(ApiError::bad_request(codes::CHARACTER_NO_FIELDS_PROVIDED));
    }

    if character_id.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHARACTER_INVALID_ID));
    }

    let input = UpdateCharacterInput {
        id: character_id,
        user_id: user.uid.clone(),
        name: request.name,
        description: request.description,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    let character = services::update::update_character(&deps, input)
        .await
        .map_err(|err| match err {
            UpdateCharacterError::NotFound => ApiError::not_found(codes::CHARACTER_NOT_FOUND),
            UpdateCharacterError::InternalError(_) => {
                error!(target: "character::api::patch_character", ?err, "Failed to patch character");
                ApiError::internal(codes::CHARACTER_UPDATE_FAILED)
            }
        })?;

    Ok(response::ok(character))
}

async fn delete_character(
    Extension(user): Extension<AuthenticatedUser>,
    Path(character_id): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    if character_id.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHARACTER_INVALID_ID));
    }

    let input = DeleteCharacterInput {
        id: character_id,
        user_id: user.uid.clone(),
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::delete::delete_character(&deps, input)
        .await
        .map_err(|err| match err {
            DeleteCharacterError::NotFound => ApiError::not_found(codes::CHARACTER_NOT_FOUND),
            DeleteCharacterError::InternalError(_) => {
                error!(target: "character::api::delete_character", ?err, "Failed to delete character");


                ApiError::internal(codes::CHARACTER_DELETE_FAILED)
            }
        })?;

    Ok(response::ok(DeleteCharacterResponse))
}

pub fn routes() -> axum::Router<AppDeps> {
    axum::Router::new()
        .route("/", get(get_all_user_characters))
        .route("/", post(create_character))
        .route("/{id}", patch(patch_character))
        .route("/{id}", delete(delete_character))
}
