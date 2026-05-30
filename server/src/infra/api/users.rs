use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Multipart, State},
    response::Response,
    routing::{get, patch, post},
};
use gg_core::{
    application::user::{UpdateUserPrerequisites, UserUpdateService},
    domain::prelude::*,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;

use crate::infra::{
    api::{
        auth::{AuthLayerExt, create_auth_state},
        extract::ExcludeSocketParticipants,
        multipart::read_multipart_file,
        personas::FileUploadResponse,
        response::{self, ToJson},
    },
    config::Config,
    db::UserRepository,
    event_bus::InMemoryEventBus,
    file::FileServices,
};

impl ToJson for User {
    fn to_json(self) -> serde_json::Value {
        json!({
            "id": self.uid.inner(),
            "username": self.name.inner(),
            "avatar_uid": self.avatar_uid.map(|value| value.into_inner()),
        })
    }
}

#[derive(Clone)]
pub struct UserState<U: UpdateUserUseCase, F: CreateFileUseCase> {
    update_user_use_case: U,
    create_file_use_case: F,
}

async fn get_current_user(Extension(user): Extension<User>) -> Response {
    response::ok(user)
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserUpdateRequest {
    avatar_uid: Option<Option<String>>,
}

async fn patch_current_user(
    Extension(user): Extension<User>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(user_state): State<UserState<impl UpdateUserUseCase, impl CreateFileUseCase>>,
    Json(request): Json<UserUpdateRequest>,
) -> Response {
    if request.avatar_uid.is_none() {
        return response::bad_request(&["At least one field should be provided"]);
    }

    let avatar_uid = match request.avatar_uid {
        None => PatchField::Unchanged,
        Some(None) => PatchField::Clear,
        Some(Some(value)) => match FileId::try_new(value) {
            Ok(file_id) => PatchField::Set(file_id),
            Err(_) => return response::bad_request(&["Invalid avatar uid"]),
        },
    };

    let command = UpdateUserCommand {
        avatar_uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    match user_state
        .update_user_use_case
        .update_user(command, user.uid.clone())
        .await
    {
        Ok(updated_user) => response::ok(updated_user),
        Err(UpdateUserError::NotFound) => response::not_found(&["User not found"]),
        Err(UpdateUserError::FileNotFound) => response::not_found(&["Avatar file not found"]),
        Err(UpdateUserError::ValidationError(message)) => response::bad_request(&[&message]),
        Err(UpdateUserError::InternalError(_)) => {
            response::internal_error(&["Failed to update user"])
        }
        Err(UpdateUserError::AccessDenied) => response::forbidden(&["Access denied"]),
    }
}

async fn upload_user_avatar(
    Extension(user): Extension<User>,
    State(user_state): State<UserState<impl UpdateUserUseCase, impl CreateFileUseCase>>,
    multipart: Multipart,
) -> Response {
    let (original_name, content_type, content) = match read_multipart_file(multipart).await {
        Ok(value) => value,
        Err(message) => return response::bad_request(&[&message]),
    };

    let command = CreateFileCommand {
        content_type,
        original_name,
        scope: FileScope::UserAvatar {
            user_id: user.uid.clone(),
        },
        content,
    };

    match user_state
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
        Err(_) => response::internal_error(&["Failed to upload avatar"]),
    }
}

pub fn create_user_router(
    config: Arc<Config>,
    connection: DatabaseConnection,
    event_bus: InMemoryEventBus,
) -> Router {
    let file_services = FileServices::new(config.clone(), connection.clone());
    let file_repository = file_services.file_repository.clone();

    let user_state = UserState {
        update_user_use_case: UserUpdateService::new(UpdateUserPrerequisites {
            load_user_by_id_port: UserRepository::new(connection.clone()),
            save_user_port: UserRepository::new(connection.clone()),
            event_publisher: event_bus,
            load_file_port: file_repository.clone(),
            activate_file_port: file_repository.clone(),
            orphan_file_port: file_repository,
        }),
        create_file_use_case: file_services.create_file,
    };

    Router::new()
        .route("/me", get(get_current_user))
        .route("/me", patch(patch_current_user))
        .route("/me/avatar", post(upload_user_avatar))
        .with_state(user_state)
        .with_auth(create_auth_state(config, connection))
}
