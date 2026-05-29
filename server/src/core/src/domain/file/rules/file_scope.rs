use crate::domain::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileScope {
    PersonaAvatar {
        user_id: UserId,
        persona_id: PersonaId,
    },
    UserAvatar {
        user_id: UserId,
    },
}
