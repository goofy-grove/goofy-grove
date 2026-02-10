use crate::domain::error::DomainValidationResult;

pub trait Validator {
    fn validate(&self) -> DomainValidationResult;
}
