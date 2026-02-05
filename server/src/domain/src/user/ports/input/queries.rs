use crate::prelude::{DomainQueryResult, User, UserId};

pub trait GetUserByNameQuery {
    async fn get_user_by_name(&self, name: &UserId) -> DomainQueryResult<User>;
}
