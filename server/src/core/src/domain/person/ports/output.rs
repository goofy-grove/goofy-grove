use crate::domain::prelude::*;

pub trait LoadPersonsPort {
    fn load_persons(&self, user_id: UserId) -> impl Future<Output = Vec<Person>>;
}

#[derive(Debug, Clone)]
pub enum SavePersonPortError {
    InternalError(String),
}

pub trait SavePersonPort {
    fn save_person(
        &self,
        person: Person,
    ) -> impl Future<Output = DomainResult<Person, SavePersonPortError>>;
}
