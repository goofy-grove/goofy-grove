use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct GetDeviceService<T: LoadDevicePort, H: TokenHasherPort> {
    load_device_port: T,
    token_hasher_port: H,
}

impl<T: LoadDevicePort, H: TokenHasherPort> GetDeviceService<T, H> {
    pub fn new(load_device_port: T, token_hasher_port: H) -> Self {
        Self {
            load_device_port,
            token_hasher_port,
        }
    }
}

impl<T: LoadDevicePort, H: TokenHasherPort> GetDeviceQuery for GetDeviceService<T, H> {
    async fn get_device(
        &self,
        token: &Token,
    ) -> DomainResult<UserToken, GetDeviceError> {
        let hashed_token = self
            .token_hasher_port
            .hash_token(token.to_owned())
            .await
            .map_err(|err| {
                DomainError::UseCaseError(GetDeviceError::InternalError(format!("{:?}", err)))
            })?;

        self.load_device_port
            .load_device(&hashed_token)
            .await
            .map_err(|err| match err {
                DomainError::ExternalServiceError(LoadDevicePortError::DeviceNotFound) => {
                    DomainError::QueryError(DomainQueryError::NotFound)
                }
                err => {
                    DomainError::UseCaseError(GetDeviceError::InternalError(format!("{:?}", err)))
                }
            })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidateDeviceService<T: InvalidateDevicePort, H: TokenHasherPort> {
    invalidate_device_port: T,
    token_hasher_port: H,
}

impl<T: InvalidateDevicePort, H: TokenHasherPort> InvalidateDeviceService<T, H> {
    pub fn new(invalidate_device_port: T, token_hasher_port: H) -> Self {
        Self {
            invalidate_device_port,
            token_hasher_port,
        }
    }
}

impl<T: InvalidateDevicePort, H: TokenHasherPort> InvalidateDeviceUseCase
    for InvalidateDeviceService<T, H>
{
    async fn invalidate_device(
        &self,
        command: InvalidateDeviceCommand,
    ) -> DomainResult<(), InvalidateDeviceError> {
        let hashed_token = self
            .token_hasher_port
            .hash_token(command.token().to_owned())
            .await
            .map_err(|err| {
                DomainError::UseCaseError(InvalidateDeviceError::InternalError(format!(
                    "{:?}",
                    err
                )))
            })?;

        self.invalidate_device_port
            .invalidate_device(&hashed_token)
            .await
            .map_err(|err| match err {
                DomainError::ExternalServiceError(InvalidateDevicePortError::DeviceNotFound) => {
                    DomainError::UseCaseError(InvalidateDeviceError::DeviceNotFound)
                }
                err => DomainError::UseCaseError(InvalidateDeviceError::InternalError(format!(
                    "{:?}",
                    err
                ))),
            })
    }
}

#[derive(Debug, Clone)]
pub struct CreateDeviceService<S: SaveDevicePort, G: IdGenerator, C: Clock, H: TokenHasherPort> {
    create_device_port: S,
    token_hasher_port: H,
    id_generator: G,
    clock: C,
}

impl<S: SaveDevicePort, G: IdGenerator, C: Clock, H: TokenHasherPort>
    CreateDeviceService<S, G, C, H>
{
    pub fn new(create_device_port: S, token_hasher_port: H, id_generator: G, clock: C) -> Self {
        Self {
            create_device_port,
            token_hasher_port,
            id_generator,
            clock,
        }
    }
}

impl<S: SaveDevicePort, G: IdGenerator, C: Clock, H: TokenHasherPort> CreateDeviceUseCase
    for CreateDeviceService<S, G, C, H>
{
    async fn create_device(
        &self,
        command: CreateDeviceCommand,
    ) -> DomainResult<UserToken, CreateDeviceError> {
        let hashed_token = self
            .token_hasher_port
            .hash_token(command.token().to_owned())
            .await
            .map_err(|err| {
                DomainError::UseCaseError(CreateDeviceError::InternalError(format!("{:?}", err)))
            })?;

        let token = UserToken::new(
            TokenId::new(self.id_generator.generate()),
            hashed_token,
            command.user_id().to_owned(),
            command.user_agent().to_owned(),
            LastAccessedAt::new(self.clock.timestamp()),
        );

        self.create_device_port
            .create_device(token)
            .await
            .map_err(|err| match err {
                DomainError::ExternalServiceError(SaveDevicePortError::DeviceAlreadyExists) => {
                    DomainError::UseCaseError(CreateDeviceError::DeviceAlreadyExists)
                }
                err => DomainError::UseCaseError(CreateDeviceError::InternalError(format!(
                    "{:?}",
                    err
                ))),
            })
    }
}
