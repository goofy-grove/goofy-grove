use super::*;

#[derive(Clone)]
struct SaveDeviceOk;
impl SaveDevicePort for SaveDeviceOk {
    async fn create_device(&self, user_token: UserToken) -> Result<UserToken, SaveDevicePortError> {
        Ok(user_token)
    }
}

#[derive(Clone)]
struct SaveDeviceExists;
impl SaveDevicePort for SaveDeviceExists {
    async fn create_device(
        &self,
        _user_token: UserToken,
    ) -> Result<UserToken, SaveDevicePortError> {
        Err(SaveDevicePortError::DeviceAlreadyExists)
    }
}

#[derive(Clone)]
struct SaveDeviceInternalErr;
impl SaveDevicePort for SaveDeviceInternalErr {
    async fn create_device(
        &self,
        _user_token: UserToken,
    ) -> Result<UserToken, SaveDevicePortError> {
        Err(SaveDevicePortError::InternalError("db-down".into()))
    }
}

#[derive(Clone)]
struct InvalidateOk;
impl InvalidateDevicePort for InvalidateOk {
    async fn invalidate_device(
        &self,
        _hashed_token: &HashedToken,
    ) -> Result<(), InvalidateDevicePortError> {
        Ok(())
    }
}

#[derive(Clone)]
struct InvalidateNotFound;
impl InvalidateDevicePort for InvalidateNotFound {
    async fn invalidate_device(
        &self,
        _hashed_token: &HashedToken,
    ) -> Result<(), InvalidateDevicePortError> {
        Err(InvalidateDevicePortError::DeviceNotFound)
    }
}

#[derive(Clone)]
struct InvalidateInternalErr;
impl InvalidateDevicePort for InvalidateInternalErr {
    async fn invalidate_device(
        &self,
        _hashed_token: &HashedToken,
    ) -> Result<(), InvalidateDevicePortError> {
        Err(InvalidateDevicePortError::InternalError("db-down".into()))
    }
}

#[derive(Clone)]
struct HashTokenOk;
impl TokenHasherPort for HashTokenOk {
    async fn hash_token(&self, _token: Token) -> Result<HashedToken, TokenHasherPortError> {
        HashedToken::try_new("hashed-token".to_string())
            .map_err(|_| TokenHasherPortError::InternalError("invalid".into()))
    }
}

#[derive(Clone)]
struct HashTokenErr;
impl TokenHasherPort for HashTokenErr {
    async fn hash_token(&self, _token: Token) -> Result<HashedToken, TokenHasherPortError> {
        Err(TokenHasherPortError::InternalError("hash-failed".into()))
    }
}

#[derive(Clone)]
struct FixedId;
impl IdGenerator for FixedId {
    fn generate(&self) -> String {
        "token-id-1".to_string()
    }
}

#[derive(Clone)]
struct FixedClock;
impl Clock for FixedClock {
    fn timestamp(&self) -> i64 {
        42
    }
}

fn create_command() -> CreateDeviceCommand {
    CreateDeviceCommand {
        token: Token::try_new("raw-token".to_string()).unwrap(),
        user_agent: UserAgent::new("browser".to_string()),
        user_id: UserId::try_new("user-1".to_string()).unwrap(),
    }
}

#[tokio::test]
async fn create_device_successfully_persists_token() {
    let service = CreateDeviceService::new(SaveDeviceOk, HashTokenOk, FixedId, FixedClock);

    assert!(service.create_device(create_command()).await.is_ok());
}

#[tokio::test]
async fn create_device_maps_duplicate_error() {
    let service = CreateDeviceService::new(SaveDeviceExists, HashTokenOk, FixedId, FixedClock);

    assert!(matches!(
        service.create_device(create_command()).await,
        Err(CreateDeviceError::DeviceAlreadyExists)
    ));
}

#[tokio::test]
async fn create_device_maps_internal_storage_error() {
    let service = CreateDeviceService::new(SaveDeviceInternalErr, HashTokenOk, FixedId, FixedClock);

    assert!(matches!(
        service.create_device(create_command()).await,
        Err(CreateDeviceError::InternalError(_))
    ));
}

#[tokio::test]
async fn create_device_maps_hashing_error() {
    let service = CreateDeviceService::new(SaveDeviceOk, HashTokenErr, FixedId, FixedClock);

    assert!(matches!(
        service.create_device(create_command()).await,
        Err(CreateDeviceError::InternalError(_))
    ));
}

#[tokio::test]
async fn invalidate_device_success() {
    let service = InvalidateDeviceService::new(InvalidateOk, HashTokenOk);
    let command = InvalidateDeviceCommand {
        token: Token::try_new("raw-token".to_string()).unwrap(),
    };

    assert!(service.invalidate_device(command).await.is_ok());
}

#[tokio::test]
async fn invalidate_device_maps_not_found_error() {
    let service = InvalidateDeviceService::new(InvalidateNotFound, HashTokenOk);
    let command = InvalidateDeviceCommand {
        token: Token::try_new("raw-token".to_string()).unwrap(),
    };

    assert!(matches!(
        service.invalidate_device(command).await,
        Err(InvalidateDeviceError::DeviceNotFound)
    ));
}

#[tokio::test]
async fn invalidate_device_maps_internal_storage_error() {
    let service = InvalidateDeviceService::new(InvalidateInternalErr, HashTokenOk);
    let command = InvalidateDeviceCommand {
        token: Token::try_new("raw-token".to_string()).unwrap(),
    };

    assert!(matches!(
        service.invalidate_device(command).await,
        Err(InvalidateDeviceError::InternalError(_))
    ));
}

#[tokio::test]
async fn invalidate_device_maps_hashing_error() {
    let service = InvalidateDeviceService::new(InvalidateOk, HashTokenErr);
    let command = InvalidateDeviceCommand {
        token: Token::try_new("raw-token".to_string()).unwrap(),
    };

    assert!(matches!(
        service.invalidate_device(command).await,
        Err(InvalidateDeviceError::InternalError(_))
    ));
}
