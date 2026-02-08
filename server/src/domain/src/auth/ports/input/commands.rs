use crate::prelude::{Secret, UserName};

pub struct AuthorizeUserCommand {
    name: UserName,
    secret: Secret,
}

impl AuthorizeUserCommand {
    pub fn new(name: UserName, secret: Secret) -> Self {
        AuthorizeUserCommand { name, secret }
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn secret(&self) -> &Secret {
        &self.secret
    }
}
