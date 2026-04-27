use crate::{domain::prelude::*, generate_entity};

generate_entity!(CreateCharacterCommand {
    name: CharacterName,
    creator_id: UserId,
    description: CharacterDescription,
    exclude_participants: Vec<ParticipantId>
});

generate_entity!(UpdateCharacterCommand {
    id: CharacterId,
    name: Option<CharacterName>,
    description: Option<CharacterDescription>,
    exclude_participants: Vec<ParticipantId>
});

generate_entity!(DeleteCharacterCommand {
    id: CharacterId,
    exclude_participants: Vec<ParticipantId>
});
