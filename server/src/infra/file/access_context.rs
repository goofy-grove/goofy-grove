use gg_core::domain::prelude::*;
use sea_orm::DatabaseConnection;

use crate::infra::db::PersonaRepository;

#[derive(Debug, Clone)]
pub struct FileAccessContextLoader {
    persona_repository: PersonaRepository,
}

impl FileAccessContextLoader {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self {
            persona_repository: PersonaRepository::new(connection),
        }
    }
}

impl LoadFileCreateAccessContextPort for FileAccessContextLoader {
    async fn load_create_context(
        &self,
        scope: &FileScope,
        user_id: &UserId,
    ) -> Result<FileCreateAccessContext, LoadFileCreateAccessContextPortError> {
        match scope {
            FileScope::UserAvatar { .. } => Ok(FileCreateAccessContext::UserAvatar),
            FileScope::PersonaAvatar { persona_id, .. } => {
                let persona_owned_by_actor = self
                    .persona_repository
                    .load_persona(persona_id, user_id)
                    .await
                    .is_ok();

                Ok(FileCreateAccessContext::PersonaAvatar {
                    persona_owned_by_actor,
                })
            }
        }
    }
}

impl LoadFileMetaAccessContextPort for FileAccessContextLoader {
    async fn load_meta_access_context(
        &self,
        meta: &FileMeta,
        user_id: &UserId,
    ) -> Result<FileMetaAccessContext, LoadFileMetaAccessContextPortError> {
        match &meta.scope {
            FileScope::UserAvatar { .. } => Ok(FileMetaAccessContext::UserAvatar),
            FileScope::PersonaAvatar { persona_id, .. } => {
                let persona_owned_by_actor = self
                    .persona_repository
                    .load_persona(persona_id, user_id)
                    .await
                    .is_ok();

                Ok(FileMetaAccessContext::PersonaAvatar {
                    persona_owned_by_actor,
                })
            }
        }
    }
}
