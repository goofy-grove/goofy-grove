mod api;
mod db;
mod services;

pub use api::mount;
pub use db::file::{FileMeta, FileScope};
pub use services::{
    avatar::{
        ApplyAvatarPatchError, OrphanAvatarError, apply_avatar_uid_patch, orphan_avatar_if_present,
    },
    create::{CreateFileInput, create_file as create_file_for_user},
};
