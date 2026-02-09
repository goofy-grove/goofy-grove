use crate::prelude::Person;

pub trait LoadPersons {
    fn load_persons(&self) -> impl Future<Output = Vec<Person>>;
}

pub trait SavePerson {
    fn save_person(&self, person: Person) -> impl Future<Output = Person>;
}
