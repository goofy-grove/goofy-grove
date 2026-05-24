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

            result.push(Persona {
                uid: persona_id,
                creator_id,
                name: persona_name,
                description,
            });
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

        Ok(Persona {
            uid: persona_id,
            creator_id,
            name: persona_name,
            description,
        })
    }
}

impl SavePersonaPort for PersonaRepository {
    async fn save_persona(&self, persona: Persona) -> Result<Persona, SavePersonaPortError> {
        let Persona {
            uid,
            creator_id,
            name,
            description,
        } = persona;

        let new_persona = personas::ActiveModel {
            uid: Set(uid.into_inner()),
            creator_id: Set(creator_id.into_inner()),
            name: Set(name.into_inner()),
            description: Set(description.into_inner()),
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
            Ok(inserted_persona) => Ok(Persona {
                uid: PersonaId::try_new(inserted_persona.uid)
                    .map_err(|err| SavePersonaPortError::InternalError(err.to_string()))?,
                creator_id: UserId::try_new(inserted_persona.creator_id)
                    .map_err(|err| SavePersonaPortError::InternalError(err.to_string()))?,
                name: PersonaName::try_new(inserted_persona.name)
                    .map_err(|err| SavePersonaPortError::InternalError(err.to_string()))?,
                description: PersonaDescription::new(inserted_persona.description),
            }),
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
