use crate::domain::prelude::*;

pub trait LoadPersonsPort {
    fn load_persons(&self) -> impl Future<Output = Vec<Person>>;
}

pub trait SavePersonPort {
    fn save_person(&self, person: Person) -> impl Future<Output = Person>;
}
