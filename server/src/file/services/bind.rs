use crate::file::db::file::{FileMeta, FileStatus};

pub fn can_serve_file(meta: &FileMeta) -> bool {
    matches!(meta.status, FileStatus::Created | FileStatus::Activated)
}
