use crate::file::db::file::{FileMeta, FileScope, FileStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindAvatarCheck {
    Allowed,
    InvalidStatus,
    InvalidScope,
}

pub fn can_bind_file_as_avatar(meta: &FileMeta, expected_scope: &FileScope) -> BindAvatarCheck {
    if meta.status != FileStatus::Created {
        return BindAvatarCheck::InvalidStatus;
    }

    if &meta.scope != expected_scope {
        return BindAvatarCheck::InvalidScope;
    }

    BindAvatarCheck::Allowed
}

pub fn can_serve_file(meta: &FileMeta) -> bool {
    matches!(meta.status, FileStatus::Created | FileStatus::Activated)
}
