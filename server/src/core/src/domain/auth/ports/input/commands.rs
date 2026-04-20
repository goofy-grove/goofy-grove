use crate::{domain::prelude::*, generate_entity};

generate_entity!(AuthorizationCommand {
    name: Username,
    secret: Secret
});
generate_entity!(RegistrationCommand {
    name: Username,
    secret: Secret
});

generate_entity!(ValidateTokenCommand {
    first_token: Token,
    secret: Secret
});
