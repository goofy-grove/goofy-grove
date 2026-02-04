use crate::{
    error::DomainError,
    validator::{DomainValidationResult, Validator},
};

pub type UserId = String;
pub type UserName = String;
pub type UserPassword = String;

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    name: UserName,
    password: UserPassword,
}

impl User {
    pub fn new(id: UserId, name: UserName, password: UserPassword) -> Self {
        User { id, name, password }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn password(&self) -> &UserPassword {
        &self.password
    }
}

impl Validator for User {
    fn validate(&self) -> DomainValidationResult {
        if self.id.is_empty() {
            return Err(DomainError::IdValidationError(
                "user_id_is_empty".to_owned(),
            ));
        }

        if self.name.is_empty() {
            return Err(DomainError::NameValidationError(
                "user_name_is_empty".to_owned(),
            ));
        }

        if self.password.is_empty() {
            return Err(DomainError::PasswordValidationError(
                "user_password_is_empty".to_owned(),
            ));
        }

        Ok(())
    }
}
