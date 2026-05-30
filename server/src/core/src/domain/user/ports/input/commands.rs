use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct UpdateUserCommand {
    pub avatar_uid: PatchField<FileId>,
    pub exclude_participants: Vec<ParticipantId>,
}
