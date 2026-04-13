use crate::{domain::prelude::*, generate_entity, impl_as_domain_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonaName(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonaDescription(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonaId(String);

impl_as_domain_newtype!(PersonaId -> String, PersonaName -> String, PersonaDescription -> String);

generate_entity!(Persona {
    uid: PersonaId,
    creator_id: UserId,
    name: PersonaName,
    description: PersonaDescription
});

impl Validator for Persona {
    fn validate(&self) -> DomainValidationResult {
        if self.uid.value().is_empty() {
            DomainValidationResult::Err(DomainValidationError::IdValidationError(
                "persona_id_is_empty".to_owned(),
            ))
        } else if self.name.value().is_empty() {
            DomainValidationResult::Err(DomainValidationError::NameValidationError(
                "persona_name_is_empty".to_owned(),
            ))
        } else {
            Ok(())
        }
    }
}
