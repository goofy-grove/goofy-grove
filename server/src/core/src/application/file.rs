use std::{ffi::OsStr, path::Path};

use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct CreateFileService<S: SaveFileToStoragePort, S1: SaveFilePort, I: IdGenerator> {
    id_generator_port: I,
    save_file_to_storage_port: S,
    save_file_port: S1,
}

impl<S: SaveFileToStoragePort, S1: SaveFilePort, I: IdGenerator> CreateFileUseCase
    for CreateFileService<S, S1, I>
{
    async fn create_file(&self, command: CreateFileCommand) -> Result<FileId, CreateFileError> {
        let CreateFileCommand {
            content_type,
            original_name,
            owner_id,
            content,
        } = command;

        let file_extension = Path::new(original_name.inner())
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default();

        let file_id = self.id_generator_port.generate();
        let filename = format!("{}-{}", file_id, file_extension);

        let file_meta = FileMeta {
            id: FileId::try_new(file_id)
                .map_err(|err| CreateFileError::ValidationError(format!("{err}")))?,
            filename: Filename::try_new(filename)
                .map_err(|err| CreateFileError::ValidationError(format!("{err}")))?,
            owner_id,
            original_name,
            content_type,
            size: FileSize::try_new(content.inner().len())
                .map_err(|err| CreateFileError::ValidationError(format!("{err}")))?,
        };

        self.save_file_to_storage_port
            .save_file_to_storage(&file_meta, content)
            .await
            .map_err(|err| CreateFileError::InternalError(format!("{:?}", err)))?;

        let file_id = self
            .save_file_port
            .save_file(file_meta)
            .await
            .map_err(|err| CreateFileError::InternalError(format!("{:?}", err)))?;

        Ok(file_id)
    }
}
