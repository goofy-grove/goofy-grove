use crate::{domain::prelude::*, generate_entity};

generate_entity!(Persona {
    uid: PersonaId,
    creator_id: UserId,
    name: PersonaName,
    description: PersonaDescription
});
