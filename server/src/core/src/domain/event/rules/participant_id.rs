use thiserror::Error;

use crate::impl_new_type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum ParticipantIdValidationError {
    #[error("participant_id_empty")]
    Empty,
}

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ParticipantId(String);
    error: ParticipantIdValidationError;
    sanitize: |id: String| id.trim().to_owned();
    validate: |id: &str| {
        if id.is_empty() {
            Err(ParticipantIdValidationError::Empty)
        } else {
            Ok(())
        }
    };
);
