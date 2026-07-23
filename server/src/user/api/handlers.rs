use axum::{
    Extension,
    extract::{Multipart, State},
    response::Response,
    routing::{get, patch, post},
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    app::AppDeps,
    auth::public::AuthenticatedUser,
    file::public::{CreateFileInput, FileScope, create_file_for_user},
    platform::{
        http::{
            error::{ApiError, codes},
            extract::{ExcludeSocketParticipants, ValidatedJson, read_multipart_file},
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
            "uid": self.uid,
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
    ValidatedJson(request): ValidatedJson<UserUpdateRequest>,
) -> Result<Response, ApiError> {
    if request.avatar_uid.is_none() {
        return Err(ApiError::bad_request(codes::USER_NO_FIELDS_PROVIDED));
    }

    let avatar_uid = match request.avatar_uid {
        None => PatchField::Unchanged,
        Some(None) => PatchField::Clear,
        Some(Some(value)) => PatchField::Set(value),
    };

    let exclude_participants: Vec<String> = exclude_participant.into_iter().collect();

    let updated_user = update_user(&deps, &user.uid, avatar_uid, exclude_participants)
        .await
        .map_err(|err| match err {
            UpdateUserError::NotFound => ApiError::not_found(codes::USER_NOT_FOUND),
            UpdateUserError::FileNotFound => ApiError::not_found(codes::USER_AVATAR_NOT_FOUND),
            UpdateUserError::InvalidFileStatus => ApiError::bad_request(codes::FILE_INVALID_STATUS),
            UpdateUserError::InvalidFileScope => ApiError::bad_request(codes::FILE_INVALID_SCOPE),
            UpdateUserError::InternalError(_) => ApiError::internal(codes::USER_UPDATE_FAILED),
        })?;

    Ok(response::ok(updated_user))
}

async fn upload_user_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    State(deps): State<AppDeps>,
    multipart: Multipart,
) -> Result<Response, ApiError> {
    let max_file_bytes = deps
        .config
        .policies
        .files
        .user_avatar
        .max_file_size
        .to_bytes() as usize;

    let (original_name, content_type, content) =
        read_multipart_file(multipart, max_file_bytes).await?;

    let input = CreateFileInput {
        content_type,
        original_name,
        scope: FileScope::UserAvatar {
            user_uid: user.uid.clone(),
        },
        content,
    };

    let file_uid = create_file_for_user(&deps, input, &user.uid)
        .await
        .map_err(ApiError::from)?;

    Ok(response::created(FileUploadResponse { uid: file_uid }))
}

pub fn routes() -> axum::Router<AppDeps> {
    axum::Router::new()
        .route("/me", get(get_current_user))
        .route("/me", patch(patch_current_user))
        .route("/me/avatar", post(upload_user_avatar))
}
