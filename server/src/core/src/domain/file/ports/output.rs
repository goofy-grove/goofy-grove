use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum SaveFileToStoragePortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait SaveFileToStoragePort {
    fn save_file_to_storage(
        &self,
        meta: &FileMeta,
        content: FileContent,
    ) -> impl Future<Output = Result<(), SaveFileToStoragePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum SaveFilePortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait SaveFilePort {
    fn save_file(&self, meta: FileMeta) -> impl Future<Output = Result<FileId, SaveFilePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum DeleteFilePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("File not found")]
    FileNotFound,
}

pub trait DeleteFilePort {
    fn delete_file(&self, id: FileId) -> impl Future<Output = Result<(), DeleteFilePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum DeleteFileFromStoragePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("File not found")]
    FileNotFound,
}

pub trait DeleteFileFromStoragePort {
    fn delete_file_from_storage(
        &self,
        meta: &FileMeta,
    ) -> impl Future<Output = Result<(), DeleteFileFromStoragePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum LoadFileFromStoragePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("File not found")]
    FileNotFound,
}

pub trait LoadFileFromStoragePort {
    fn load_file_from_storage(
        &self,
        meta: &FileMeta,
    ) -> impl Future<Output = Result<FileContent, LoadFileFromStoragePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum LoadFilePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("File not found")]
    FileNotFound,
}

pub trait LoadFilePort {
    fn load_file(&self, id: FileId) -> impl Future<Output = Result<FileMeta, LoadFilePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum LoadScopePolicyPortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Policy not found for scope")]
    PolicyForScopeNotFound,
}

pub trait LoadScopePolicyPort {
    fn load_scope_policy(
        &self,
        scope: &FileScope,
    ) -> impl Future<Output = Result<FilePolicy, LoadScopePolicyPortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum ResolveFilenamePortError {
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait ResolveFilenamePort {
    fn resolve_filename(
        &self,
        file_id: &FileId,
        original_name: &FileOriginalName,
    ) -> impl Future<Output = Result<Filename, ResolveFilenamePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum EnsureFileReadPortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Access denied")]
    AccessDenied,
}

pub trait EnsureFileReadPort {
    fn ensure_file_read(
        &self,
        meta: &FileMeta,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), EnsureFileReadPortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum EnsureFileCreatePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Access denied")]
    AccessDenied,
}

pub trait EnsureFileCreatePort {
    fn ensure_file_create(
        &self,
        scope: &FileScope,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), EnsureFileCreatePortError>>;
}

#[derive(Debug, Clone, Error)]
pub enum EnsureFileDeletePortError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Access denied")]
    AccessDenied,
}

pub trait EnsureFileDeletePort {
    fn ensure_file_delete(
        &self,
        meta: &FileMeta,
        user_id: &UserId,
    ) -> impl Future<Output = Result<(), EnsureFileDeletePortError>>;
}
