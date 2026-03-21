use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct GetUserByNameService<L: LoadUserByNamePort> {
    load_user_by_name_port: L,
}

impl<L: LoadUserByNamePort> GetUserByNameService<L> {
    pub fn new(load_user_by_name_port: L) -> Self {
        Self {
            load_user_by_name_port,
        }
    }
}

impl<L: LoadUserByNamePort> GetUserByNameQuery for GetUserByNameService<L> {
    async fn get_user_by_name(&self, id: &UserName) -> DomainResult<User, GetUserByNameError> {
        self.load_user_by_name_port
            .load_user_by_name(id)
            .await
            .map_err(|err| match err {
                DomainError::ExternalServiceError(LoadUserByNamePortError::NotFound) => {
                    DomainError::QueryError(DomainQueryError::NotFound)
                }
                err => DomainError::UseCaseError(GetUserByNameError::InternalError(format!(
                    "{:?}",
                    err
                ))),
            })
    }
}
