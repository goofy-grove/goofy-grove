use thiserror::Error;

use crate::domain::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum GetUserByNameError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("User not found")]
    UserNotFound,
}

pub trait GetUserByNameQuery {
    fn get_user_by_name(
        &self,
        username: &UserName,
    ) -> impl Future<Output = DomainResult<User, GetUserByNameError>>;
}
