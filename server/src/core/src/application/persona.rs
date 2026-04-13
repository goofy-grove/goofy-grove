use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct PersonaCreateService<S: SavePersonaPort, U: IdGenerator, E: EventPublisher> {
    save_persona_port: S,
    uid_generator: U,
    event_publisher: E,
}

#[derive(Debug, Clone)]
pub struct GetPersonasService<L: LoadPersonasPort> {
    load_personas_port: L,
}

impl<S: SavePersonaPort, U: IdGenerator, E: EventPublisher> PersonaCreateService<S, U, E> {
    pub fn new(save_persona_port: S, uid_generator: U, event_publisher: E) -> Self {
        Self {
            save_persona_port,
            uid_generator,
            event_publisher,
        }
    }
}

impl<S: SavePersonaPort, U: IdGenerator, E: EventPublisher> CreatePersonaUseCase
    for PersonaCreateService<S, U, E>
{
    async fn create_persona(
        &self,
        command: CreatePersonaCommand,
    ) -> DomainResult<Persona, CreatePersonaError> {
        let persona = Persona::new(
            PersonaId::new(self.uid_generator.generate()),
            command.creator_id().clone(),
            command.name().clone(),
            command.description().clone(),
        );

        match self.save_persona_port.save_persona(persona).await {
            Ok(saved_persona) => {
                self.event_publisher
                    .publish(PersonaCreatedEvent {
                        persona: saved_persona.clone(),
                    })
                    .await;

                Ok(saved_persona)
            }
            Err(err) => Err(DomainError::UseCaseError(
                CreatePersonaError::InternalError(format!("{:?}", err)),
            )),
        }
    }
}

impl<L: LoadPersonasPort> GetPersonasService<L> {
    pub fn new(load_personas_port: L) -> Self {
        Self { load_personas_port }
    }
}

impl<L: LoadPersonasPort> GetPersonasQuery for GetPersonasService<L> {
    async fn get_personas(&self, user_id: &UserId) -> DomainResult<Vec<Persona>, GetPersonasErorr> {
        // TODO: add error propagation
        DomainResult::Ok(self.load_personas_port.load_personas(user_id).await)
    }
}
