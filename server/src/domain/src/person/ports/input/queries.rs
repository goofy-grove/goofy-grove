use crate::{error::DomainResult, prelude::Person};

pub trait GetPersonsQuery {
    fn get_persons(&self) -> impl Future<Output = DomainResult<Vec<Person>, ()>>;
}
