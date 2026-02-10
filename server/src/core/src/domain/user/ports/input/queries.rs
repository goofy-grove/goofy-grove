use crate::domain::prelude::*;

pub trait GetUserByNameQuery {
    fn get_user_by_name(&self, name: &UserId) -> impl Future<Output = DomainQueryResult<User>>;
}
