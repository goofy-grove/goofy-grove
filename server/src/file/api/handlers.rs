use axum::{
    Extension, Router,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::Response,
    routing::get,
};
use tracing::error;

use crate::{
    app::AppDeps,
    auth::AuthenticatedUser,
    file::{
        db::file::{self, LoadFileError},
        services::get::{GetFileError, get_file as fetch_file_bytes},
    },
    platform::http::error::{ApiError, codes},
};

async fn get_file(
    Extension(user): Extension<AuthenticatedUser>,
    Path(file_uid): Path<String>,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    let meta = match file::load_file(&deps.db, &file_uid).await {
        Ok(meta) => meta,
        Err(LoadFileError::NotFound) => return Err(ApiError::not_found(codes::FILE_NOT_FOUND)),
        Err(err) => {
            error!(target: "application::api::get_file", ?err, "Failed to load file metadata");
            return Err(ApiError::internal(codes::FILE_GET_FAILED));
        }
    };

    let content_type = meta.content_type.clone();

    match fetch_file_bytes(&deps, &file_uid, &user.uid).await {
        Ok(content) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .body(Body::from(content))
            .map_err(|_| ApiError::internal(codes::FILE_RESPONSE_BUILD_FAILED)),
        Err(GetFileError::NotFound) => Err(ApiError::not_found(codes::FILE_NOT_FOUND)),
        Err(GetFileError::AccessDenied) => Err(ApiError::forbidden(codes::FILE_ACCESS_DENIED)),
        Err(err) => {
            error!(target: "application::api::get_file", ?err, "Failed to get file");
            Err(ApiError::internal(codes::FILE_GET_FAILED))
        }
    }
}

pub fn routes() -> Router<AppDeps> {
    Router::new().route("/{uid}", get(get_file))
}
