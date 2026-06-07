use axum::{
    Extension, Json,
    extract::{Multipart, State},
    response::Response,
    routing::{get, patch, post},
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    app::AppDeps,
    auth::public::AuthenticatedUser,
    file::public::{CreateFileError, CreateFileInput, FileScope, create_file_for_user},
    platform::{
        http::{
            extract::ExcludeSocketParticipants,
            multipart::read_multipart_file,
            response::{self, ToJson},
        },
        types::PatchField,
    },
    user::{
        db::user::User,
        services::update::{UpdateUserError, update_user},
    },
};

impl ToJson for User {
    fn to_json(self) -> serde_json::Value {
        json!({
            "id": self.uid,
            "username": self.name,
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

async fn get_current_user(Extension(user): Extension<AuthenticatedUser>) -> Response {
    response::ok(user)
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserUpdateRequest {
    avatar_uid: Option<Option<String>>,
}

async fn patch_current_user(
    Extension(user): Extension<AuthenticatedUser>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    Json(request): Json<UserUpdateRequest>,
) -> Response {
    if request.avatar_uid.is_none() {
        return response::bad_request(&["At least one field should be provided"]);
    }

    let avatar_uid = match request.avatar_uid {
        None => PatchField::Unchanged,
        Some(None) => PatchField::Clear,
        Some(Some(value)) => PatchField::Set(value),
    };

    let exclude_participants: Vec<String> = exclude_participant.into_iter().collect();

    match update_user(&deps, &user.uid, avatar_uid, exclude_participants).await {
        Ok(updated_user) => response::ok(updated_user),
        Err(UpdateUserError::NotFound) => response::not_found(&["User not found"]),
        Err(UpdateUserError::FileNotFound) => response::not_found(&["Avatar file not found"]),
        Err(UpdateUserError::ValidationError(message)) => response::bad_request(&[&message]),
        Err(UpdateUserError::InternalError(_)) => {
            response::internal_error(&["Failed to update user"])
        }
    }
}

async fn upload_user_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    State(deps): State<AppDeps>,
    multipart: Multipart,
) -> Response {
    let (original_name, content_type, content) = match read_multipart_file(multipart).await {
        Ok(value) => value,
        Err(message) => return response::bad_request(&[&message]),
    };

    let input = CreateFileInput {
        content_type,
        original_name,
        scope: FileScope::UserAvatar {
            user_id: user.uid.clone(),
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
        Err(_) => response::internal_error(&["Failed to upload avatar"]),
    }
}

pub fn routes() -> axum::Router<AppDeps> {
    axum::Router::new()
        .route("/me", get(get_current_user))
        .route("/me", patch(patch_current_user))
        .route("/me/avatar", post(upload_user_avatar))
}
