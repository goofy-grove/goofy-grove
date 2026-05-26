use thiserror::Error;

use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Error)]
pub enum FilePolicyViolationError {
    #[error("Invalid file size max_size: {}, size: {}", max_size.inner(), size.inner())]
    InvalidFileSize { max_size: FileSize, size: FileSize },

    #[error(
        "Invalid content type allowed_content_types: {:?}, content_type: {}",
        allowed_content_types.iter().map(|t| t.inner()).collect::<Vec<_>>(),
        content_type.inner()
    )]
    InvalidContentType {
        allowed_content_types: Vec<FileContentType>,
        content_type: FileContentType,
    },
}

pub fn assert_file_matches_policy(
    size: &FileSize,
    content_type: &FileContentType,
    policy: FilePolicy,
) -> Result<(), FilePolicyViolationError> {
    if *size.inner() > *policy.max_size.inner() {
        return Err(FilePolicyViolationError::InvalidFileSize {
            max_size: policy.max_size,
            size: size.clone(),
        });
    }

    if !policy.allowed_content_types.is_empty() && !policy.allowed_content_types.contains(&content_type) {
        return Err(FilePolicyViolationError::InvalidContentType {
            allowed_content_types: policy.allowed_content_types,
            content_type: content_type.clone(),
        });
    }

    Ok(())
}
