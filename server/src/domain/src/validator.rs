use crate::error::DomainValidationResult;

pub trait Validator {
    fn validate(&self) -> DomainValidationResult;
}
