use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::*;
use crate::domain::prelude::*;

type FileDb = Arc<Mutex<HashMap<FileId, FileMeta>>>;

#[derive(Clone)]
struct TrackingPorts {
    db: FileDb,
    activated: Arc<Mutex<Vec<FileId>>>,
    orphaned: Arc<Mutex<Vec<FileId>>>,
}

impl TrackingPorts {
    fn new(entries: HashMap<FileId, FileMeta>) -> Self {
        Self {
            db: Arc::new(Mutex::new(entries)),
            activated: Arc::new(Mutex::new(Vec::new())),
            orphaned: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl LoadFilePort for TrackingPorts {
    async fn load_file(&self, id: FileId) -> Result<FileMeta, LoadFilePortError> {
        self.db
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(LoadFilePortError::FileNotFound)
    }
}

impl ActivateFilePort for TrackingPorts {
    async fn activate_file(&self, meta: &FileMeta) -> Result<(), ActivateFilePortError> {
        self.activated.lock().unwrap().push(meta.id.clone());
        if let Some(stored) = self.db.lock().unwrap().get_mut(&meta.id) {
            stored.status = FileStatus::Activated;
        }
        Ok(())
    }
}

impl OrphanFilePort for TrackingPorts {
    async fn orphan_file(&self, meta: &FileMeta) -> Result<(), OrphanFilePortError> {
        self.orphaned.lock().unwrap().push(meta.id.clone());
        if let Some(stored) = self.db.lock().unwrap().get_mut(&meta.id) {
            stored.status = FileStatus::Orphaned;
        }
        Ok(())
    }
}

fn user_id(value: &str) -> UserId {
    UserId::try_new(value.to_string()).unwrap()
}

fn file_id(value: &str) -> FileId {
    FileId::try_new(value.to_string()).unwrap()
}

fn user_avatar_scope(user_id: UserId) -> FileScope {
    FileScope::UserAvatar { user_id }
}

fn fixture_meta(id: &str, scope: FileScope, status: FileStatus) -> FileMeta {
    FileMeta {
        id: file_id(id),
        filename: Filename::try_new("avatar.png".to_string()).unwrap(),
        uploaded_by: user_id("user-1"),
        scope,
        original_name: FileOriginalName::try_new("avatar.png".to_string()).unwrap(),
        content_type: FileContentType::try_new("image/png".to_string()).unwrap(),
        size: FileSize::try_new(128).unwrap(),
        status,
        uploaded_at: UploadedAt::try_new(1).unwrap(),
    }
}

#[tokio::test]
async fn apply_avatar_uid_patch_absent_leaves_current_unchanged() {
    let scope = user_avatar_scope(user_id("user-1"));
    let current = Some(file_id("old"));
    let ports = TrackingPorts::new(HashMap::from([(
        file_id("old"),
        fixture_meta("old", scope.clone(), FileStatus::Activated),
    )]));

    let result = apply_avatar_uid_patch(
        &ports,
        &ports,
        &ports,
        current.clone(),
        PatchField::Unchanged,
        &scope,
    )
    .await
    .unwrap();

    assert_eq!(result, current);
    assert!(ports.activated.lock().unwrap().is_empty());
    assert!(ports.orphaned.lock().unwrap().is_empty());
}

#[tokio::test]
async fn apply_avatar_uid_patch_null_orphans_current() {
    let scope = user_avatar_scope(user_id("user-1"));
    let old_id = file_id("old");
    let ports = TrackingPorts::new(HashMap::from([(
        old_id.clone(),
        fixture_meta("old", scope.clone(), FileStatus::Activated),
    )]));

    let result = apply_avatar_uid_patch(
        &ports,
        &ports,
        &ports,
        Some(old_id.clone()),
        PatchField::Clear,
        &scope,
    )
    .await
    .unwrap();

    assert_eq!(result, None);
    assert_eq!(&*ports.orphaned.lock().unwrap(), &vec![old_id]);
    assert!(ports.activated.lock().unwrap().is_empty());
}

#[tokio::test]
async fn apply_avatar_uid_patch_new_file_activates_and_orphans_previous() {
    let scope = user_avatar_scope(user_id("user-1"));
    let old_id = file_id("old");
    let new_id = file_id("new");
    let ports = TrackingPorts::new(HashMap::from([
        (
            old_id.clone(),
            fixture_meta("old", scope.clone(), FileStatus::Activated),
        ),
        (
            new_id.clone(),
            fixture_meta("new", scope.clone(), FileStatus::Created),
        ),
    ]));

    let result = apply_avatar_uid_patch(
        &ports,
        &ports,
        &ports,
        Some(old_id.clone()),
        PatchField::Set(new_id.clone()),
        &scope,
    )
    .await
    .unwrap();

    assert_eq!(result, Some(new_id.clone()));
    assert_eq!(&*ports.activated.lock().unwrap(), &vec![new_id]);
    assert_eq!(&*ports.orphaned.lock().unwrap(), &vec![old_id]);
}

#[tokio::test]
async fn apply_avatar_uid_patch_same_id_is_noop() {
    let scope = user_avatar_scope(user_id("user-1"));
    let current_id = file_id("same");
    let ports = TrackingPorts::new(HashMap::from([(
        current_id.clone(),
        fixture_meta("same", scope.clone(), FileStatus::Activated),
    )]));

    let result = apply_avatar_uid_patch(
        &ports,
        &ports,
        &ports,
        Some(current_id.clone()),
        PatchField::Set(current_id.clone()),
        &scope,
    )
    .await
    .unwrap();

    assert_eq!(result, Some(current_id));
    assert!(ports.activated.lock().unwrap().is_empty());
    assert!(ports.orphaned.lock().unwrap().is_empty());
}

#[tokio::test]
async fn apply_avatar_uid_patch_rejects_non_created_file() {
    let scope = user_avatar_scope(user_id("user-1"));
    let activated_id = file_id("activated");
    let ports = TrackingPorts::new(HashMap::from([(
        activated_id.clone(),
        fixture_meta("activated", scope.clone(), FileStatus::Activated),
    )]));

    let err = apply_avatar_uid_patch(
        &ports,
        &ports,
        &ports,
        None,
        PatchField::Set(activated_id),
        &scope,
    )
    .await
    .unwrap_err();

    assert!(matches!(err, AvatarBindingError::ValidationError(_)));
}

#[tokio::test]
async fn orphan_avatar_if_present_orphans_file() {
    let scope = user_avatar_scope(user_id("user-1"));
    let avatar_id = file_id("avatar");
    let ports = TrackingPorts::new(HashMap::from([(
        avatar_id.clone(),
        fixture_meta("avatar", scope, FileStatus::Activated),
    )]));

    orphan_avatar_if_present(&ports, &ports, Some(avatar_id.clone()))
        .await
        .unwrap();

    assert_eq!(&*ports.orphaned.lock().unwrap(), &vec![avatar_id]);
}

#[tokio::test]
async fn orphan_avatar_if_present_without_avatar_is_noop() {
    let ports = TrackingPorts::new(HashMap::new());

    orphan_avatar_if_present(&ports, &ports, None)
        .await
        .unwrap();

    assert!(ports.orphaned.lock().unwrap().is_empty());
}
