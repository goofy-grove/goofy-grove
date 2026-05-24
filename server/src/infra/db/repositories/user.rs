use gg_core::domain::prelude::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

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
    async fn load_user_by_name(&self, name: &Username) -> Result<User, LoadUserByNamePortError> {
        let result = Users::find()
            .filter(users::Column::Name.eq(name.inner()))
            .one(&self.connection)
            .await;

        match result {
            Ok(Some(user)) => Ok(User {
                uid: UserId::try_new(user.uid)
                    .map_err(|err| LoadUserByNamePortError::InternalError(err.to_string()))?,
                name: Username::try_new(user.name)
                    .map_err(|err| LoadUserByNamePortError::InternalError(err.to_string()))?,
                password: UserPassword::try_new(user.password)
                    .map_err(|err| LoadUserByNamePortError::InternalError(err.to_string()))?,
            }),
            Ok(None) => Err(LoadUserByNamePortError::NotFound),
            Err(err) => Err(LoadUserByNamePortError::InternalError(err.to_string())),
        }
    }
}

impl SaveUserPort for UserRepository {
    async fn save_user(&self, user: User) -> Result<User, SaveUserPortError> {
        let User {
            uid,
            name,
            password,
        } = user;

        let new_user = users::ActiveModel {
            uid: Set(uid.into_inner()),
            name: Set(name.into_inner()),
            password: Set(password.into_inner()),
        };

        match new_user.insert(&self.connection).await {
            Ok(inserted_user) => Ok(User {
                uid: UserId::try_new(inserted_user.uid)
                    .map_err(|err| SaveUserPortError::InternalError(err.to_string()))?,
                name: Username::try_new(inserted_user.name)
                    .map_err(|err| SaveUserPortError::InternalError(err.to_string()))?,
                password: UserPassword::try_new(inserted_user.password)
                    .map_err(|err| SaveUserPortError::InternalError(err.to_string()))?,
            }),
            Err(err) => Err(SaveUserPortError::InternalError(err.to_string())),
        }
    }
}
