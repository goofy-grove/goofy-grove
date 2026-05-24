use std::pin::Pin;
use std::sync::{Arc, Mutex};

use super::*;

#[derive(Clone)]
struct SavePersonaOk;
impl SavePersonaPort for SavePersonaOk {
    async fn save_persona(&self, persona: Persona) -> Result<Persona, SavePersonaPortError> {
        Ok(persona)
    }
}

#[derive(Clone)]
struct SavePersonaErr;
impl SavePersonaPort for SavePersonaErr {
    async fn save_persona(&self, _persona: Persona) -> Result<Persona, SavePersonaPortError> {
        Err(SavePersonaPortError::InternalError("db".into()))
    }
}

#[derive(Clone)]
struct LoadPersonasOk {
    personas: Vec<Persona>,
}
impl LoadPersonasPort for LoadPersonasOk {
    async fn load_personas(
        &self,
        _user_id: &UserId,
    ) -> Result<Vec<Persona>, LoadPersonasPortError> {
        Ok(self.personas.clone())
    }
}

#[derive(Clone)]
struct LoadPersonasErr;
impl LoadPersonasPort for LoadPersonasErr {
    async fn load_personas(
        &self,
        _user_id: &UserId,
    ) -> Result<Vec<Persona>, LoadPersonasPortError> {
        Err(LoadPersonasPortError::InternalError("db".into()))
    }
}

#[derive(Clone)]
struct LoadPersonaOk {
    persona: Persona,
}
impl LoadPersonaPort for LoadPersonaOk {
    async fn load_persona(
        &self,
        _persona_id: &PersonaId,
        _user_id: &UserId,
    ) -> Result<Persona, LoadPersonasPortError> {
        Ok(self.persona.clone())
    }
}

#[derive(Clone)]
struct LoadPersonaNotFound;
impl LoadPersonaPort for LoadPersonaNotFound {
    async fn load_persona(
        &self,
        _persona_id: &PersonaId,
        _user_id: &UserId,
    ) -> Result<Persona, LoadPersonasPortError> {
        Err(LoadPersonasPortError::NotFound)
    }
}

#[derive(Clone)]
struct DeletePersonaOk;
impl DeletePersonaPort for DeletePersonaOk {
    async fn delete_persona(
        &self,
        _persona_id: &PersonaId,
        _user_id: &UserId,
    ) -> Result<(), DeletePersonaPortError> {
        Ok(())
    }
}

#[derive(Clone)]
struct DeletePersonaErr;
impl DeletePersonaPort for DeletePersonaErr {
    async fn delete_persona(
        &self,
        _persona_id: &PersonaId,
        _user_id: &UserId,
    ) -> Result<(), DeletePersonaPortError> {
        Err(DeletePersonaPortError::InternalError("db".into()))
    }
}

