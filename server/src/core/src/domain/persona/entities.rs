use crate::{domain::prelude::*, generate_entity};

#[cfg(test)]
mod tests;

generate_entity!(Persona {
    uid: PersonaId,
    creator_id: UserId,
    name: PersonaName,
    description: PersonaDescription
});
