use crate::infra::db::entities::{personas, prelude::Personas};
use crate::infra::db::mappers::persona_from_model;
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

        personas
            .into_iter()
            .map(|persona| {
                persona_from_model(
                    persona.uid,
                    persona.creator_id,
                    persona.name,
                    persona.description,
                    persona.avatar_uid,
                )
                .map_err(LoadPersonasPortError::InternalError)
            })
            .collect()
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

        persona_from_model(
            persona.uid,
            persona.creator_id,
            persona.name,
            persona.description,
            persona.avatar_uid,
        )
        .map_err(LoadPersonasPortError::InternalError)
    }
}

impl SavePersonaPort for PersonaRepository {
    async fn save_persona(&self, persona: Persona) -> Result<Persona, SavePersonaPortError> {
        let Persona {
            uid,
            creator_id,
            name,
            description,
            avatar_uid,
        } = persona;

        let new_persona = personas::ActiveModel {
            uid: Set(uid.into_inner()),
            creator_id: Set(creator_id.into_inner()),
            name: Set(name.into_inner()),
            description: Set(description.into_inner()),
            avatar_uid: Set(avatar_uid.map(|value| value.into_inner())),
        };
        let request = Personas::insert(new_persona)
            .on_conflict(
                sea_query::OnConflict::column(personas::Column::Uid)
                    .update_columns([
                        personas::Column::Name,
                        personas::Column::Description,
                        personas::Column::CreatorId,
                        personas::Column::AvatarUid,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.connection)
            .await;

        match request {
            Ok(inserted_persona) => persona_from_model(
                inserted_persona.uid,
                inserted_persona.creator_id,
                inserted_persona.name,
                inserted_persona.description,
                inserted_persona.avatar_uid,
            )
            .map_err(SavePersonaPortError::InternalError),
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
