use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct CreateFileService<
    S: SaveFileToStoragePort,
    S1: SaveFilePort,
    I: IdGenerator,
    R: ResolveFilenamePort,
    E: EnsureFileCreatePort,
    P: LoadScopePolicyPort,
    D: DeleteFileFromStoragePort,
    C: Clock,
> {
    id_generator_port: I,
    save_file_to_storage_port: S,
    save_file_port: S1,
    resolve_filename_port: R,
    ensure_file_create_port: E,
    load_scope_policy_port: P,
    delete_file_from_storage_port: D,
    clock: C,
}

impl<
    S: SaveFileToStoragePort,
    S1: SaveFilePort,
    I: IdGenerator,
    R: ResolveFilenamePort,
    E: EnsureFileCreatePort,
    P: LoadScopePolicyPort,
    D: DeleteFileFromStoragePort,
    C: Clock
> CreateFileService<S, S1, I, R, E, P, D, C>
{
    pub fn new(
        id_generator_port: I,
        save_file_to_storage_port: S,
        save_file_port: S1,
        resolve_filename_port: R,
        ensure_file_create_port: E,
        load_scope_policy_port: P,
        delete_file_from_storage_port: D,
        clock: C,
    ) -> CreateFileService<S, S1, I, R, E, P, D, C> {
        CreateFileService {
            id_generator_port,
            save_file_to_storage_port,
            save_file_port,
            resolve_filename_port,
            ensure_file_create_port,
            load_scope_policy_port,
            delete_file_from_storage_port,
            clock,
        }
    }
}

impl<
    S: SaveFileToStoragePort,
    S1: SaveFilePort,
    I: IdGenerator,
    R: ResolveFilenamePort,
    E: EnsureFileCreatePort,
    P: LoadScopePolicyPort,
    D: DeleteFileFromStoragePort,
    C: Clock,
