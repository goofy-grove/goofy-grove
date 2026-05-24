use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct AuthorizationCommand {
    pub name: Username,
    pub secret: Secret,
}

#[derive(Debug, Clone)]
pub struct RegistrationCommand {
    pub name: Username,
    pub secret: Secret,
}

#[derive(Debug, Clone)]
pub struct ValidateTokenCommand {
    pub first_token: Token,
    pub secret: Secret,
}
