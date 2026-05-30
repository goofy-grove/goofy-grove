use gg_core::domain::prelude::*;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, sea_query,
};

use crate::infra::db::entities::{prelude::Users, users};
use crate::infra::db::mappers::user_from_model;

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
            Ok(Some(user)) => user_from_model(user.uid, user.name, user.password, user.avatar_uid)
                .map_err(LoadUserByNamePortError::InternalError),
            Ok(None) => Err(LoadUserByNamePortError::NotFound),
            Err(err) => Err(LoadUserByNamePortError::InternalError(err.to_string())),
        }
    }
}

impl LoadUserByIdPort for UserRepository {
    async fn load_user_by_id(&self, user_id: &UserId) -> Result<User, LoadUserByIdPortError> {
        let result = Users::find_by_id(user_id.inner())
            .one(&self.connection)
            .await;

        match result {
            Ok(Some(user)) => user_from_model(user.uid, user.name, user.password, user.avatar_uid)
                .map_err(LoadUserByIdPortError::InternalError),
            Ok(None) => Err(LoadUserByIdPortError::NotFound),
            Err(err) => Err(LoadUserByIdPortError::InternalError(err.to_string())),
        }
    }
}

impl SaveUserPort for UserRepository {
    async fn save_user(&self, user: User) -> Result<User, SaveUserPortError> {
        let User {
            uid,
            name,
            password,
            avatar_uid,
        } = user;

        let new_user = users::ActiveModel {
            uid: Set(uid.into_inner()),
            name: Set(name.into_inner()),
            password: Set(password.into_inner()),
            avatar_uid: Set(avatar_uid.map(|value| value.into_inner())),
        };

        match Users::insert(new_user)
            .on_conflict(
                sea_query::OnConflict::column(users::Column::Uid)
                    .update_columns([
                        users::Column::Name,
                        users::Column::Password,
                        users::Column::AvatarUid,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.connection)
            .await
        {
            Ok(inserted_user) => user_from_model(
                inserted_user.uid,
                inserted_user.name,
                inserted_user.password,
                inserted_user.avatar_uid,
            )
            .map_err(SaveUserPortError::InternalError),
            Err(err) => Err(SaveUserPortError::InternalError(err.to_string())),
        }
    }
}
