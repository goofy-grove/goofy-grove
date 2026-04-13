use crate::{domain::prelude::*, generate_entity};

generate_entity!(CreatePersonaCommand {
    name: PersonaName,
    creator_id: UserId,
    description: PersonaDescription
});

generate_entity!(UpdatePersonaCommand {
    id: String,
    name: PersonaName,
    description: PersonaDescription
});
