use crate::prelude::{Secret, UserName};

#[derive(Debug, Clone)]
pub struct AuthorizationCommand {
    name: UserName,
    secret: Secret,
}

impl AuthorizationCommand {
    pub fn new(name: UserName, secret: Secret) -> Self {
        AuthorizationCommand { name, secret }
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn secret(&self) -> &Secret {
        &self.secret
    }
}

#[derive(Debug, Clone)]
pub struct RegistrationCommand {
    name: UserName,
    secret: Secret,
}

impl RegistrationCommand {
    pub fn new(name: UserName, secret: Secret) -> Self {
        RegistrationCommand { name, secret }
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn secret(&self) -> &Secret {
        &self.secret
    }
}
