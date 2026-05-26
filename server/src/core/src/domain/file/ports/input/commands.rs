use crate::domain::prelude::*;

#[derive(Debug)]
pub struct CreateFileCommand {
    pub content_type: FileContentType,
    pub original_name: FileOriginalName,
    pub scope: FileScope,
    pub content: FileContent,
}

#[derive(Debug, Clone)]
pub struct DeleteFileCommand {
    pub id: FileId,
}
