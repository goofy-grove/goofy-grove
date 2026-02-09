use crate::{
    generate_entity,
    prelude::{PersonDescription, PersonName},
};

generate_entity!(CreatePersonCommand {
    name: PersonName,
    description: PersonDescription
});

generate_entity!(UpdatePersonCommand {
    id: String,
    name: PersonName,
    description: PersonDescription
});
