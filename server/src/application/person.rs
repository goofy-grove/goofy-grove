use domain::prelude::*;

#[derive(Debug, Clone)]
pub struct PersonCreateService<S: SavePersonPort, U: IdGenerator> {
    save_person_port: S,
    uid_generator: U,
}

#[derive(Debug, Clone)]
pub struct GetPersonsService<L: LoadPersonsPort> {
    load_persons_port: L,
}

impl<S: SavePersonPort, U: IdGenerator> PersonCreateService<S, U> {
    pub fn new(save_person_port: S, uid_generator: U) -> Self {
        Self {
            save_person_port,
            uid_generator,
        }
    }
}

impl<S: SavePersonPort, U: IdGenerator> CreatePersonUseCase for PersonCreateService<S, U> {
    async fn create_person(&self, command: CreatePersonCommand) -> DomainResult<Person, ()> {
        let person = Person::new(
            PersonId::new(self.uid_generator.generate()),
            command.name().clone(),
            command.description().clone(),
        );

        DomainResult::Ok(self.save_person_port.save_person(person).await)
    }
}

impl<L: LoadPersonsPort> GetPersonsService<L> {
    pub fn new(load_persons_port: L) -> Self {
        Self { load_persons_port }
    }
}

impl<L: LoadPersonsPort> GetPersonsQuery for GetPersonsService<L> {
    async fn get_persons(&self) -> DomainResult<Vec<Person>, ()> {
        DomainResult::Ok(self.load_persons_port.load_persons().await)
    }
}
