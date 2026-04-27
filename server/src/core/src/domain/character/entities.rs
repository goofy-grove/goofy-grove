use crate::{domain::prelude::*, generate_entity};

#[cfg(test)]
mod tests;

generate_entity!(Character {
    uid: CharacterId,
    creator_id: UserId,
    name: CharacterName,
    description: CharacterDescription
});
