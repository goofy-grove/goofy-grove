use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub enum CreatePersonError {
    InternalError(String),
}

pub trait CreatePersonUseCase {
    fn create_person(
        &self,
        command: CreatePersonCommand,
    ) -> impl Future<Output = DomainResult<Person, CreatePersonError>>;
}
