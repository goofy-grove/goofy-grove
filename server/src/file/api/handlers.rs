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
    auth::public::AuthenticatedUser,
    file::{
        db::file::{self, LoadFileError},
        services::get::{GetFileError, get_file as fetch_file_bytes},
    },
    platform::http::response,
};

async fn get_file(
    Extension(user): Extension<AuthenticatedUser>,
    Path(file_uid): Path<String>,
    State(deps): State<AppDeps>,
) -> Response {
    let meta = match file::load_file(&deps.db, &file_uid).await {
        Ok(meta) => meta,
        Err(LoadFileError::NotFound) => return response::not_found(&["File not found"]),
        Err(err) => {
            error!(target: "application::api::get_file", ?err, "Failed to load file metadata");
            return response::internal_error(&["Failed to get file"]);
        }
    };

    let content_type = meta.content_type.clone();

    match fetch_file_bytes(&deps, &file_uid, &user.uid).await {
        Ok(content) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .body(Body::from(content))
            .unwrap_or_else(|_| response::internal_error(&["Failed to build file response"])),
        Err(GetFileError::NotFound) => response::not_found(&["File not found"]),
        Err(GetFileError::AccessDenied) => response::forbidden(&["Access denied"]),
        Err(err) => {
            error!(target: "application::api::get_file", ?err, "Failed to get file");

            response::internal_error(&["Failed to get file"])
        }
    }
}

pub fn routes() -> Router<AppDeps> {
    Router::new().route("/{uid}", get(get_file))
}
