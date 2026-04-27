use std::pin::Pin;
use std::sync::{Arc, Mutex};

use super::*;

#[derive(Clone)]
struct SaveCharacterOk;
impl SaveCharacterPort for SaveCharacterOk {
    async fn save_character(
        &self,
        character: Character,
    ) -> Result<Character, SaveCharacterPortError> {
        Ok(character)
    }
}

#[derive(Clone)]
struct SaveCharacterErr;
impl SaveCharacterPort for SaveCharacterErr {
    async fn save_character(
        &self,
        _character: Character,
    ) -> Result<Character, SaveCharacterPortError> {
        Err(SaveCharacterPortError::InternalError("db".into()))
    }
}

#[derive(Clone)]
struct LoadCharactersOk {
    characters: Vec<Character>,
}
impl LoadCharactersPort for LoadCharactersOk {
    async fn load_characters(
        &self,
        _user_id: &UserId,
    ) -> Result<Vec<Character>, LoadCharactersPortError> {
        Ok(self.characters.clone())
    }
}

#[derive(Clone)]
struct LoadCharactersErr;
impl LoadCharactersPort for LoadCharactersErr {
    async fn load_characters(
        &self,
        _user_id: &UserId,
    ) -> Result<Vec<Character>, LoadCharactersPortError> {
        Err(LoadCharactersPortError::InternalError("db".into()))
    }
}

#[derive(Clone)]
struct LoadCharacterOk {
    character: Character,
}
impl LoadCharacterPort for LoadCharacterOk {
    async fn load_character(
        &self,
        _character_id: &CharacterId,
        _user_id: &UserId,
    ) -> Result<Character, LoadCharactersPortError> {
        Ok(self.character.clone())
    }
}

#[derive(Clone)]
struct LoadCharacterNotFound;
impl LoadCharacterPort for LoadCharacterNotFound {
    async fn load_character(
        &self,
        _character_id: &CharacterId,
        _user_id: &UserId,
    ) -> Result<Character, LoadCharactersPortError> {
        Err(LoadCharactersPortError::NotFound)
    }
}

#[derive(Clone)]
struct DeleteCharacterOk;
impl DeleteCharacterPort for DeleteCharacterOk {
    async fn delete_character(
        &self,
        _character_id: &CharacterId,
        _user_id: &UserId,
    ) -> Result<(), DeleteCharacterPortError> {
        Ok(())
    }
}

#[derive(Clone)]
struct DeleteCharacterErr;
impl DeleteCharacterPort for DeleteCharacterErr {
    async fn delete_character(
        &self,
        _character_id: &CharacterId,
        _user_id: &UserId,
    ) -> Result<(), DeleteCharacterPortError> {
        Err(DeleteCharacterPortError::InternalError("db".into()))
    }
}

#[derive(Clone)]
struct FixedId;
impl IdGenerator for FixedId {
    fn generate(&self) -> String {
        "character-1".to_string()
    }
}

#[derive(Clone)]
struct InvalidId;
impl IdGenerator for InvalidId {
    fn generate(&self) -> String {
        "   ".to_string()
    }
}

#[derive(Clone)]
struct RecordingPublisher {
    hits: Arc<Mutex<usize>>,
}
impl EventPublisher for RecordingPublisher {
    fn publish<E: Event>(&self, _event: E) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let hits = self.hits.clone();

        Box::pin(async move {
            let mut lock = hits.lock().unwrap();
            *lock += 1;
        })
    }
}

fn sample_command() -> CreateCharacterCommand {
    CreateCharacterCommand::new(
        CharacterName::try_new("Knight".to_string()).unwrap(),
        UserId::try_new("user-1".to_string()).unwrap(),
        CharacterDescription::new("brave".to_string()),
        vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    )
}

#[tokio::test]
async fn create_character_saves_and_publishes_event() {
    let hits = Arc::new(Mutex::new(0));
    let service = CharacterCreateService::new(
        SaveCharacterOk,
        FixedId,
        RecordingPublisher { hits: hits.clone() },
    );

    assert!(service.create_character(sample_command()).await.is_ok());
    assert_eq!(*hits.lock().unwrap(), 1);
}

#[tokio::test]
async fn create_character_maps_storage_error() {
    let hits = Arc::new(Mutex::new(0));
    let service =
        CharacterCreateService::new(SaveCharacterErr, FixedId, RecordingPublisher { hits });

    assert!(matches!(
        service.create_character(sample_command()).await,
        Err(CreateCharacterError::InternalError(_))
    ));
}

#[tokio::test]
async fn create_character_maps_validation_error_for_invalid_generated_id() {
    let hits = Arc::new(Mutex::new(0));
    let service =
        CharacterCreateService::new(SaveCharacterOk, InvalidId, RecordingPublisher { hits });

    assert!(matches!(
        service.create_character(sample_command()).await,
        Err(CreateCharacterError::ValidationError(_))
    ));
}

