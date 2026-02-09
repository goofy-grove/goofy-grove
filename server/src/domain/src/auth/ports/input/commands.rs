use crate::{
    generate_entity,
    prelude::{Secret, UserName},
};

generate_entity!(AuthorizationCommand {
    name: UserName,
    secret: Secret
});
generate_entity!(RegistrationCommand {
    name: UserName,
    secret: Secret
});
