use crate::application::avatar::{
    AvatarBindingError, apply_avatar_uid_patch, orphan_avatar_if_present,
};
use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct CreatePersonaPrerequisites<S, U, E, LF, A, O> {
    pub save_persona_port: S,
    pub uid_generator: U,
    pub event_publisher: E,
    pub load_file_port: LF,
    pub activate_file_port: A,
    pub orphan_file_port: O,
}

#[derive(Debug, Clone)]
pub struct PersonaCreateService<
    S: SavePersonaPort,
    U: IdGenerator,
    E: EventPublisher,
    LF: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
> {
    save_persona_port: S,
    uid_generator: U,
    event_publisher: E,
    load_file_port: LF,
    activate_file_port: A,
    orphan_file_port: O,
}

#[derive(Debug, Clone)]
pub struct GetPersonasService<L: LoadPersonasPort> {
    load_personas_port: L,
}

#[derive(Debug, Clone)]
pub struct UpdatePersonaPrerequisites<L, S, E, LF, A, O> {
    pub load_persona_port: L,
    pub save_persona_port: S,
    pub event_publisher: E,
    pub load_file_port: LF,
    pub activate_file_port: A,
    pub orphan_file_port: O,
}

#[derive(Debug, Clone)]
pub struct DeletePersonaPrerequisites<L, D, E, LF, O> {
    pub load_persona_port: L,
    pub delete_persona_port: D,
    pub event_publisher: E,
    pub load_file_port: LF,
    pub orphan_file_port: O,
}

#[derive(Debug, Clone)]
pub struct PersonaUpdateService<
    L: LoadPersonaPort,
    S: SavePersonaPort,
    E: EventPublisher,
    LF: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
> {
    load_persona_port: L,
    save_persona_port: S,
    event_publisher: E,
    load_file_port: LF,
    activate_file_port: A,
    orphan_file_port: O,
}

#[derive(Debug, Clone)]
pub struct PersonaDeleteService<
    L: LoadPersonaPort,
    D: DeletePersonaPort,
    E: EventPublisher,
    LF: LoadFilePort,
    O: OrphanFilePort,
> {
    load_persona_port: L,
    delete_persona_port: D,
    event_publisher: E,
    load_file_port: LF,
    orphan_file_port: O,
}

impl<
    S: SavePersonaPort,
    U: IdGenerator,
    E: EventPublisher,
    LF: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
> PersonaCreateService<S, U, E, LF, A, O>
{
    pub fn new(prerequisites: CreatePersonaPrerequisites<S, U, E, LF, A, O>) -> Self {
        let CreatePersonaPrerequisites {
            save_persona_port,
            uid_generator,
            event_publisher,
            load_file_port,
            activate_file_port,
            orphan_file_port,
        } = prerequisites;

        Self {
            save_persona_port,
            uid_generator,
            event_publisher,
            load_file_port,
            activate_file_port,
            orphan_file_port,
        }
    }
}

impl<
    S: SavePersonaPort,
    U: IdGenerator,
    E: EventPublisher,
    LF: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
