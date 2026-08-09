use serde_json::json;

use crate::{
    file::services::{
        avatar::ReplaceAvatarError, create::CreateFileError, policy::PolicyViolationError,
    },
    platform::http::error::{ApiError, codes},
};

impl From<PolicyViolationError> for ApiError {
    fn from(err: PolicyViolationError) -> Self {
        match err {
            PolicyViolationError::InvalidFileSize { max_size, size } => {
                ApiError::bad_request(codes::FILE_INVALID_SIZE)
                    .with_params(json!({ "max_size": max_size, "size": size }))
            }
            PolicyViolationError::InvalidContentType {
                allowed,
                content_type,
            } => ApiError::bad_request(codes::FILE_INVALID_CONTENT_TYPE).with_params(json!({
                "allowed": allowed,
                "content_type": content_type,
            })),
        }
    }
}

impl From<CreateFileError> for ApiError {
    fn from(err: CreateFileError) -> Self {
        match err {
            CreateFileError::AccessDenied => ApiError::forbidden(codes::FILE_ACCESS_DENIED),
            CreateFileError::PolicyViolation(violation) => violation.into(),
            CreateFileError::PolicyForScopeNotFound => {
                ApiError::internal(codes::FILE_UPLOAD_POLICY_NOT_CONFIGURED)
            }
            CreateFileError::InternalError(_) => ApiError::internal(codes::FILE_UPLOAD_FAILED),
        }
    }
}

impl From<ReplaceAvatarError> for ApiError {
    fn from(err: ReplaceAvatarError) -> Self {
        match err {
            ReplaceAvatarError::AccessDenied => ApiError::forbidden(codes::FILE_ACCESS_DENIED),
            ReplaceAvatarError::PolicyViolation(violation) => violation.into(),
            ReplaceAvatarError::PolicyForScopeNotFound => {
                ApiError::internal(codes::FILE_UPLOAD_POLICY_NOT_CONFIGURED)
            }
            ReplaceAvatarError::InternalError(_) => ApiError::internal(codes::FILE_UPLOAD_FAILED),
        }
    }
}
