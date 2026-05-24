use crate::domain::prelude::*;

#[derive(Debug)]
pub struct CreateFileCommand {
    pub content_type: FileContentType,
    pub original_name: FileOriginalName,
    pub owner_id: UserId,
    pub content: FileContent,
}

#[derive(Debug, Clone)]
pub struct DeleteFileCommand {
    pub id: FileId,
}

#[derive(Debug)]
pub struct ReplaceFileCommand {
    pub id: FileId,
    pub content_type: FileContentType,
    pub content: FileContent,
}
