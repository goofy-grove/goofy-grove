use domain::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::infra::db::entities::{prelude::Users, users};

#[derive(Debug, Clone)]
pub struct UserRepository {
    connection: DatabaseConnection,
}

impl UserRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl LoadUserByNamePort for UserRepository {
    async fn load_user_by_name(
        &self,
        name: &UserName,
    ) -> DomainResult<User, LoadUserByNamePortError> {
        let result = Users::find()
            .filter(users::Column::Name.eq(name.value()))
            .one(&self.connection)
            .await;

        match result {
            Ok(Some(user)) => Ok(User::new(
                UserId::new(user.uid),
                UserName::new(user.name),
                UserPassword::new(user.password),
            )),
            Ok(None) => Err(DomainError::ExternalServiceError(
                LoadUserByNamePortError::NotFound,
            )),
            Err(err) => Err(DomainError::ExternalServiceError(
                LoadUserByNamePortError::InternalError(err.to_string()),
            )),
        }
    }
}
