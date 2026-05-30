use std::sync::Arc;

use gg_core::application::file::{
    CreateFilePrerequisites, CreateFileService, DeleteFilePrerequisites, DeleteFileService,
    GetFilePrerequisites, GetFileService,
};
use sea_orm::DatabaseConnection;

use crate::infra::{
    clock::ChronoClock,
    config::Config,
    db::FileRepository,
    file::{ConfigScopePolicyLoader, ExtensionResolveFilename, FileAccessContextLoader},
    id_generator::UuidGenerator,
    storage::LocalFileStorage,
};

#[derive(Clone)]
pub struct FileServices {
    pub create_file: CreateFileService<
        LocalFileStorage,
        FileRepository,
        UuidGenerator,
        ExtensionResolveFilename,
        FileAccessContextLoader,
        ConfigScopePolicyLoader,
        LocalFileStorage,
        ChronoClock,
    >,
    pub get_file: GetFileService<LocalFileStorage, FileRepository, FileAccessContextLoader>,
    #[allow(dead_code)]
    pub delete_file: DeleteFileService<
        LocalFileStorage,
        FileRepository,
        FileRepository,
        FileAccessContextLoader,
    >,
    pub file_repository: FileRepository,
}

impl FileServices {
    pub fn new(config: Arc<Config>, connection: DatabaseConnection) -> Self {
        let storage = LocalFileStorage::new(config.storage.files_dir.clone());
        let file_repository = FileRepository::new(connection.clone());
        let access_context = FileAccessContextLoader::new(connection);
        let policy_loader = ConfigScopePolicyLoader::new(Arc::new(config.policies.clone()));

        let create_file = CreateFileService::new(CreateFilePrerequisites {
            id_generator_port: UuidGenerator,
            save_file_to_storage_port: storage.clone(),
            save_file_port: file_repository.clone(),
            resolve_filename_port: ExtensionResolveFilename,
            load_file_create_access_context_port: access_context.clone(),
            load_scope_policy_port: policy_loader,
            delete_file_from_storage_port: storage.clone(),
            clock: ChronoClock,
        });

        let get_file = GetFileService::new(GetFilePrerequisites {
            load_file_from_storage_port: storage.clone(),
            load_file_port: file_repository.clone(),
            load_file_meta_access_context_port: access_context.clone(),
        });

        let delete_file = DeleteFileService::new(DeleteFilePrerequisites {
            delete_file_port: file_repository.clone(),
            delete_file_from_storage_port: storage,
            load_file_port: file_repository.clone(),
            load_file_meta_access_context_port: access_context,
        });

        Self {
            create_file,
            get_file,
            delete_file,
            file_repository,
        }
    }
}
