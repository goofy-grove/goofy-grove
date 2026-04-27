use crate::infra::db::entities::{characters, prelude::Characters};
use gg_core::domain::prelude::*;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, sea_query,
};

#[derive(Debug, Clone)]
pub struct CharacterRepository {
    connection: DatabaseConnection,
}

impl CharacterRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl LoadCharactersPort for CharacterRepository {
    async fn load_characters(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Character>, LoadCharactersPortError> {
        let characters = Characters::find()
            .filter(characters::Column::UserId.eq(user_id.inner()))
            .all(&self.connection)
            .await
            .map_err(|err| LoadCharactersPortError::InternalError(err.to_string()))?;

        let mut result = Vec::with_capacity(characters.len());

        for character in characters {
            let character_id = CharacterId::try_new(character.uid)
                .map_err(|err| LoadCharactersPortError::InternalError(err.to_string()))?;
            let creator_id = UserId::try_new(character.user_id)
                .map_err(|err| LoadCharactersPortError::InternalError(err.to_string()))?;
            let character_name = CharacterName::try_new(character.name)
                .map_err(|err| LoadCharactersPortError::InternalError(err.to_string()))?;
            let description = CharacterDescription::new(character.description);

            result.push(Character::new(
                character_id,
                creator_id,
                character_name,
                description,
            ));
        }

        Ok(result)
    }
}

impl LoadCharacterPort for CharacterRepository {
    async fn load_character(
        &self,
        character_id: &CharacterId,
        user_id: &UserId,
    ) -> Result<Character, LoadCharactersPortError> {
        let character = Characters::find()
            .filter(characters::Column::Uid.eq(character_id.inner()))
            .filter(characters::Column::UserId.eq(user_id.inner()))
            .one(&self.connection)
            .await
            .map_err(|err| LoadCharactersPortError::InternalError(err.to_string()))?
            .ok_or(LoadCharactersPortError::NotFound)?;

        let character_id = CharacterId::try_new(character.uid)
            .map_err(|err| LoadCharactersPortError::InternalError(err.to_string()))?;
        let creator_id = UserId::try_new(character.user_id)
            .map_err(|err| LoadCharactersPortError::InternalError(err.to_string()))?;
        let character_name = CharacterName::try_new(character.name)
            .map_err(|err| LoadCharactersPortError::InternalError(err.to_string()))?;
        let description = CharacterDescription::new(character.description);

        Ok(Character::new(
            character_id,
            creator_id,
            character_name,
            description,
        ))
    }
}

impl SaveCharacterPort for CharacterRepository {
    async fn save_character(
        &self,
        character: Character,
    ) -> Result<Character, SaveCharacterPortError> {
        let new_character = characters::ActiveModel {
            uid: Set(character.uid().inner().to_owned()),
            user_id: Set(character.creator_id().inner().to_owned()),
            name: Set(character.name().inner().to_owned()),
            description: Set(character.description().inner().to_owned()),
        };
        let request = Characters::insert(new_character)
            .on_conflict(
                sea_query::OnConflict::column(characters::Column::Uid)
                    .update_columns([
                        characters::Column::Name,
                        characters::Column::Description,
                        characters::Column::UserId,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.connection)
            .await;

        match request {
            Ok(inserted_character) => Ok(Character::new(
                CharacterId::try_new(inserted_character.uid)
                    .map_err(|err| SaveCharacterPortError::InternalError(err.to_string()))?,
                UserId::try_new(inserted_character.user_id)
                    .map_err(|err| SaveCharacterPortError::InternalError(err.to_string()))?,
                CharacterName::try_new(inserted_character.name)
                    .map_err(|err| SaveCharacterPortError::InternalError(err.to_string()))?,
                CharacterDescription::new(inserted_character.description),
            )),
            Err(err) => Err(SaveCharacterPortError::InternalError(err.to_string())),
        }
    }
}

impl DeleteCharacterPort for CharacterRepository {
    async fn delete_character(
        &self,
        character_id: &CharacterId,
        user_id: &UserId,
    ) -> Result<(), DeleteCharacterPortError> {
        let result = Characters::delete_many()
            .filter(characters::Column::Uid.eq(character_id.inner()))
            .filter(characters::Column::UserId.eq(user_id.inner()))
            .exec(&self.connection)
            .await
            .map_err(|err| DeleteCharacterPortError::InternalError(err.to_string()))?;

        if result.rows_affected == 0 {
            return Err(DeleteCharacterPortError::NotFound);
        }

        Ok(())
    }
}
