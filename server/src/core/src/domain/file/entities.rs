use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct FileMeta {
    pub id: FileId,
    pub filename: Filename,
    pub owner_id: UserId,
    pub original_name: FileOriginalName,
    pub content_type: FileContentType,
    pub size: FileSize,
}
