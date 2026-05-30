use chrono::DateTime;
use gg_core::domain::prelude::*;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, sea_query::Expr,
};

use crate::infra::db::entities::{files, prelude::Files};
use crate::infra::db::mappers::file::{
    STATUS_ACTIVATED, STATUS_ORPHANED, file_meta_from_model, scope_to_db, status_to_db,
};

#[derive(Debug, Clone)]
pub struct FileRepository {
    connection: DatabaseConnection,
}

impl FileRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl SaveFilePort for FileRepository {
    async fn save_file(&self, meta: FileMeta) -> Result<FileId, SaveFilePortError> {
        let (scope_kind, scope_owner_id, scope_entity_id) = scope_to_db(&meta.scope);
        let uploaded_at = DateTime::from_timestamp(*meta.uploaded_at.inner(), 0)
            .map(|value| value.naive_utc())
            .ok_or_else(|| SaveFilePortError::InternalError("invalid uploaded_at".into()))?;

        let active = files::ActiveModel {
            uid: Set(meta.id.inner().clone()),
            filename: Set(meta.filename.inner().clone()),
            uploaded_by: Set(meta.uploaded_by.inner().clone()),
            scope_kind: Set(scope_kind),
            scope_owner_id: Set(scope_owner_id),
            scope_entity_id: Set(scope_entity_id),
            uploaded_at: Set(uploaded_at),
            status: Set(status_to_db(&meta.status).to_string()),
            original_name: Set(meta.original_name.inner().clone()),
            content_type: Set(meta.content_type.inner().clone()),
            size: Set(*meta.size.inner() as i64),
        };

        Files::insert(active)
            .exec(&self.connection)
            .await
            .map_err(|err| SaveFilePortError::InternalError(err.to_string()))?;

        Ok(meta.id)
    }
}

impl LoadFilePort for FileRepository {
    async fn load_file(&self, id: FileId) -> Result<FileMeta, LoadFilePortError> {
        let model = Files::find_by_id(id.inner())
            .one(&self.connection)
            .await
            .map_err(|err| LoadFilePortError::InternalError(err.to_string()))?
            .ok_or(LoadFilePortError::FileNotFound)?;

        file_meta_from_model(model).map_err(LoadFilePortError::InternalError)
    }
}

impl DeleteFilePort for FileRepository {
    async fn delete_file(&self, id: FileId) -> Result<(), DeleteFilePortError> {
        let result = Files::delete_by_id(id.inner())
            .exec(&self.connection)
            .await
            .map_err(|err| DeleteFilePortError::InternalError(err.to_string()))?;

        if result.rows_affected == 0 {
            return Err(DeleteFilePortError::FileNotFound);
        }

        Ok(())
    }
}

impl ActivateFilePort for FileRepository {
    async fn activate_file(&self, meta: &FileMeta) -> Result<(), ActivateFilePortError> {
        Files::update_many()
            .col_expr(
                files::Column::Status,
                Expr::value(STATUS_ACTIVATED.to_string()),
            )
            .filter(files::Column::Uid.eq(meta.id.inner()))
            .exec(&self.connection)
            .await
            .map_err(|err| ActivateFilePortError::InternalError(err.to_string()))?;

        Ok(())
    }
}

impl OrphanFilePort for FileRepository {
    async fn orphan_file(&self, meta: &FileMeta) -> Result<(), OrphanFilePortError> {
        Files::update_many()
            .col_expr(
                files::Column::Status,
                Expr::value(STATUS_ORPHANED.to_string()),
            )
            .filter(files::Column::Uid.eq(meta.id.inner()))
            .exec(&self.connection)
            .await
            .map_err(|err| OrphanFilePortError::InternalError(err.to_string()))?;

        Ok(())
    }
}
