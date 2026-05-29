use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use super::*;

// --- Helpers ---

fn user_id(val: &str) -> UserId {
    UserId::try_new(val.to_string()).unwrap()
}

fn persona_id(val: &str) -> PersonaId {
    PersonaId::try_new(val.to_string()).unwrap()
}

fn persona_scope(uid: UserId, pid: PersonaId) -> FileScope {
    FileScope::PersonaAvatar {
        user_id: uid,
        persona_id: pid,
    }
}

const FIXED_TIMESTAMP: i64 = 42;

#[derive(Clone)]
struct FixedClock;

impl Clock for FixedClock {
    fn timestamp(&self) -> i64 {
        FIXED_TIMESTAMP
    }
}

fn sample_policy_png_max_1kb() -> FilePolicy {
    FilePolicy {
        max_size: FileSize::try_new(1024).unwrap(),
        allowed_content_types: vec![FileContentType::try_new("image/png".to_string()).unwrap()],
    }
}

fn create_command_png(scope: FileScope, bytes: &[u8]) -> CreateFileCommand {
    CreateFileCommand {
        content_type: FileContentType::try_new("image/png".to_string()).unwrap(),
        original_name: FileOriginalName::try_new("a.png".to_string()).unwrap(),
        scope,
        content: FileContent::new(bytes.to_vec()),
    }
}

// --- Ports (create pipeline) ---

#[derive(Clone)]
struct SaveStorageOk;
impl SaveFileToStoragePort for SaveStorageOk {
    async fn save_file_to_storage(
        &self,
        _meta: &FileMeta,
        _content: FileContent,
    ) -> Result<(), SaveFileToStoragePortError> {
        Ok(())
    }
}

#[derive(Clone)]
struct SaveFileReturnsMetaId;

impl SaveFilePort for SaveFileReturnsMetaId {
    async fn save_file(&self, meta: FileMeta) -> Result<FileId, SaveFilePortError> {
        Ok(meta.id)
    }
}

#[derive(Clone)]
struct SaveFileAssertCreatedMeta;

impl SaveFilePort for SaveFileAssertCreatedMeta {
    async fn save_file(&self, meta: FileMeta) -> Result<FileId, SaveFilePortError> {
        assert_eq!(meta.status, FileStatus::Created);
        assert_eq!(
            meta.uploaded_at,
            UploadedAt::try_new(FIXED_TIMESTAMP).unwrap()
        );
        Ok(meta.id)
    }
}

#[derive(Clone)]
struct SaveFileAlwaysErr;

impl SaveFilePort for SaveFileAlwaysErr {
    async fn save_file(&self, _meta: FileMeta) -> Result<FileId, SaveFilePortError> {
        Err(SaveFilePortError::InternalError("db_fail".into()))
    }
}

