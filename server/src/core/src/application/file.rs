use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct CreateFileService<
    S: SaveFileToStoragePort,
    S1: SaveFilePort,
    I: IdGenerator,
    R: ResolveFilenamePort,
    L: LoadFileCreateAccessContextPort,
    P: LoadScopePolicyPort,
    D: DeleteFileFromStoragePort,
    C: Clock,
> {
    id_generator_port: I,
    save_file_to_storage_port: S,
    save_file_port: S1,
    resolve_filename_port: R,
    load_file_create_access_context_port: L,
    load_scope_policy_port: P,
    delete_file_from_storage_port: D,
    clock: C,
}

impl<
    S: SaveFileToStoragePort,
    S1: SaveFilePort,
    I: IdGenerator,
    R: ResolveFilenamePort,
    L: LoadFileCreateAccessContextPort,
    P: LoadScopePolicyPort,
    D: DeleteFileFromStoragePort,
    C: Clock,
> CreateFileService<S, S1, I, R, L, P, D, C>
{
    pub fn new(
        id_generator_port: I,
        save_file_to_storage_port: S,
        save_file_port: S1,
        resolve_filename_port: R,
        load_file_create_access_context_port: L,
        load_scope_policy_port: P,
        delete_file_from_storage_port: D,
        clock: C,
    ) -> CreateFileService<S, S1, I, R, L, P, D, C> {
        CreateFileService {
            id_generator_port,
            save_file_to_storage_port,
            save_file_port,
            resolve_filename_port,
            load_file_create_access_context_port,
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
    L: LoadFileCreateAccessContextPort,
    P: LoadScopePolicyPort,
    D: DeleteFileFromStoragePort,
    C: Clock,
> CreateFileUseCase for CreateFileService<S, S1, I, R, L, P, D, C>
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

        let access_ctx = self
            .load_file_create_access_context_port
            .load_create_context(&scope, &user_id)
            .await
            .map_err(|err| match err {
                LoadFileCreateAccessContextPortError::InternalError(err) => {
                    CreateFileError::InternalError(format!("{:?}", err))
                }
            })?;

        can_create_file(&user_id, &scope, &access_ctx)
            .map_err(|_| CreateFileError::AccessDenied)?;

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
    L1: LoadFileMetaAccessContextPort,
> {
    delete_file_port: D1,
    delete_file_from_storage_port: D,
    load_file_port: L,
    load_file_meta_access_context_port: L1,
}

impl<
    D: DeleteFileFromStoragePort,
    D1: DeleteFilePort,
    L: LoadFilePort,
    L1: LoadFileMetaAccessContextPort,
> DeleteFileService<D, D1, L, L1>
{
    pub fn new(
        delete_file_port: D1,
        delete_file_from_storage_port: D,
        load_file_port: L,
        load_file_meta_access_context_port: L1,
    ) -> DeleteFileService<D, D1, L, L1> {
        DeleteFileService {
            delete_file_port,
            delete_file_from_storage_port,
            load_file_port,
            load_file_meta_access_context_port,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetFileService<
    L: LoadFileFromStoragePort,
    L1: LoadFilePort,
    L2: LoadFileMetaAccessContextPort,
> {
    load_file_from_storage_port: L,
    load_file_port: L1,
    load_file_meta_access_context_port: L2,
}

impl<L: LoadFileFromStoragePort, L1: LoadFilePort, L2: LoadFileMetaAccessContextPort>
    GetFileService<L, L1, L2>
{
    pub fn new(
        load_file_from_storage_port: L,
        load_file_port: L1,
        load_file_meta_access_context_port: L2,
    ) -> GetFileService<L, L1, L2> {
        GetFileService {
            load_file_from_storage_port,
            load_file_port,
            load_file_meta_access_context_port,
        }
    }
}

impl<L: LoadFileFromStoragePort, L1: LoadFilePort, L2: LoadFileMetaAccessContextPort> GetFileQuery
    for GetFileService<L, L1, L2>
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

        let access_ctx = self
            .load_file_meta_access_context_port
            .load_meta_access_context(&file_meta, &user_id)
            .await
            .map_err(|err| match err {
                LoadFileMetaAccessContextPortError::InternalError(err) => {
                    GetFileQueryError::InternalError(format!("{:?}", err))
                }
            })?;

        can_read_file(&user_id, &file_meta, &access_ctx)
            .map_err(|_| GetFileQueryError::AccessDenied)?;

        self.load_file_from_storage_port
            .load_file_from_storage(&file_meta)
            .await
            .map_err(|err| GetFileQueryError::InternalError(format!("{:?}", err)))
    }
}

impl<
    D: DeleteFileFromStoragePort,
    D1: DeleteFilePort,
    L: LoadFilePort,
    L1: LoadFileMetaAccessContextPort,
> DeleteFileUseCase for DeleteFileService<D, D1, L, L1>
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

        let access_ctx = self
            .load_file_meta_access_context_port
            .load_meta_access_context(&file, &user_id)
            .await
            .map_err(|err| match err {
                LoadFileMetaAccessContextPortError::InternalError(err) => {
                    DeleteFileError::InternalError(format!("{:?}", err))
                }
            })?;

        can_delete_file(&user_id, &file, &access_ctx).map_err(|_| DeleteFileError::AccessDenied)?;

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
