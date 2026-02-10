use crate::domain::prelude::*;

pub trait GetPersonsQuery {
    fn get_persons(&self) -> impl Future<Output = DomainResult<Vec<Person>, ()>>;
}
