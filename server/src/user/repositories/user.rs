use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, sea_query,
};
use thiserror::Error;

use crate::infra::db::entities::{prelude::Users, users};

#[derive(Debug, Clone, Error)]
pub enum LoadUserError {
    #[error("User not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone)]
pub struct User {
    pub uid: String,
    pub name: String,
    pub password: String,
    pub avatar_uid: Option<String>,
}

impl From<users::Model> for User {
    fn from(model: users::Model) -> Self {
        Self {
            uid: model.uid,
            name: model.name,
            password: model.password,
            avatar_uid: model.avatar_uid,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserRepository {
    connection: DatabaseConnection,
}

impl UserRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    async fn load_user_by_name(&self, name: &str) -> Result<User, LoadUserError> {
        let result = Users::find()
            .filter(users::Column::Name.eq(name))
            .one(&self.connection)
            .await;

        match result {
            Ok(Some(user)) => Ok(user.into()),
            Ok(None) => Err(LoadUserError::NotFound),
            Err(err) => Err(LoadUserError::InternalError(err.to_string())),
        }
    }

    async fn load_user_by_id(&self, user_id: &str) -> Result<User, LoadUserError> {
        let result = Users::find()
            .filter(users::Column::Uid.eq(user_id))
            .one(&self.connection)
            .await;

        match result {
            Ok(Some(user)) => Ok(user.into()),
            Ok(None) => Err(LoadUserError::NotFound),
            Err(err) => Err(LoadUserError::InternalError(err.to_string())),
        }
    }

    async fn save_user(&self, user: User) -> Result<User, LoadUserError> {
        let User {
            uid,
            name,
            password,
            avatar_uid,
        } = user;

        let new_user = users::ActiveModel {
            uid: Set(uid),
            name: Set(name),
            password: Set(password),
            avatar_uid: Set(avatar_uid),
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
            Ok(user) => Ok(user.into()),
            Err(err) => Err(LoadUserError::InternalError(err.to_string())),
        }
    }
}