> CreateFileUseCase for CreateFileService<S, S1, I, R, E, P, D, C>
{
    async fn create_file(
        &self,
        command: CreateFileCommand,
        user_id: UserId,
    ) -> Result<FileId, CreateFileError> {
        let CreateFileCommand {
            content_type,
            original_name,
            scope,
            content,
        } = command;
        let size = FileSize::try_new(content.inner().len())
            .map_err(|err| CreateFileError::ValidationError(err.to_string()))?;

        self.ensure_file_create_port
            .ensure_file_create(&scope, &user_id)
            .await
            .map_err(|err| match err {
                EnsureFileCreatePortError::InternalError(err) => {
                    CreateFileError::InternalError(format!("{:?}", err))
                }
                EnsureFileCreatePortError::AccessDenied => CreateFileError::AccessDenied,
            })?;

        self.load_scope_policy_port
            .load_scope_policy(&scope)
            .await
            .map_err(|err| match err {
                LoadScopePolicyPortError::InternalError(err) => {
                    CreateFileError::InternalError(format!("{:?}", err))
                }
                LoadScopePolicyPortError::PolicyForScopeNotFound => {
                    CreateFileError::PolicyForScopeNotFound
                }
            })
            .and_then(|policy| {
                assert_file_matches_policy(&size, &content_type, policy)
                    .map_err(CreateFileError::PolicyViolation)
            })?;

        let id = FileId::try_new(self.id_generator_port.generate())
            .map_err(|err| CreateFileError::ValidationError(format!("{err}")))?;
        let filename = self
            .resolve_filename_port
            .resolve_filename(&id, &original_name)
            .await
            .map_err(|err| match err {
                ResolveFilenamePortError::InternalError(err) => {
                    CreateFileError::InternalError(format!("{:?}", err))
                }
            })?;

        let file_meta = FileMeta {
            id,
            filename,
            uploaded_by: user_id,
            scope,
            original_name,
            content_type,
            size,
            status: FileStatus::Created,
            uploaded_at: UploadedAt::try_new(self.clock.timestamp())
                .map_err(|err| CreateFileError::ValidationError(format!("{err}")))?,
        };

        self.save_file_to_storage_port
            .save_file_to_storage(&file_meta, content)
            .await
            .map_err(|err| match err {
                SaveFileToStoragePortError::InternalError(err) => {
                    CreateFileError::InternalError(format!("{:?}", err))
                }
            })?;

        match self.save_file_port.save_file(file_meta.clone()).await {
            Ok(file_id) => Ok(file_id),
            Err(err) => {
                let _ = self
                    .delete_file_from_storage_port
                    .delete_file_from_storage(&file_meta)
                    .await;

                match err {
                    SaveFilePortError::InternalError(err) => {
                        Err(CreateFileError::InternalError(format!("{:?}", err)))
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteFileService<
    D: DeleteFileFromStoragePort,
    D1: DeleteFilePort,
    L: LoadFilePort,
    E: EnsureFileDeletePort,
> {
    delete_file_port: D1,
    delete_file_from_storage_port: D,
    load_file_port: L,
    ensure_file_delete_port: E,
}

impl<D: DeleteFileFromStoragePort, D1: DeleteFilePort, L: LoadFilePort, E: EnsureFileDeletePort>
    DeleteFileService<D, D1, L, E>
{
    pub fn new(
        delete_file_port: D1,
        delete_file_from_storage_port: D,
        load_file_port: L,
        ensure_file_delete_port: E,
    ) -> DeleteFileService<D, D1, L, E> {
        DeleteFileService {
            delete_file_port,
            delete_file_from_storage_port,
            load_file_port,
            ensure_file_delete_port,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetFileService<L: LoadFileFromStoragePort, L1: LoadFilePort, E: EnsureFileReadPort> {
    load_file_from_storage_port: L,
    load_file_port: L1,
    ensure_file_read_port: E,
}

impl<L: LoadFileFromStoragePort, L1: LoadFilePort, E: EnsureFileReadPort> GetFileService<L, L1, E> {
    pub fn new(
        load_file_from_storage_port: L,
        load_file_port: L1,
        ensure_file_read_port: E,
    ) -> GetFileService<L, L1, E> {
        GetFileService {
            load_file_from_storage_port,
            load_file_port,
            ensure_file_read_port,
        }
    }
}

impl<L: LoadFileFromStoragePort, L1: LoadFilePort, E: EnsureFileReadPort> GetFileQuery
    for GetFileService<L, L1, E>
{
    async fn get_file(
        &self,
        file_id: FileId,
        user_id: UserId,
    ) -> Result<FileContent, GetFileQueryError> {
        let file_meta = self
            .load_file_port
            .load_file(file_id)
            .await
            .map_err(|err| match err {
                LoadFilePortError::FileNotFound => GetFileQueryError::FileNotFound,
                LoadFilePortError::InternalError(err) => {
                    GetFileQueryError::InternalError(format!("{:?}", err))
                }
            })?;

        self.ensure_file_read_port
            .ensure_file_read(&file_meta, &user_id)
            .await
            .map_err(|err| match err {
                EnsureFileReadPortError::InternalError(err) => {
                    GetFileQueryError::InternalError(format!("{:?}", err))
                }
                EnsureFileReadPortError::AccessDenied => GetFileQueryError::AccessDenied,
            })?;

        self.load_file_from_storage_port
            .load_file_from_storage(&file_meta)
            .await
            .map_err(|err| GetFileQueryError::InternalError(format!("{:?}", err)))
    }
}

impl<D: DeleteFileFromStoragePort, D1: DeleteFilePort, L: LoadFilePort, E: EnsureFileDeletePort>
    DeleteFileUseCase for DeleteFileService<D, D1, L, E>
{
    async fn delete_file(
        &self,
        command: DeleteFileCommand,
        user_id: UserId,
    ) -> Result<(), DeleteFileError> {
        let DeleteFileCommand { id } = command;

        let file = self
            .load_file_port
            .load_file(id)
            .await
            .map_err(|err| match err {
                LoadFilePortError::FileNotFound => DeleteFileError::FileNotFound,
                LoadFilePortError::InternalError(err) => {
                    DeleteFileError::InternalError(format!("{:?}", err))
                }
            })?;

        self.ensure_file_delete_port
            .ensure_file_delete(&file, &user_id)
            .await
            .map_err(|err| match err {
                EnsureFileDeletePortError::InternalError(err) => {
                    DeleteFileError::InternalError(format!("{:?}", err))
                }
                EnsureFileDeletePortError::AccessDenied => DeleteFileError::AccessDenied,
            })?;

        self.delete_file_from_storage_port
            .delete_file_from_storage(&file)
            .await
            .map_err(|err| match err {
                DeleteFileFromStoragePortError::FileNotFound => DeleteFileError::FileNotFound,
                DeleteFileFromStoragePortError::InternalError(err) => {
                    DeleteFileError::InternalError(format!("{:?}", err))
                }
            })?;

        self.delete_file_port
            .delete_file(file.id)
            .await
            .map_err(|err| match err {
                DeleteFilePortError::FileNotFound => DeleteFileError::FileNotFound,
                DeleteFilePortError::InternalError(err) => {
                    DeleteFileError::InternalError(format!("{:?}", err))
                }
            })?;

        Ok(())
    }
}
