use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub enum GetPersonsErorr {
    InternalError(String),
}

pub trait GetPersonsQuery {
    fn get_persons(&self) -> impl Future<Output = DomainResult<Vec<Person>, GetPersonsErorr>>;
}
