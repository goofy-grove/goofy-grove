use crate::{domain::prelude::*, generate_entity};

generate_entity!(CreatePersonCommand {
    name: PersonName,
    description: PersonDescription
});

generate_entity!(UpdatePersonCommand {
    id: String,
    name: PersonName,
    description: PersonDescription
});