#[derive(Clone)]
struct FixedId;
impl IdGenerator for FixedId {
    fn generate(&self) -> String {
        "persona-1".to_string()
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

fn sample_command() -> CreatePersonaCommand {
    CreatePersonaCommand {
        name: PersonaName::try_new("Guide".to_string()).unwrap(),
        creator_id: UserId::try_new("user-1".to_string()).unwrap(),
        description: PersonaDescription::new("friendly".to_string()),
        exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
    }
}

#[tokio::test]
async fn create_persona_saves_and_publishes_event() {
    let hits = Arc::new(Mutex::new(0));
    let service = PersonaCreateService::new(
        SavePersonaOk,
        FixedId,
        RecordingPublisher { hits: hits.clone() },
    );

    assert!(service.create_persona(sample_command()).await.is_ok());
    assert_eq!(*hits.lock().unwrap(), 1);
}

#[tokio::test]
async fn create_persona_maps_storage_error() {
    let hits = Arc::new(Mutex::new(0));
    let service = PersonaCreateService::new(SavePersonaErr, FixedId, RecordingPublisher { hits });

    assert!(matches!(
        service.create_persona(sample_command()).await,
        Err(CreatePersonaError::InternalError(_))
    ));
}

#[tokio::test]
async fn create_persona_maps_validation_error_for_invalid_generated_id() {
    let hits = Arc::new(Mutex::new(0));
    let service = PersonaCreateService::new(SavePersonaOk, InvalidId, RecordingPublisher { hits });

    assert!(matches!(
        service.create_persona(sample_command()).await,
        Err(CreatePersonaError::ValidationError(_))
    ));
}

#[tokio::test]
async fn get_personas_returns_loaded_list() {
    let persona = Persona {
        uid: PersonaId::try_new("persona-1".to_string()).unwrap(),
        creator_id: UserId::try_new("user-1".to_string()).unwrap(),
        name: PersonaName::try_new("Guide".to_string()).unwrap(),
        description: PersonaDescription::new("friendly".to_string()),
    };
    let service = GetPersonasService::new(LoadPersonasOk {
        personas: vec![persona],
    });
    let result = service
        .get_personas(&UserId::try_new("user-1".to_string()).unwrap())
        .await
        .unwrap();

    assert_eq!(result.len(), 1);
}

#[tokio::test]
async fn get_personas_maps_load_errors() {
    let service = GetPersonasService::new(LoadPersonasErr);

    assert!(matches!(
        service
            .get_personas(&UserId::try_new("user-1".to_string()).unwrap())
            .await,
        Err(GetPersonasError::InternalError(_))
    ));
}

#[tokio::test]
async fn delete_persona_deletes_and_publishes_event() {
    let hits = Arc::new(Mutex::new(0));
    let persona = Persona {
        uid: PersonaId::try_new("persona-1".to_string()).unwrap(),
        creator_id: UserId::try_new("user-1".to_string()).unwrap(),
        name: PersonaName::try_new("Guide".to_string()).unwrap(),
        description: PersonaDescription::new("friendly".to_string()),
    };
    let service = PersonaDeleteService::new(
        LoadPersonaOk { persona },
        DeletePersonaOk,
        RecordingPublisher { hits: hits.clone() },
    );

    let result = service
        .delete_persona(
            DeletePersonaCommand {
                id: PersonaId::try_new("persona-1".to_string()).unwrap(),
                exclude_participants: vec![ParticipantId::try_new("user-2".to_string()).unwrap()],
            },
            UserId::try_new("user-1".to_string()).unwrap(),
        )
        .await;

    assert!(result.is_ok());
    assert_eq!(*hits.lock().unwrap(), 1);
}

#[tokio::test]
async fn delete_persona_returns_not_found_when_load_fails() {
    let hits = Arc::new(Mutex::new(0));
    let service = PersonaDeleteService::new(
        LoadPersonaNotFound,
        DeletePersonaOk,
        RecordingPublisher { hits },
    );

    let result = service
        .delete_persona(
            DeletePersonaCommand {
                id: PersonaId::try_new("persona-1".to_string()).unwrap(),
                exclude_participants: vec![],
            },
            UserId::try_new("user-1".to_string()).unwrap(),
        )
        .await;

    assert!(matches!(result, Err(DeletePersonaError::NotFound)));
}

#[tokio::test]
async fn delete_persona_maps_delete_errors() {
    let hits = Arc::new(Mutex::new(0));
    let persona = Persona {
        uid: PersonaId::try_new("persona-1".to_string()).unwrap(),
        creator_id: UserId::try_new("user-1".to_string()).unwrap(),
        name: PersonaName::try_new("Guide".to_string()).unwrap(),
        description: PersonaDescription::new("friendly".to_string()),
    };
    let service = PersonaDeleteService::new(
        LoadPersonaOk { persona },
        DeletePersonaErr,
        RecordingPublisher { hits },
    );

    let result = service
        .delete_persona(
            DeletePersonaCommand {
                id: PersonaId::try_new("persona-1".to_string()).unwrap(),
                exclude_participants: vec![],
            },
            UserId::try_new("user-1".to_string()).unwrap(),
        )
        .await;

    assert!(matches!(result, Err(DeletePersonaError::InternalError(_))));
}