> CreatePersonaUseCase for PersonaCreateService<S, U, E, LF, A, O>
{
    async fn create_persona(
        &self,
        command: CreatePersonaCommand,
    ) -> Result<Persona, CreatePersonaError> {
        let CreatePersonaCommand {
            name,
            creator_id,
            description,
            avatar_uid,
            exclude_participants,
        } = command;

        let uid = PersonaId::try_new(self.uid_generator.generate())
            .map_err(|err| CreatePersonaError::ValidationError(format!("{err}")))?;

        let avatar_uid = if let Some(file_id) = avatar_uid {
            let scope = FileScope::PersonaAvatar {
                user_id: creator_id.clone(),
                persona_id: uid.clone(),
            };

            apply_avatar_uid_patch(
                &self.load_file_port,
                &self.activate_file_port,
                &self.orphan_file_port,
                None,
                PatchField::Set(file_id),
                &scope,
            )
            .await
            .map_err(|err| match err {
                AvatarBindingError::FileNotFound => CreatePersonaError::FileNotFound,
                AvatarBindingError::ValidationError(message) => {
                    CreatePersonaError::ValidationError(message)
                }
                AvatarBindingError::InternalError(message) => {
                    CreatePersonaError::InternalError(message)
                }
            })?
        } else {
            None
        };

        let persona = Persona {
            uid,
            creator_id,
            name,
            description,
            avatar_uid,
        };

        match self.save_persona_port.save_persona(persona).await {
            Ok(saved_persona) => {
                self.event_publisher
                    .publish(PersonaCreatedEvent {
                        persona: saved_persona.clone(),
                        exclude_participants,
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

impl<
    L: LoadPersonaPort,
    S: SavePersonaPort,
    E: EventPublisher,
    LF: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
> PersonaUpdateService<L, S, E, LF, A, O>
{
    pub fn new(prerequisites: UpdatePersonaPrerequisites<L, S, E, LF, A, O>) -> Self {
        let UpdatePersonaPrerequisites {
            load_persona_port,
            save_persona_port,
            event_publisher,
            load_file_port,
            activate_file_port,
            orphan_file_port,
        } = prerequisites;

        Self {
            load_persona_port,
            save_persona_port,
            event_publisher,
            load_file_port,
            activate_file_port,
            orphan_file_port,
        }
    }
}

impl<
    L: LoadPersonaPort,
    D: DeletePersonaPort,
    E: EventPublisher,
    LF: LoadFilePort,
    O: OrphanFilePort,
> PersonaDeleteService<L, D, E, LF, O>
{
    pub fn new(prerequisites: DeletePersonaPrerequisites<L, D, E, LF, O>) -> Self {
        let DeletePersonaPrerequisites {
            load_persona_port,
            delete_persona_port,
            event_publisher,
            load_file_port,
            orphan_file_port,
        } = prerequisites;

        Self {
            load_persona_port,
            delete_persona_port,
            event_publisher,
            load_file_port,
            orphan_file_port,
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

impl<
    L: LoadPersonaPort,
    S: SavePersonaPort,
    E: EventPublisher,
    LF: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
> UpdatePersonaUseCase for PersonaUpdateService<L, S, E, LF, A, O>
{
    async fn update_persona(
        &self,
        command: UpdatePersonaCommand,
        user_id: UserId,
    ) -> Result<Persona, UpdatePersonaError> {
        let UpdatePersonaCommand {
            id,
            name,
            description,
            avatar_uid,
            exclude_participants,
        } = command;

        let persona = self
            .load_persona_port
            .load_persona(&id, &user_id)
            .await
            .map_err(|err| match err {
                LoadPersonasPortError::NotFound => UpdatePersonaError::NotFound,
                LoadPersonasPortError::InternalError(message) => {
                    UpdatePersonaError::InternalError(message)
                }
            })?;

        can_update_persona(&user_id, &persona).map_err(|_| UpdatePersonaError::AccessDenied)?;

        let Persona {
            uid,
            creator_id,
            name: existing_name,
            description: existing_description,
            avatar_uid: current_avatar_uid,
        } = persona;

        let expected_scope = FileScope::PersonaAvatar {
            user_id: user_id.clone(),
            persona_id: uid.clone(),
        };

        let next_avatar_uid = apply_avatar_uid_patch(
            &self.load_file_port,
            &self.activate_file_port,
            &self.orphan_file_port,
            current_avatar_uid,
            avatar_uid,
            &expected_scope,
        )
        .await
        .map_err(|err| match err {
            AvatarBindingError::FileNotFound => UpdatePersonaError::FileNotFound,
            AvatarBindingError::ValidationError(message) => {
                UpdatePersonaError::ValidationError(message)
            }
            AvatarBindingError::InternalError(message) => {
                UpdatePersonaError::InternalError(message)
            }
        })?;

        let updated_persona = Persona {
            uid,
            creator_id,
            name: name.unwrap_or(existing_name),
            description: description.unwrap_or(existing_description),
            avatar_uid: next_avatar_uid,
        };

        let saved_persona = self
            .save_persona_port
            .save_persona(updated_persona)
            .await
            .map_err(|err| UpdatePersonaError::InternalError(err.to_string()))?;

        self.event_publisher
            .publish(PersonaUpdatedEvent {
                persona: saved_persona.clone(),
                exclude_participants,
            })
            .await;

        Ok(saved_persona)
    }
}

impl<
    L: LoadPersonaPort,
    D: DeletePersonaPort,
    E: EventPublisher,
    LF: LoadFilePort,
    O: OrphanFilePort,
> DeletePersonaUseCase for PersonaDeleteService<L, D, E, LF, O>
{
    async fn delete_persona(
        &self,
        command: DeletePersonaCommand,
        user_id: UserId,
    ) -> Result<(), DeletePersonaError> {
        let DeletePersonaCommand {
            id,
            exclude_participants,
        } = command;

        let persona = self
            .load_persona_port
            .load_persona(&id, &user_id)
            .await
            .map_err(|err| match err {
                LoadPersonasPortError::NotFound => DeletePersonaError::NotFound,
                LoadPersonasPortError::InternalError(message) => {
                    DeletePersonaError::InternalError(message)
                }
            })?;

        can_delete_persona(&user_id, &persona).map_err(|_| DeletePersonaError::AccessDenied)?;

        orphan_avatar_if_present(
            &self.load_file_port,
            &self.orphan_file_port,
            persona.avatar_uid.clone(),
        )
        .await
        .map_err(|err| DeletePersonaError::InternalError(err.to_string()))?;

        self.delete_persona_port
            .delete_persona(&id, &user_id)
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
                exclude_participants,
            })
            .await;

        Ok(())
    }
}
