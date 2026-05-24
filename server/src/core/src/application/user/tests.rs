use super::*;

#[derive(Clone)]
struct LoadUserOk {
    user: User,
}
impl LoadUserByNamePort for LoadUserOk {
    async fn load_user_by_name(&self, _name: &Username) -> Result<User, LoadUserByNamePortError> {
        Ok(self.user.clone())
    }
}

#[derive(Clone)]
struct LoadUserNotFound;
impl LoadUserByNamePort for LoadUserNotFound {
    async fn load_user_by_name(&self, _name: &Username) -> Result<User, LoadUserByNamePortError> {
        Err(LoadUserByNamePortError::NotFound)
    }
}

#[derive(Clone)]
struct LoadUserInternalErr;
impl LoadUserByNamePort for LoadUserInternalErr {
    async fn load_user_by_name(&self, _name: &Username) -> Result<User, LoadUserByNamePortError> {
        Err(LoadUserByNamePortError::InternalError("db-down".into()))
    }
}

#[tokio::test]
async fn get_user_by_name_returns_user() {
    let user = User {
        uid: UserId::try_new("user-1".to_string()).unwrap(),
        name: Username::try_new("john".to_string()).unwrap(),
        password: UserPassword::try_new("hashed".to_string()).unwrap(),
    };
    let service = GetUserByNameService::new(LoadUserOk { user });

    assert!(
        service
            .get_user_by_name(&Username::try_new("john".to_string()).unwrap())
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn get_user_by_name_maps_not_found() {
    let service = GetUserByNameService::new(LoadUserNotFound);

    assert!(matches!(
        service
            .get_user_by_name(&Username::try_new("ghost".to_string()).unwrap())
            .await,
        Err(GetUserByNameError::UserNotFound)
    ));
}

#[tokio::test]
async fn get_user_by_name_maps_internal_error() {
    let service = GetUserByNameService::new(LoadUserInternalErr);

    assert!(matches!(
        service
            .get_user_by_name(&Username::try_new("ghost".to_string()).unwrap())
            .await,
        Err(GetUserByNameError::InternalError(_))
    ));
}
