use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub enum FileScope {
    PersonaAvatar {
        user_id: UserId,
        persona_id: PersonaId,
    },
    UserAvatar {
        user_id: UserId,
    },
}
