use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct PersonCreateService<S: SavePersonPort, U: IdGenerator, E: EventPublisher> {
    save_person_port: S,
    uid_generator: U,
    event_publisher: E,
}

#[derive(Debug, Clone)]
pub struct GetPersonsService<L: LoadPersonsPort> {
    load_persons_port: L,
}

impl<S: SavePersonPort, U: IdGenerator, E: EventPublisher> PersonCreateService<S, U, E> {
    pub fn new(save_person_port: S, uid_generator: U, event_publisher: E) -> Self {
        Self {
            save_person_port,
            uid_generator,
            event_publisher,
        }
    }
}

impl<S: SavePersonPort, U: IdGenerator, E: EventPublisher> CreatePersonUseCase
    for PersonCreateService<S, U, E>
{
    async fn create_person(
        &self,
        command: CreatePersonCommand,
    ) -> DomainResult<Person, CreatePersonError> {
        let person = Person::new(
            PersonId::new(self.uid_generator.generate()),
            command.creator_id().clone(),
            command.name().clone(),
            command.description().clone(),
        );

        match self.save_person_port.save_person(person).await {
            Ok(saved_person) => {
                self.event_publisher
                    .publish(PersonCreatedEvent {
                        person: saved_person.clone(),
                    })
                    .await;

                Ok(saved_person)
            }
            Err(err) => Err(DomainError::UseCaseError(CreatePersonError::InternalError(
                format!("{:?}", err),
            ))),
        }
    }
}

impl<L: LoadPersonsPort> GetPersonsService<L> {
    pub fn new(load_persons_port: L) -> Self {
        Self { load_persons_port }
    }
}

impl<L: LoadPersonsPort> GetPersonsQuery for GetPersonsService<L> {
    async fn get_persons(&self, user_id: &UserId) -> DomainResult<Vec<Person>, GetPersonsErorr> {
        // TODO: add error propagation
        DomainResult::Ok(self.load_persons_port.load_persons(user_id).await)
    }
}
