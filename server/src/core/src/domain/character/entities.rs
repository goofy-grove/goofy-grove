use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Character {
    pub uid: CharacterId,
    pub creator_id: UserId,
    pub name: CharacterName,
    pub description: CharacterDescription,
}
