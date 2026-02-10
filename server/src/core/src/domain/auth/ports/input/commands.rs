use crate::{domain::prelude::*, generate_entity};

generate_entity!(AuthorizationCommand {
    name: UserName,
    secret: Secret
});
generate_entity!(RegistrationCommand {
    name: UserName,
    secret: Secret
});
