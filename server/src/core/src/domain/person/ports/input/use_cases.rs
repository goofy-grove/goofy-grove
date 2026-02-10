use crate::domain::prelude::*;

pub trait CreatePersonUseCase {
    fn create_person(
        &self,
        command: CreatePersonCommand,
    ) -> impl Future<Output = DomainResult<Person, ()>>;
}
