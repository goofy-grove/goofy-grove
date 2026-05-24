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
        let file_extension = Path::new(command.original_name().inner())
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default();

        let file_id = self.id_generator_port.generate();
        let filename = format!("{}-{}", file_id, file_extension);

        let file_meta = FileMeta::new(
            FileId::try_new(file_id)
                .map_err(|err| CreateFileError::ValidationError(format!("{err}")))?,
            Filename::try_new(filename)
                .map_err(|err| CreateFileError::ValidationError(format!("{err}")))?,
            command.owner_id().clone(),
            command.original_name().clone(),
            command.content_type().clone(),
            FileSize::try_new(command.content().inner().len())
                .map_err(|err| CreateFileError::ValidationError(format!("{err}")))?,
        );

        let CreateFileCommand { content, .. } = command;

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
