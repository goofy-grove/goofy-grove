use crate::{
    error::DomainResult,
    prelude::{CreatePersonCommand, Person},
};

pub trait CreatePersonUseCase {
    fn create_person(
        &self,
        command: CreatePersonCommand,
    ) -> impl Future<Output = DomainResult<Person, ()>>;
}
