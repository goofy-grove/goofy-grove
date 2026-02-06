use crate::prelude::{DomainQueryResult, User, UserId};

pub trait GetUserByNameQuery {
    fn get_user_by_name(&self, name: &UserId) -> impl Future<Output = DomainQueryResult<User>>;
}
