use crate::prelude::{UserName, UserPassword};

pub struct AuthorizeUserCommand {
    name: UserName,
    password: UserPassword,
}

impl AuthorizeUserCommand {
    pub fn new(name: UserName, password: UserPassword) -> Self {
        AuthorizeUserCommand { name, password }
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn password(&self) -> &UserPassword {
        &self.password
    }
}
