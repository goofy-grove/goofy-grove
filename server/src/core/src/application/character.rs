use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct CharacterCreateService<S: SaveCharacterPort, U: IdGenerator, E: EventPublisher> {
    save_character_port: S,
    uid_generator: U,
    event_publisher: E,
}

#[derive(Debug, Clone)]
pub struct GetCharactersService<L: LoadCharactersPort> {
    load_characters_port: L,
}

#[derive(Debug, Clone)]
pub struct CharacterUpdateService<L: LoadCharacterPort, S: SaveCharacterPort, E: EventPublisher> {
    load_character_port: L,
    save_character_port: S,
    event_publisher: E,
}

#[derive(Debug, Clone)]
pub struct CharacterDeleteService<L: LoadCharacterPort, D: DeleteCharacterPort, E: EventPublisher> {
    load_character_port: L,
    delete_character_port: D,
    event_publisher: E,
}

impl<S: SaveCharacterPort, U: IdGenerator, E: EventPublisher> CharacterCreateService<S, U, E> {
    pub fn new(save_character_port: S, uid_generator: U, event_publisher: E) -> Self {
        Self {
            save_character_port,
            uid_generator,
            event_publisher,
        }
    }
}

impl<S: SaveCharacterPort, U: IdGenerator, E: EventPublisher> CreateCharacterUseCase
    for CharacterCreateService<S, U, E>
{
    async fn create_character(
        &self,
        command: CreateCharacterCommand,
    ) -> Result<Character, CreateCharacterError> {
        let character = Character::new(
            CharacterId::try_new(self.uid_generator.generate())
                .map_err(|err| CreateCharacterError::ValidationError(format!("{err}")))?,
            command.creator_id().clone(),
            command.name().clone(),
            command.description().clone(),
        );

        match self.save_character_port.save_character(character).await {
            Ok(saved_character) => {
                self.event_publisher
                    .publish(CharacterCreatedEvent {
                        character: saved_character.clone(),
                        exclude_participants: command.exclude_participants().clone(),
                    })
                    .await;

                Ok(saved_character)
            }
            Err(err) => Err(CreateCharacterError::InternalError(format!("{:?}", err))),
        }
    }
}

impl<L: LoadCharactersPort> GetCharactersService<L> {
    pub fn new(load_characters_port: L) -> Self {
        Self {
            load_characters_port,
        }
    }
}

impl<L: LoadCharacterPort, S: SaveCharacterPort, E: EventPublisher>
    CharacterUpdateService<L, S, E>
{
    pub fn new(load_character_port: L, save_character_port: S, event_publisher: E) -> Self {
        Self {
            load_character_port,
            save_character_port,
            event_publisher,
        }
    }
}

impl<L: LoadCharacterPort, D: DeleteCharacterPort, E: EventPublisher>
    CharacterDeleteService<L, D, E>
{
    pub fn new(load_character_port: L, delete_character_port: D, event_publisher: E) -> Self {
        Self {
            load_character_port,
            delete_character_port,
            event_publisher,
        }
    }
}

impl<L: LoadCharactersPort> GetCharactersQuery for GetCharactersService<L> {
    async fn get_characters(&self, user_id: &UserId) -> Result<Vec<Character>, GetCharactersError> {
        self.load_characters_port
            .load_characters(user_id)
            .await
            .map_err(|err| GetCharactersError::InternalError(err.to_string()))
    }
}

impl<L: LoadCharacterPort, S: SaveCharacterPort, E: EventPublisher> UpdateCharacterUseCase
    for CharacterUpdateService<L, S, E>
{
    async fn update_character(
        &self,
        command: UpdateCharacterCommand,
        user_id: UserId,
    ) -> Result<Character, UpdateCharacterError> {
        let character = self
            .load_character_port
            .load_character(command.id(), &user_id)
            .await
            .map_err(|err| match err {
                LoadCharactersPortError::NotFound => UpdateCharacterError::NotFound,
                LoadCharactersPortError::InternalError(message) => {
                    UpdateCharacterError::InternalError(message)
                }
            })?;

        let updated_character = Character::new(
            character.uid().clone(),
            character.creator_id().clone(),
            command
                .name()
                .clone()
                .unwrap_or_else(|| character.name().clone()),
            command
                .description()
                .clone()
                .unwrap_or_else(|| character.description().clone()),
        );

        let saved_character = self
            .save_character_port
            .save_character(updated_character)
            .await
            .map_err(|err| UpdateCharacterError::InternalError(err.to_string()))?;

        self.event_publisher
            .publish(CharacterUpdatedEvent {
                character: saved_character.clone(),
                exclude_participants: command.exclude_participants().clone(),
            })
            .await;

        Ok(saved_character)
    }
}

impl<L: LoadCharacterPort, D: DeleteCharacterPort, E: EventPublisher> DeleteCharacterUseCase
    for CharacterDeleteService<L, D, E>
{
    async fn delete_character(
        &self,
        command: DeleteCharacterCommand,
        user_id: UserId,
    ) -> Result<(), DeleteCharacterError> {
        let character = self
            .load_character_port
            .load_character(command.id(), &user_id)
            .await
            .map_err(|err| match err {
                LoadCharactersPortError::NotFound => DeleteCharacterError::NotFound,
                LoadCharactersPortError::InternalError(message) => {
                    DeleteCharacterError::InternalError(message)
                }
            })?;

        self.delete_character_port
            .delete_character(command.id(), &user_id)
            .await
            .map_err(|err| match err {
                DeleteCharacterPortError::NotFound => DeleteCharacterError::NotFound,
                DeleteCharacterPortError::InternalError(message) => {
                    DeleteCharacterError::InternalError(message)
                }
            })?;

        self.event_publisher
            .publish(CharacterDeletedEvent {
                character,
                exclude_participants: command.exclude_participants().clone(),
            })
            .await;

        Ok(())
    }
}