#[derive(Clone)]
struct FixedFileId(pub &'static str);

impl IdGenerator for FixedFileId {
    fn generate(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone)]
struct ResolveFilenameEcho;

impl ResolveFilenamePort for ResolveFilenameEcho {
    async fn resolve_filename(
        &self,
        file_id: &FileId,
        _original_name: &FileOriginalName,
    ) -> Result<Filename, ResolveFilenamePortError> {
        Filename::try_new(format!("{}.bin", file_id.inner()))
            .map_err(|e| ResolveFilenamePortError::InternalError(e.to_string()))
    }
}

#[derive(Clone)]
struct LoadCreateContextAllow;

impl LoadFileCreateAccessContextPort for LoadCreateContextAllow {
    async fn load_create_context(
        &self,
        _scope: &FileScope,
        _user_id: &UserId,
    ) -> Result<FileCreateAccessContext, LoadFileCreateAccessContextPortError> {
        Ok(FileCreateAccessContext {
            persona_owned_by_actor: true,
        })
    }
}

#[derive(Clone)]
struct LoadCreateContextDenyPersonaOwnership;

impl LoadFileCreateAccessContextPort for LoadCreateContextDenyPersonaOwnership {
    async fn load_create_context(
        &self,
        _scope: &FileScope,
        _user_id: &UserId,
    ) -> Result<FileCreateAccessContext, LoadFileCreateAccessContextPortError> {
        Ok(FileCreateAccessContext {
            persona_owned_by_actor: false,
        })
    }
}

#[derive(Clone)]
struct LoadPolicyFixed {
    policy: FilePolicy,
}

impl LoadScopePolicyPort for LoadPolicyFixed {
    async fn load_scope_policy(
        &self,
        _scope: &FileScope,
    ) -> Result<FilePolicy, LoadScopePolicyPortError> {
        Ok(self.policy.clone())
    }
}

#[derive(Clone)]
struct LoadPolicyMissing;

impl LoadScopePolicyPort for LoadPolicyMissing {
    async fn load_scope_policy(
        &self,
        _scope: &FileScope,
    ) -> Result<FilePolicy, LoadScopePolicyPortError> {
        Err(LoadScopePolicyPortError::PolicyForScopeNotFound)
    }
}

#[derive(Clone, Default)]
struct RecordingDeleteStorage {
    pub deleted_meta_ids: Arc<Mutex<Vec<FileId>>>,
}

impl DeleteFileFromStoragePort for RecordingDeleteStorage {
    async fn delete_file_from_storage(
        &self,
        meta: &FileMeta,
    ) -> Result<(), DeleteFileFromStoragePortError> {
        self.deleted_meta_ids.lock().unwrap().push(meta.id.clone());
        Ok(())
    }
}

#[derive(Clone, Default)]
struct DeleteFromStoragePanicIfCalled;

impl DeleteFileFromStoragePort for DeleteFromStoragePanicIfCalled {
    async fn delete_file_from_storage(
        &self,
        _meta: &FileMeta,
    ) -> Result<(), DeleteFileFromStoragePortError> {
        panic!("delete_from_storage should not run on happy-path create")
    }
}

/// In-memory blob store: tracks which file ids were written and whether delete was attempted.
#[derive(Clone, Default)]
struct InMemoryBlobStore {
    blob_ids: Arc<Mutex<HashSet<FileId>>>,
    delete_attempts: Arc<Mutex<usize>>,
}

#[derive(Clone)]
struct SaveToInMemoryBlobStore {
    store: InMemoryBlobStore,
}

impl SaveFileToStoragePort for SaveToInMemoryBlobStore {
    async fn save_file_to_storage(
        &self,
        meta: &FileMeta,
        _content: FileContent,
    ) -> Result<(), SaveFileToStoragePortError> {
        self.store.blob_ids.lock().unwrap().insert(meta.id.clone());
        Ok(())
    }
}

#[derive(Clone)]
struct DeleteFromInMemoryBlobStoreFails {
    store: InMemoryBlobStore,
}

impl DeleteFileFromStoragePort for DeleteFromInMemoryBlobStoreFails {
    async fn delete_file_from_storage(
        &self,
        _meta: &FileMeta,
    ) -> Result<(), DeleteFileFromStoragePortError> {
        *self.store.delete_attempts.lock().unwrap() += 1;
        Err(DeleteFileFromStoragePortError::InternalError(
            "storage_delete_fail".into(),
        ))
    }
}

#[tokio::test]
async fn create_file_maps_access_denied() {
    let service = CreateFileService::new(CreateFilePrerequisites {
        id_generator_port: FixedFileId("f-access"),
        save_file_to_storage_port: SaveStorageOk,
        save_file_port: SaveFileReturnsMetaId,
        resolve_filename_port: ResolveFilenameEcho,
        load_file_create_access_context_port: LoadCreateContextDenyPersonaOwnership,
        load_scope_policy_port: LoadPolicyFixed {
            policy: sample_policy_png_max_1kb(),
        },
        delete_file_from_storage_port: DeleteFromStoragePanicIfCalled,
        clock: FixedClock,
    });

    let uid = user_id("u1");
    let scope = persona_scope(uid.clone(), persona_id("p1"));

    assert!(matches!(
        service
            .create_file(create_command_png(scope, &[1]), uid)
            .await,
        Err(CreateFileError::AccessDenied)
    ));
}

#[tokio::test]
async fn create_file_maps_policy_for_scope_missing() {
    let service = CreateFileService::new(CreateFilePrerequisites {
        id_generator_port: FixedFileId("f-nopolicy"),
        save_file_to_storage_port: SaveStorageOk,
        save_file_port: SaveFileReturnsMetaId,
        resolve_filename_port: ResolveFilenameEcho,
        load_file_create_access_context_port: LoadCreateContextAllow,
        load_scope_policy_port: LoadPolicyMissing,
        delete_file_from_storage_port: DeleteFromStoragePanicIfCalled,
        clock: FixedClock,
    });

    let uid = user_id("u1");
    let scope = persona_scope(uid.clone(), persona_id("p1"));

    assert!(matches!(
        service
            .create_file(create_command_png(scope, &[1]), uid)
            .await,
        Err(CreateFileError::PolicyForScopeNotFound)
    ));
}

#[tokio::test]
async fn create_file_policy_violation_on_content_type() {
    let cmd = CreateFileCommand {
        content_type: FileContentType::try_new("image/jpeg".to_string()).unwrap(),
        original_name: FileOriginalName::try_new("x.jpg".to_string()).unwrap(),
        scope: persona_scope(user_id("u1"), persona_id("p1")),
        content: FileContent::new(vec![1]),
    };

    let service = CreateFileService::new(CreateFilePrerequisites {
        id_generator_port: FixedFileId("f-badmime"),
        save_file_to_storage_port: SaveStorageOk,
        save_file_port: SaveFileReturnsMetaId,
        resolve_filename_port: ResolveFilenameEcho,
        load_file_create_access_context_port: LoadCreateContextAllow,
        load_scope_policy_port: LoadPolicyFixed {
            policy: sample_policy_png_max_1kb(),
        },
        delete_file_from_storage_port: DeleteFromStoragePanicIfCalled,
        clock: FixedClock,
    });

    assert!(matches!(
        service.create_file(cmd, user_id("u1")).await,
        Err(CreateFileError::PolicyViolation(_))
    ));
}

#[tokio::test]
async fn create_file_success() {
    let service = CreateFileService::new(CreateFilePrerequisites {
        id_generator_port: FixedFileId("f-good"),
        save_file_to_storage_port: SaveStorageOk,
        save_file_port: SaveFileAssertCreatedMeta,
        resolve_filename_port: ResolveFilenameEcho,
        load_file_create_access_context_port: LoadCreateContextAllow,
        load_scope_policy_port: LoadPolicyFixed {
            policy: sample_policy_png_max_1kb(),
        },
        delete_file_from_storage_port: DeleteFromStoragePanicIfCalled,
        clock: FixedClock,
    });

    let uid = user_id("u1");
    let scope = persona_scope(uid.clone(), persona_id("p1"));
    let file_id = service
        .create_file(create_command_png(scope, &[1, 2, 3]), uid)
        .await
        .unwrap();

    assert_eq!(file_id.inner(), "f-good");
}

#[tokio::test]
async fn create_file_compensation_calls_delete_after_db_fail() {
    let recorder = RecordingDeleteStorage::default();
    let service = CreateFileService::new(CreateFilePrerequisites {
        id_generator_port: FixedFileId("f-db-fail"),
        save_file_to_storage_port: SaveStorageOk,
        save_file_port: SaveFileAlwaysErr,
        resolve_filename_port: ResolveFilenameEcho,
        load_file_create_access_context_port: LoadCreateContextAllow,
        load_scope_policy_port: LoadPolicyFixed {
            policy: sample_policy_png_max_1kb(),
        },
        delete_file_from_storage_port: recorder.clone(),
        clock: FixedClock,
    });

    let uid = user_id("u1");
    let scope = persona_scope(uid.clone(), persona_id("p1"));

    assert!(matches!(
        service
            .create_file(create_command_png(scope, &[1]), uid.clone())
            .await,
        Err(CreateFileError::InternalError(_))
    ));

    let deleted = recorder.deleted_meta_ids.lock().unwrap();
    assert_eq!(deleted.len(), 1);
    assert_eq!(deleted[0].inner(), "f-db-fail");
}

#[tokio::test]
async fn create_file_compensation_delete_fails_leaves_blob_in_storage() {
    let store = InMemoryBlobStore::default();
    let file_id = FileId::try_new("f-orphan".to_string()).unwrap();

    let service = CreateFileService::new(CreateFilePrerequisites {
        id_generator_port: FixedFileId("f-orphan"),
        save_file_to_storage_port: SaveToInMemoryBlobStore {
            store: store.clone(),
        },
        save_file_port: SaveFileAlwaysErr,
        resolve_filename_port: ResolveFilenameEcho,
        load_file_create_access_context_port: LoadCreateContextAllow,
        load_scope_policy_port: LoadPolicyFixed {
            policy: sample_policy_png_max_1kb(),
        },
        delete_file_from_storage_port: DeleteFromInMemoryBlobStoreFails {
            store: store.clone(),
        },
        clock: FixedClock,
    });

    let uid = user_id("u1");
    let scope = persona_scope(uid.clone(), persona_id("p1"));

    assert!(matches!(
        service
            .create_file(create_command_png(scope, &[1, 2, 3]), uid)
            .await,
        Err(CreateFileError::InternalError(_))
    ));

    assert_eq!(*store.delete_attempts.lock().unwrap(), 1);
    assert!(store.blob_ids.lock().unwrap().contains(&file_id));
}

// --- Delete / load meta store ---

type MetaDb = Arc<Mutex<HashMap<FileId, FileMeta>>>;

#[derive(Clone)]
struct LoadFileFromMap {
    db: MetaDb,
}

impl LoadFilePort for LoadFileFromMap {
    async fn load_file(&self, id: FileId) -> Result<FileMeta, LoadFilePortError> {
        self.db
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(LoadFilePortError::FileNotFound)
    }
}

#[derive(Clone)]
struct DeleteFileFromDb {
    db: MetaDb,
}

impl DeleteFilePort for DeleteFileFromDb {
    async fn delete_file(&self, id: FileId) -> Result<(), DeleteFilePortError> {
        self.db.lock().unwrap().remove(&id);
        Ok(())
    }
}

#[derive(Clone, Default)]
struct DeleteFromStorageOk;

impl DeleteFileFromStoragePort for DeleteFromStorageOk {
    async fn delete_file_from_storage(
        &self,
        _meta: &FileMeta,
    ) -> Result<(), DeleteFileFromStoragePortError> {
        Ok(())
    }
}

#[derive(Clone)]
struct LoadMetaContextAllow;

impl LoadFileMetaAccessContextPort for LoadMetaContextAllow {
    async fn load_meta_access_context(
        &self,
        _meta: &FileMeta,
        _user_id: &UserId,
    ) -> Result<FileMetaAccessContext, LoadFileMetaAccessContextPortError> {
        Ok(FileMetaAccessContext {
            persona_owned_by_actor: true,
        })
    }
}

fn fixture_meta(fid: &'static str, uid: UserId, scope: FileScope) -> FileMeta {
    let id = FileId::try_new(fid.to_string()).unwrap();
    FileMeta {
        id,
        filename: Filename::try_new("fixture.bin".to_string()).unwrap(),
        uploaded_by: uid,
        scope,
        original_name: FileOriginalName::try_new("o.png".to_string()).unwrap(),
        content_type: FileContentType::try_new("image/png".to_string()).unwrap(),
        size: FileSize::try_new(1).unwrap(),
        status: FileStatus::Activated,
        uploaded_at: UploadedAt::try_new(FIXED_TIMESTAMP).unwrap(),
    }
}

#[tokio::test]
async fn delete_file_ok() {
    let db: MetaDb = Arc::new(Mutex::new(HashMap::new()));
    let fid = FileId::try_new("del-1".to_string()).unwrap();
    let uid = user_id("u1");
    let scope = persona_scope(uid.clone(), persona_id("p1"));
    db.lock().unwrap().insert(
        fid.clone(),
        fixture_meta("del-1", uid.clone(), scope.clone()),
    );

    let service = DeleteFileService::new(DeleteFilePrerequisites {
        delete_file_port: DeleteFileFromDb { db: db.clone() },
        delete_file_from_storage_port: DeleteFromStorageOk,
        load_file_port: LoadFileFromMap { db: db.clone() },
        load_file_meta_access_context_port: LoadMetaContextAllow,
    });

    service
        .delete_file(DeleteFileCommand { id: fid.clone() }, uid.clone())
        .await
        .unwrap();

    assert!(!db.lock().unwrap().contains_key(&fid));
}

#[tokio::test]
async fn delete_file_access_denied() {
    let db: MetaDb = Arc::new(Mutex::new(HashMap::new()));
    let fid = FileId::try_new("del-den".to_string()).unwrap();
    let uid = user_id("u1");
    let scope = persona_scope(uid.clone(), persona_id("p1"));
    db.lock()
        .unwrap()
        .insert(fid.clone(), fixture_meta("del-den", uid.clone(), scope));

    let service = DeleteFileService::new(DeleteFilePrerequisites {
        delete_file_port: DeleteFileFromDb { db: db.clone() },
        delete_file_from_storage_port: DeleteFromStorageOk,
        load_file_port: LoadFileFromMap { db },
        load_file_meta_access_context_port: LoadMetaContextAllow,
    });

    assert!(matches!(
        service
            .delete_file(DeleteFileCommand { id: fid.clone() }, user_id("other"))
            .await,
        Err(DeleteFileError::AccessDenied)
    ));
}

#[tokio::test]
async fn delete_file_not_found_meta() {
    let db: MetaDb = Arc::new(Mutex::new(HashMap::new()));
    let fid = FileId::try_new("missing".to_string()).unwrap();

    let service = DeleteFileService::new(DeleteFilePrerequisites {
        delete_file_port: DeleteFileFromDb { db: db.clone() },
        delete_file_from_storage_port: DeleteFromStorageOk,
        load_file_port: LoadFileFromMap { db },
        load_file_meta_access_context_port: LoadMetaContextAllow,
    });

    assert!(matches!(
        service
            .delete_file(DeleteFileCommand { id: fid }, user_id("u1"))
            .await,
        Err(DeleteFileError::FileNotFound)
    ));
}

// --- Get ---

#[derive(Clone)]
struct LoadStorageFromMap {
    bytes: Arc<Mutex<HashMap<FileId, Vec<u8>>>>,
}

impl LoadFileFromStoragePort for LoadStorageFromMap {
    async fn load_file_from_storage(
        &self,
        meta: &FileMeta,
    ) -> Result<FileContent, LoadFileFromStoragePortError> {
        let map = self.bytes.lock().unwrap();
        map.get(&meta.id)
            .map(|v| FileContent::new(v.clone()))
            .ok_or(LoadFileFromStoragePortError::FileNotFound)
    }
}

#[tokio::test]
async fn get_file_returns_bytes() {
    let db: MetaDb = Arc::new(Mutex::new(HashMap::new()));
    let bytes_map: Arc<Mutex<HashMap<FileId, Vec<u8>>>> = Arc::new(Mutex::new(HashMap::new()));

    let fid = FileId::try_new("g-1".to_string()).unwrap();
    let uid = user_id("u1");
    let scope = persona_scope(uid.clone(), persona_id("p1"));
    let meta = fixture_meta("g-1", uid.clone(), scope.clone());
    db.lock().unwrap().insert(fid.clone(), meta);
    bytes_map
        .lock()
        .unwrap()
        .insert(fid.clone(), vec![10u8, 20, 30]);

    let service = GetFileService::new(GetFilePrerequisites {
        load_file_from_storage_port: LoadStorageFromMap { bytes: bytes_map },
        load_file_port: LoadFileFromMap { db },
        load_file_meta_access_context_port: LoadMetaContextAllow,
    });

    let content = service.get_file(fid, uid.clone()).await.unwrap();
    assert_eq!(content.inner().as_slice(), &[10, 20, 30]);
}

#[tokio::test]
async fn get_file_access_denied_after_meta_load() {
    let db: MetaDb = Arc::new(Mutex::new(HashMap::new()));
    let bytes_map: Arc<Mutex<HashMap<FileId, Vec<u8>>>> = Arc::default();

    let fid = FileId::try_new("g-den".to_string()).unwrap();
    let uid = user_id("u1");
    let scope = persona_scope(uid.clone(), persona_id("p1"));
    db.lock()
        .unwrap()
        .insert(fid.clone(), fixture_meta("g-den", uid, scope.clone()));
    bytes_map.lock().unwrap().insert(fid.clone(), vec![7]);

    let service = GetFileService::new(GetFilePrerequisites {
        load_file_from_storage_port: LoadStorageFromMap { bytes: bytes_map },
        load_file_port: LoadFileFromMap { db },
        load_file_meta_access_context_port: LoadMetaContextAllow,
    });

    assert!(matches!(
        service.get_file(fid, user_id("intruder")).await,
        Err(GetFileQueryError::AccessDenied)
    ));
}
