use crate::{
    error::{DomainValidationError, DomainValidationResult}, generate_entity, impl_as_domain_newtype, validator::Validator
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserName(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserPassword(String);

impl_as_domain_newtype!(UserId -> String, UserName -> String, UserPassword -> String);
generate_entity!(User { uid: UserId, name: UserName, password: UserPassword });

impl Validator for User {
    fn validate(&self) -> DomainValidationResult {
        if self.uid.value().is_empty() {
            return Err(DomainValidationError::IdValidationError(
                "user_id_is_empty".to_owned(),
            ));
        }

        if self.name.value().is_empty() {
            return Err(DomainValidationError::NameValidationError(
                "user_name_is_empty".to_owned(),
            ));
        }

        if self.password.value().is_empty() {
            return Err(DomainValidationError::PasswordValidationError(
                "user_password_is_empty".to_owned(),
            ));
        }

        Ok(())
    }
}