#[tokio::test]
async fn get_characters_returns_loaded_list() {
    let character = Character::new(
        CharacterId::try_new("character-1".to_string()).unwrap(),
        UserId::try_new("user-1".to_string()).unwrap(),
        CharacterName::try_new("Knight".to_string()).unwrap(),
        CharacterDescription::new("brave".to_string()),
    );
    let service = GetCharactersService::new(LoadCharactersOk {
        characters: vec![character],
    });
    let result = service
        .get_characters(&UserId::try_new("user-1".to_string()).unwrap())
        .await
        .unwrap();

    assert_eq!(result.len(), 1);
}

#[tokio::test]
async fn get_characters_maps_load_errors() {
    let service = GetCharactersService::new(LoadCharactersErr);

    assert!(matches!(
        service
            .get_characters(&UserId::try_new("user-1".to_string()).unwrap())
            .await,
        Err(GetCharactersError::InternalError(_))
    ));
}

#[tokio::test]
async fn update_character_updates_existing_and_publishes_event() {
    let hits = Arc::new(Mutex::new(0));
    let character = Character::new(
        CharacterId::try_new("character-1".to_string()).unwrap(),
        UserId::try_new("user-1".to_string()).unwrap(),
        CharacterName::try_new("Knight".to_string()).unwrap(),
        CharacterDescription::new("brave".to_string()),
    );
    let service = CharacterUpdateService::new(
        LoadCharacterOk { character },
        SaveCharacterOk,
        RecordingPublisher { hits: hits.clone() },
    );

    let result = service
        .update_character(
            UpdateCharacterCommand::new(
                CharacterId::try_new("character-1".to_string()).unwrap(),
                Some(CharacterName::try_new("Mage".to_string()).unwrap()),
                None,
                vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
            ),
            UserId::try_new("user-1".to_string()).unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(result.name().inner(), "Mage");
    assert_eq!(*hits.lock().unwrap(), 1);
}

#[tokio::test]
async fn update_character_returns_not_found_when_load_fails() {
    let hits = Arc::new(Mutex::new(0));
    let service = CharacterUpdateService::new(
        LoadCharacterNotFound,
        SaveCharacterOk,
        RecordingPublisher { hits },
    );

    let result = service
        .update_character(
            UpdateCharacterCommand::new(
                CharacterId::try_new("character-1".to_string()).unwrap(),
                Some(CharacterName::try_new("Mage".to_string()).unwrap()),
                None,
                vec![],
            ),
            UserId::try_new("user-1".to_string()).unwrap(),
        )
        .await;

    assert!(matches!(result, Err(UpdateCharacterError::NotFound)));
}

#[tokio::test]
async fn delete_character_deletes_and_publishes_event() {
    let hits = Arc::new(Mutex::new(0));
    let character = Character::new(
        CharacterId::try_new("character-1".to_string()).unwrap(),
        UserId::try_new("user-1".to_string()).unwrap(),
        CharacterName::try_new("Knight".to_string()).unwrap(),
        CharacterDescription::new("brave".to_string()),
    );
    let service = CharacterDeleteService::new(
        LoadCharacterOk { character },
        DeleteCharacterOk,
        RecordingPublisher { hits: hits.clone() },
    );

    let result = service
        .delete_character(
            DeleteCharacterCommand::new(
                CharacterId::try_new("character-1".to_string()).unwrap(),
                vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
            ),
            UserId::try_new("user-1".to_string()).unwrap(),
        )
        .await;

    assert!(result.is_ok());
    assert_eq!(*hits.lock().unwrap(), 1);
}

#[tokio::test]
async fn delete_character_returns_not_found_when_load_fails() {
    let hits = Arc::new(Mutex::new(0));
    let service = CharacterDeleteService::new(
        LoadCharacterNotFound,
        DeleteCharacterOk,
        RecordingPublisher { hits },
    );

    let result = service
        .delete_character(
            DeleteCharacterCommand::new(
                CharacterId::try_new("character-1".to_string()).unwrap(),
                vec![],
            ),
            UserId::try_new("user-1".to_string()).unwrap(),
        )
        .await;

    assert!(matches!(result, Err(DeleteCharacterError::NotFound)));
}

#[tokio::test]
async fn delete_character_maps_delete_errors() {
    let hits = Arc::new(Mutex::new(0));
    let character = Character::new(
        CharacterId::try_new("character-1".to_string()).unwrap(),
        UserId::try_new("user-1".to_string()).unwrap(),
        CharacterName::try_new("Knight".to_string()).unwrap(),
        CharacterDescription::new("brave".to_string()),
    );
    let service = CharacterDeleteService::new(
        LoadCharacterOk { character },
        DeleteCharacterErr,
        RecordingPublisher { hits },
    );

    let result = service
        .delete_character(
            DeleteCharacterCommand::new(
                CharacterId::try_new("character-1".to_string()).unwrap(),
                vec![],
            ),
            UserId::try_new("user-1".to_string()).unwrap(),
        )
        .await;

    assert!(matches!(
        result,
        Err(DeleteCharacterError::InternalError(_))
    ));
}
