use crate::prelude::{DomainResult, User, UserName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadUserByNamePortError {
    InternalError(String),
    NotFound,
}

pub trait LoadUserByNamePort {
    fn load_user_by_name(
        &self,
        name: &UserName,
    ) -> impl Future<Output = DomainResult<User, LoadUserByNamePortError>>;
}
