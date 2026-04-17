use crate::{domain::prelude::*, generate_entity};

generate_entity!(CreatePersonaCommand {
    name: PersonaName,
    creator_id: UserId,
    description: PersonaDescription,
    exclude_participants: Vec<ParticipantId>
});

generate_entity!(UpdatePersonaCommand {
    id: String,
    name: PersonaName,
    description: PersonaDescription
});
