use crate::infra::db::entities::{personas, prelude::Personas};
use gg_core::domain::prelude::*;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, sea_query,
};

#[derive(Debug, Clone)]
pub struct PersonaRepository {
    connection: DatabaseConnection,
}

impl PersonaRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl LoadPersonasPort for PersonaRepository {
    async fn load_personas(&self, user_id: &UserId) -> Vec<Persona> {
        let personas = Personas::find()
            .filter(personas::Column::CreatorId.eq(user_id.value()))
            .all(&self.connection)
            .await;

        match personas {
            Ok(personas) => personas
                .into_iter()
                .map(|persona| {
                    Persona::new(
                        PersonaId::new(persona.uid),
                        UserId::new(persona.creator_id),
                        PersonaName::new(persona.name),
                        PersonaDescription::new(persona.description),
                    )
                })
                .collect(),
            Err(err) => {
                // TODO: add error propagation
                log::error!("Failed to load personas: {}", err);

                vec![]
            }
        }
    }
}

impl SavePersonaPort for PersonaRepository {
    async fn save_persona(&self, persona: Persona) -> DomainResult<Persona, SavePersonaPortError> {
        let new_persona = personas::ActiveModel {
            uid: Set(persona.uid().value().to_owned()),
            creator_id: Set(persona.creator_id().value().to_owned()),
            name: Set(persona.name().value().to_owned()),
            description: Set(persona.description().value().to_owned()),
        };
        let request = Personas::insert(new_persona)
            .on_conflict(
                sea_query::OnConflict::column(personas::Column::Uid)
                    .update_columns([
                        personas::Column::Name,
                        personas::Column::Description,
                        personas::Column::CreatorId,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.connection)
            .await;

        match request {
            Ok(inserted_persona) => Ok(Persona::new(
                PersonaId::new(inserted_persona.uid),
                UserId::new(inserted_persona.creator_id),
                PersonaName::new(inserted_persona.name),
                PersonaDescription::new(inserted_persona.description),
            )),
            Err(err) => Err(DomainError::ExternalServiceError(
                SavePersonaPortError::InternalError(err.to_string()),
            )),
        }
    }
}
