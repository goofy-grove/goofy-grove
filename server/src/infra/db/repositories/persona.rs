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
    async fn load_personas(&self, user_id: &UserId) -> Result<Vec<Persona>, LoadPersonasPortError> {
        let personas = Personas::find()
            .filter(personas::Column::CreatorId.eq(user_id.inner()))
            .all(&self.connection)
            .await
            .map_err(|err| LoadPersonasPortError::InternalError(err.to_string()))?;

        let mut result = Vec::with_capacity(personas.len());

        for persona in personas {
            let persona_id = PersonaId::try_new(persona.uid)
                .map_err(|err| LoadPersonasPortError::InternalError(err.to_string()))?;
            let creator_id = UserId::try_new(persona.creator_id)
                .map_err(|err| LoadPersonasPortError::InternalError(err.to_string()))?;
            let persona_name = PersonaName::try_new(persona.name)
                .map_err(|err| LoadPersonasPortError::InternalError(err.to_string()))?;
            let description = PersonaDescription::new(persona.description);

            result.push(Persona::new(
                persona_id,
                creator_id,
                persona_name,
                description,
            ));
        }

        Ok(result)
    }
}

impl LoadPersonaPort for PersonaRepository {
    async fn load_persona(
        &self,
        persona_id: &PersonaId,
        user_id: &UserId,
    ) -> Result<Persona, LoadPersonasPortError> {
        let persona = Personas::find()
            .filter(personas::Column::Uid.eq(persona_id.inner()))
            .filter(personas::Column::CreatorId.eq(user_id.inner()))
            .one(&self.connection)
            .await
            .map_err(|err| LoadPersonasPortError::InternalError(err.to_string()))?
            .ok_or(LoadPersonasPortError::NotFound)?;

        let persona_id = PersonaId::try_new(persona.uid)
            .map_err(|err| LoadPersonasPortError::InternalError(err.to_string()))?;
        let creator_id = UserId::try_new(persona.creator_id)
            .map_err(|err| LoadPersonasPortError::InternalError(err.to_string()))?;
        let persona_name = PersonaName::try_new(persona.name)
            .map_err(|err| LoadPersonasPortError::InternalError(err.to_string()))?;
        let description = PersonaDescription::new(persona.description);

        Ok(Persona::new(
            persona_id,
            creator_id,
            persona_name,
            description,
        ))
    }
}

impl SavePersonaPort for PersonaRepository {
    async fn save_persona(&self, persona: Persona) -> Result<Persona, SavePersonaPortError> {
        let new_persona = personas::ActiveModel {
            uid: Set(persona.uid().inner().to_owned()),
            creator_id: Set(persona.creator_id().inner().to_owned()),
            name: Set(persona.name().inner().to_owned()),
            description: Set(persona.description().inner().to_owned()),
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
                PersonaId::try_new(inserted_persona.uid)
                    .map_err(|err| SavePersonaPortError::InternalError(err.to_string()))?,
                UserId::try_new(inserted_persona.creator_id)
                    .map_err(|err| SavePersonaPortError::InternalError(err.to_string()))?,
                PersonaName::try_new(inserted_persona.name)
                    .map_err(|err| SavePersonaPortError::InternalError(err.to_string()))?,
                PersonaDescription::new(inserted_persona.description),
            )),
            Err(err) => Err(SavePersonaPortError::InternalError(err.to_string())),
        }
    }
}

impl DeletePersonaPort for PersonaRepository {
    async fn delete_persona(
        &self,
        persona_id: &PersonaId,
        user_id: &UserId,
    ) -> Result<(), DeletePersonaPortError> {
        let result = Personas::delete_many()
            .filter(personas::Column::Uid.eq(persona_id.inner()))
            .filter(personas::Column::CreatorId.eq(user_id.inner()))
            .exec(&self.connection)
            .await
            .map_err(|err| DeletePersonaPortError::InternalError(err.to_string()))?;

        if result.rows_affected == 0 {
            return Err(DeletePersonaPortError::NotFound);
        }

        Ok(())
    }
}
