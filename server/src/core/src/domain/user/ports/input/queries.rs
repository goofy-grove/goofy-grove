use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub enum GetUserByNameError {
    InternalError(String),
    UserNotFound,
}

pub trait GetUserByNameQuery {
    fn get_user_by_name(
        &self,
        username: &UserName,
    ) -> impl Future<Output = DomainResult<User, GetUserByNameError>>;
}
