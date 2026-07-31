use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, sea_query,
};
use serde::Serialize;
use thiserror::Error;

use crate::platform::database::entities::{prelude::Users, users};

#[derive(Debug, Clone, Error)]
pub enum LoadUserError {
    #[error("User not found")]
    NotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub uid: String,
    #[serde(rename = "username")]
    pub name: String,
    pub avatar_uid: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserCredentials {
    pub user_uid: String,
    pub password_hash: String,
}

impl From<users::Model> for (User, UserCredentials) {
    fn from(model: users::Model) -> Self {
        let credentials = UserCredentials {
            user_uid: model.uid.clone(),
            password_hash: model.password,
        };
        let user = User {
            uid: model.uid,
            name: model.name,
            avatar_uid: model.avatar_uid,
        };

        (user, credentials)
    }
}

pub async fn load_user_by_name(
    connection: &impl ConnectionTrait,
    name: &str,
) -> Result<(User, UserCredentials), LoadUserError> {
    let result = Users::find()
        .filter(users::Column::Name.eq(name))
        .one(connection)
        .await;

    match result {
        Ok(Some(user)) => Ok(user.into()),
        Ok(None) => Err(LoadUserError::NotFound),
        Err(err) => Err(LoadUserError::InternalError(err.to_string())),
    }
}

pub async fn load_user_by_uid(
    connection: &impl ConnectionTrait,
    user_uid: &str,
) -> Result<(User, UserCredentials), LoadUserError> {
    let result = Users::find()
        .filter(users::Column::Uid.eq(user_uid))
        .one(connection)
        .await;

    match result {
        Ok(Some(user)) => Ok(user.into()),
        Ok(None) => Err(LoadUserError::NotFound),
        Err(err) => Err(LoadUserError::InternalError(err.to_string())),
    }
}

pub async fn save_user(
    connection: &impl ConnectionTrait,
    user: User,
    credentials: UserCredentials,
) -> Result<User, LoadUserError> {
    if user.uid != credentials.user_uid {
        return Err(LoadUserError::InternalError(
            "user uid and credentials user_uid mismatch".into(),
        ));
    }

    let User {
        uid,
        name,
        avatar_uid,
    } = user;

    let new_user = users::ActiveModel {
        uid: Set(uid),
        name: Set(name),
        password: Set(credentials.password_hash),
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
        .exec_with_returning(connection)
        .await
    {
        Ok(model) => {
            let (user, _) = model.into();
            Ok(user)
        }
        Err(err) => Err(LoadUserError::InternalError(err.to_string())),
    }
}
