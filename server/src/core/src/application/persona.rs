use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

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

#[derive(Debug, Clone)]
pub struct PersonaUpdateService<L: LoadPersonaPort, S: SavePersonaPort, E: EventPublisher> {
    load_persona_port: L,
    save_persona_port: S,
    event_publisher: E,
}

#[derive(Debug, Clone)]
pub struct PersonaDeleteService<L: LoadPersonaPort, D: DeletePersonaPort, E: EventPublisher> {
    load_persona_port: L,
    delete_persona_port: D,
    event_publisher: E,
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
    ) -> Result<Persona, CreatePersonaError> {
        let persona = Persona::new(
            PersonaId::try_new(self.uid_generator.generate())
                .map_err(|err| CreatePersonaError::ValidationError(format!("{err}")))?,
            command.creator_id().clone(),
            command.name().clone(),
            command.description().clone(),
        );

        match self.save_persona_port.save_persona(persona).await {
            Ok(saved_persona) => {
                self.event_publisher
                    .publish(PersonaCreatedEvent {
                        persona: saved_persona.clone(),
                        exclude_participants: command.exclude_participants().clone(),
                    })
                    .await;

                Ok(saved_persona)
            }
            Err(err) => Err(CreatePersonaError::InternalError(format!("{:?}", err))),
        }
    }
}

impl<L: LoadPersonasPort> GetPersonasService<L> {
    pub fn new(load_personas_port: L) -> Self {
        Self { load_personas_port }
    }
}

impl<L: LoadPersonaPort, S: SavePersonaPort, E: EventPublisher> PersonaUpdateService<L, S, E> {
    pub fn new(load_persona_port: L, save_persona_port: S, event_publisher: E) -> Self {
        Self {
            load_persona_port,
            save_persona_port,
            event_publisher,
        }
    }
}

impl<L: LoadPersonaPort, D: DeletePersonaPort, E: EventPublisher> PersonaDeleteService<L, D, E> {
    pub fn new(load_persona_port: L, delete_persona_port: D, event_publisher: E) -> Self {
        Self {
            load_persona_port,
            delete_persona_port,
            event_publisher,
        }
    }
}

impl<L: LoadPersonasPort> GetPersonasQuery for GetPersonasService<L> {
    async fn get_personas(&self, user_id: &UserId) -> Result<Vec<Persona>, GetPersonasError> {
        self.load_personas_port
            .load_personas(user_id)
            .await
            .map_err(|err| GetPersonasError::InternalError(err.to_string()))
    }
}

impl<L: LoadPersonaPort, S: SavePersonaPort, E: EventPublisher> UpdatePersonaUseCase
    for PersonaUpdateService<L, S, E>
{
    async fn update_persona(
        &self,
        command: UpdatePersonaCommand,
        user_id: UserId,
    ) -> Result<Persona, UpdatePersonaError> {
        let persona = self
            .load_persona_port
            .load_persona(command.id(), &user_id)
            .await
            .map_err(|err| match err {
                LoadPersonasPortError::NotFound => UpdatePersonaError::NotFound,
                LoadPersonasPortError::InternalError(message) => {
                    UpdatePersonaError::InternalError(message)
                }
            })?;

        let updated_persona = Persona::new(
            persona.uid().clone(),
            persona.creator_id().clone(),
            command
                .name()
                .clone()
                .unwrap_or_else(|| persona.name().clone()),
            command
                .description()
                .clone()
                .unwrap_or_else(|| persona.description().clone()),
        );

        let saved_persona = self
            .save_persona_port
            .save_persona(updated_persona)
            .await
            .map_err(|err| UpdatePersonaError::InternalError(err.to_string()))?;

        self.event_publisher
            .publish(PersonaUpdatedEvent {
                persona: saved_persona.clone(),
                exclude_participants: command.exclude_participants().clone(),
            })
            .await;

        Ok(saved_persona)
    }
}

impl<L: LoadPersonaPort, D: DeletePersonaPort, E: EventPublisher> DeletePersonaUseCase
    for PersonaDeleteService<L, D, E>
{
    async fn delete_persona(
        &self,
        command: DeletePersonaCommand,
        user_id: UserId,
    ) -> Result<(), DeletePersonaError> {
        let persona = self
            .load_persona_port
            .load_persona(command.id(), &user_id)
            .await
            .map_err(|err| match err {
                LoadPersonasPortError::NotFound => DeletePersonaError::NotFound,
                LoadPersonasPortError::InternalError(message) => {
                    DeletePersonaError::InternalError(message)
                }
            })?;

        self.delete_persona_port
            .delete_persona(command.id(), &user_id)
            .await
            .map_err(|err| match err {
                DeletePersonaPortError::NotFound => DeletePersonaError::NotFound,
                DeletePersonaPortError::InternalError(message) => {
                    DeletePersonaError::InternalError(message)
                }
            })?;

        self.event_publisher
            .publish(PersonaDeletedEvent {
                persona,
                exclude_participants: command.exclude_participants().clone(),
            })
            .await;

        Ok(())
    }
}
