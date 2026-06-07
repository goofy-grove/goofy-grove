use thiserror::Error;

use crate::platform::config::FilePolicyConfig;

#[derive(Debug, Clone, Error)]
pub enum PolicyViolationError {
    #[error("Invalid file size: max {max_size}, got {size}")]
    InvalidFileSize { max_size: usize, size: usize },

    #[error("Invalid content type: allowed {allowed:?}, got {content_type}")]
    InvalidContentType {
        allowed: Vec<String>,
        content_type: String,
    },
}

pub fn assert_file_matches_policy(
    size: usize,
    content_type: &str,
    policy: &FilePolicyConfig,
) -> Result<(), PolicyViolationError> {
    let max_size_bytes = policy.max_file_size.to_bytes() as usize;

    if size > max_size_bytes {
        return Err(PolicyViolationError::InvalidFileSize {
            max_size: max_size_bytes,
            size,
        });
    }

    if !policy.allowed_content_types.is_empty()
        && !policy
            .allowed_content_types
            .iter()
            .any(|allowed| allowed == content_type)
    {
        return Err(PolicyViolationError::InvalidContentType {
            allowed: policy.allowed_content_types.clone(),
            content_type: content_type.to_string(),
        });
    }

    Ok(())
}
