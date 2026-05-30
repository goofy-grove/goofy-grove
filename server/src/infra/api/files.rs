use std::sync::Arc;

use axum::{
    Extension, Router,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::Response,
    routing::get,
};
use gg_core::{application::file::GetFileService, domain::prelude::*};
use sea_orm::DatabaseConnection;
use tracing::error;

use crate::infra::{
    api::{
        auth::{AuthLayerExt, create_auth_state},
        response,
    },
    config::Config,
    db::FileRepository,
    file::{FileAccessContextLoader, FileServices},
    storage::LocalFileStorage,
};

#[derive(Clone)]
pub struct FilesState {
    get_file_query: GetFileService<LocalFileStorage, FileRepository, FileAccessContextLoader>,
    file_repository: FileRepository,
}

pub fn create_files_router(config: Arc<Config>, connection: DatabaseConnection) -> Router {
    let services = FileServices::new(config.clone(), connection.clone());
    let state = FilesState {
        get_file_query: services.get_file,
        file_repository: services.file_repository,
    };

    Router::new()
        .route("/{uid}", get(get_file))
        .with_state(state)
        .with_auth(create_auth_state(config, connection))
}

async fn get_file(
    Extension(user): Extension<User>,
    Path(file_uid): Path<String>,
    State(state): State<FilesState>,
) -> Response {
    let file_id = match FileId::try_new(file_uid) {
        Ok(value) => value,
        Err(_) => return response::bad_request(&["Invalid file uid"]),
    };

    let meta = match state.file_repository.load_file(file_id.clone()).await {
        Ok(meta) => meta,
        Err(LoadFilePortError::FileNotFound) => return response::not_found(&["File not found"]),
        Err(err) => {
            error!(target: "application::api::get_file", ?err, "Failed to load file metadata");
            return response::internal_error(&["Failed to get file"]);
        }
    };

    let content_type = meta.content_type.inner().clone();

    match state
        .get_file_query
        .get_file(file_id, user.uid.clone())
        .await
    {
        Ok(content) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .body(Body::from(content.into_inner()))
            .unwrap_or_else(|_| response::internal_error(&["Failed to build file response"])),
        Err(GetFileQueryError::FileNotFound) => response::not_found(&["File not found"]),
        Err(GetFileQueryError::AccessDenied) => response::forbidden(&["Access denied"]),
        Err(err) => {
            error!(target: "application::api::get_file", ?err, "Failed to get file");
            response::internal_error(&["Failed to get file"])
        }
    }
}
