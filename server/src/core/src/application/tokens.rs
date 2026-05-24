use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

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
    ) -> Result<(), InvalidateDeviceError> {
        let InvalidateDeviceCommand { token } = command;

        let hashed_token = self
            .token_hasher_port
            .hash_token(token)
            .await
            .map_err(|err| InvalidateDeviceError::InternalError(format!("{:?}", err)))?;

        self.invalidate_device_port
            .invalidate_device(&hashed_token)
            .await
            .map_err(|err| match err {
                InvalidateDevicePortError::DeviceNotFound => InvalidateDeviceError::DeviceNotFound,
                err => InvalidateDeviceError::InternalError(format!("{:?}", err)),
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
    ) -> Result<UserToken, CreateDeviceError> {
        let CreateDeviceCommand {
            token,
            user_id,
            user_agent,
        } = command;

        let hashed_token = self
            .token_hasher_port
            .hash_token(token)
            .await
            .map_err(|err| CreateDeviceError::InternalError(format!("{:?}", err)))?;

        let token = UserToken {
            uid: TokenId::try_new(self.id_generator.generate())
                .map_err(|err| CreateDeviceError::ValidationError(format!("{err}")))?,
            hashed_token,
            user_id,
            user_agent,
            last_accessed_at: LastAccessedAt::try_new(self.clock.timestamp())
                .map_err(|err| CreateDeviceError::ValidationError(format!("{err}")))?,
        };

        self.create_device_port
            .create_device(token)
            .await
            .map_err(|err| match err {
                SaveDevicePortError::DeviceAlreadyExists => CreateDeviceError::DeviceAlreadyExists,
                err => CreateDeviceError::InternalError(format!("{:?}", err)),
            })
    }
}
