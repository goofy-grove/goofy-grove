use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct FileMeta {
    pub id: FileId,
    pub filename: Filename,
    pub uploaded_by: UserId,
    pub scope: FileScope,
    pub uploaded_at: UploadedAt,
    pub status: FileStatus,
    pub original_name: FileOriginalName,
    pub content_type: FileContentType,
    pub size: FileSize,
}

#[derive(Debug, Clone)]
pub struct FilePolicy {
    pub max_size: FileSize,
    pub allowed_content_types: Vec<FileContentType>,
}
