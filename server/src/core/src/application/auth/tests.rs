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

#[derive(Clone)]
struct VerifyOk;
impl PasswordVerifierPort for VerifyOk {
    async fn verify(
        &self,
        _proposed_password: &Secret,
        _confirmed_password: &Secret,
    ) -> Result<(), PasswordVerifierPortError> {
        Ok(())
    }
}

#[derive(Clone)]
struct VerifyFail;
impl PasswordVerifierPort for VerifyFail {
    async fn verify(
        &self,
        _proposed_password: &Secret,
        _confirmed_password: &Secret,
    ) -> Result<(), PasswordVerifierPortError> {
        Err(PasswordVerifierPortError::PasswordNotMatch)
    }
}

#[derive(Clone)]
struct SaveUserOk;
impl SaveUserPort for SaveUserOk {
    async fn save_user(&self, user: &User) -> Result<User, SaveUserPortError> {
        Ok(user.clone())
    }
}

#[derive(Clone)]
struct SaveUserExists;
impl SaveUserPort for SaveUserExists {
    async fn save_user(&self, _user: &User) -> Result<User, SaveUserPortError> {
        Err(SaveUserPortError::UserAlreadyExists)
    }
}

#[derive(Clone)]
struct SaveUserInternalErr;
impl SaveUserPort for SaveUserInternalErr {
    async fn save_user(&self, _user: &User) -> Result<User, SaveUserPortError> {
        Err(SaveUserPortError::InternalError("db-down".into()))
    }
}

#[derive(Clone)]
struct HashOk;
impl PasswordHasherPort for HashOk {
    async fn hash(&self, _password: &Secret) -> Result<Secret, PasswordHasherPortError> {
        Secret::try_new("hashed".to_string())
            .map_err(|_| PasswordHasherPortError::InternalError("invalid".into()))
    }
}

#[derive(Clone)]
struct HashErr;
impl PasswordHasherPort for HashErr {
    async fn hash(&self, _password: &Secret) -> Result<Secret, PasswordHasherPortError> {
        Err(PasswordHasherPortError::InternalError("hash-failed".into()))
    }
}

#[derive(Clone)]
struct FixedId;
impl IdGenerator for FixedId {
    fn generate(&self) -> String {
        "user-id-1".to_string()
    }
}

fn sample_user() -> User {
    User::new(
        UserId::try_new("user-id-1".to_string()).unwrap(),
        Username::try_new("john".to_string()).unwrap(),
        UserPassword::try_new("hashed".to_string()).unwrap(),
    )
}

#[tokio::test]
async fn authorize_returns_user_for_valid_credentials() {
    let service = UserAuthorizationService::new(
        LoadUserOk {
            user: sample_user(),
        },
        VerifyOk,
    );
    let command = AuthorizationCommand::new(
        Username::try_new("john".to_string()).unwrap(),
        Secret::try_new("plain".to_string()).unwrap(),
    );

    assert!(service.authorize(command).await.is_ok());
}

#[tokio::test]
async fn authorize_returns_not_found_when_user_missing() {
    let service = UserAuthorizationService::new(LoadUserNotFound, VerifyOk);
    let command = AuthorizationCommand::new(
        Username::try_new("ghost".to_string()).unwrap(),
        Secret::try_new("plain".to_string()).unwrap(),
    );

    assert!(matches!(
        service.authorize(command).await,
        Err(AuthorizationError::UserNotFound)
    ));
}

#[tokio::test]
async fn authorize_maps_load_user_internal_error_to_user_not_found() {
    let service = UserAuthorizationService::new(LoadUserInternalErr, VerifyOk);
    let command = AuthorizationCommand::new(
        Username::try_new("john".to_string()).unwrap(),
        Secret::try_new("plain".to_string()).unwrap(),
    );

    assert!(matches!(
        service.authorize(command).await,
        Err(AuthorizationError::UserNotFound)
    ));
}

#[tokio::test]
async fn authorize_returns_invalid_credentials_when_password_mismatch() {
    let service = UserAuthorizationService::new(
        LoadUserOk {
            user: sample_user(),
        },
        VerifyFail,
    );
    let command = AuthorizationCommand::new(
        Username::try_new("john".to_string()).unwrap(),
        Secret::try_new("wrong".to_string()).unwrap(),
    );

    assert!(matches!(
        service.authorize(command).await,
        Err(AuthorizationError::InvalidCredentials)
    ));
}

#[tokio::test]
async fn register_creates_user_on_success() {
    let service = RegistrationService::new(SaveUserOk, HashOk, FixedId);
    let command = RegistrationCommand::new(
        Username::try_new("john".to_string()).unwrap(),
        Secret::try_new("secret".to_string()).unwrap(),
    );

    assert!(service.register(command).await.is_ok());
}

#[tokio::test]
async fn register_maps_user_exists_error() {
    let service = RegistrationService::new(SaveUserExists, HashOk, FixedId);
    let command = RegistrationCommand::new(
        Username::try_new("john".to_string()).unwrap(),
        Secret::try_new("secret".to_string()).unwrap(),
    );

    assert!(matches!(
        service.register(command).await,
        Err(RegistrationError::UserAlreadyExists)
    ));
}

#[tokio::test]
async fn register_maps_internal_save_error() {
    let service = RegistrationService::new(SaveUserInternalErr, HashOk, FixedId);
    let command = RegistrationCommand::new(
        Username::try_new("john".to_string()).unwrap(),
        Secret::try_new("secret".to_string()).unwrap(),
    );

    assert!(matches!(
        service.register(command).await,
        Err(RegistrationError::InternalError(_))
    ));
}

#[tokio::test]
async fn register_maps_hashing_error() {
    let service = RegistrationService::new(SaveUserOk, HashErr, FixedId);
    let command = RegistrationCommand::new(
        Username::try_new("john".to_string()).unwrap(),
        Secret::try_new("secret".to_string()).unwrap(),
    );

    assert!(matches!(
        service.register(command).await,
        Err(RegistrationError::FailedToHashPassword)
    ));
}
