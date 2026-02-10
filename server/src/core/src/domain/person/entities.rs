use crate::{domain::prelude::*, generate_entity, impl_as_domain_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonName(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonDescription(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(String);

impl_as_domain_newtype!(PersonId -> String, PersonName -> String, PersonDescription -> String);

generate_entity!(Person {
    id: PersonId,
    name: PersonName,
    description: PersonDescription
});

impl Validator for Person {
    fn validate(&self) -> DomainValidationResult {
        if self.name.value().is_empty() {
            DomainValidationResult::Err(DomainValidationError::NameValidationError(
                "person_name_is_empty".to_owned(),
            ))
        } else {
            Ok(())
        }
    }
}
