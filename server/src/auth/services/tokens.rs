use thiserror::Error;

use crate::{
    app::AppDeps,
    auth::services::{
        device::{RegisterDeviceError, RegisterDeviceInput, register_device},
        jwt::{UserTokenInput, generate_token},
    },
    user::public::User,
};

#[derive(Debug, Clone, Error)]
pub enum GenerateTokensError {
    #[error("failed_to_generate_access_token")]
    GenerateAccessToken,

    #[error("failed_to_generate_refresh_token")]
    GenerateRefreshToken,

    #[error("failed_to_create_user_device")]
    CreateUserDevice,
}

pub struct GeneratedTokens {
    pub access_token: String,
    pub access_exp: usize,
    pub refresh_token: String,
    pub refresh_max_age: usize,
}

pub async fn generate_tokens(
    deps: &AppDeps,
    user: &User,
) -> Result<GeneratedTokens, GenerateTokensError> {
    let uid = user.uid.clone();
    let username = user.name.clone();

    let (access_token, access_exp) = generate_token(
        UserTokenInput {
            uid: uid.clone(),
            username: username.clone(),
        },
        &deps.config.jwt.access_token,
    )
    .map_err(|_| GenerateTokensError::GenerateAccessToken)?;

    let (refresh_token, _) = generate_token(
        UserTokenInput { uid, username },
        &deps.config.jwt.refresh_token,
    )
    .map_err(|_| GenerateTokensError::GenerateRefreshToken)?;

    let refresh_max_age = deps.config.jwt.refresh_token.expiration_time as usize;

    register_device(
        deps,
        RegisterDeviceInput {
            refresh_token: refresh_token.clone(),
            user_uid: user.uid.clone(),
            user_agent: String::new(),
        },
    )
    .await
    .map_err(|err| match err {
        RegisterDeviceError::DeviceAlreadyExists | RegisterDeviceError::InternalError(_) => {
            GenerateTokensError::CreateUserDevice
        }
    })?;

    Ok(GeneratedTokens {
        access_token,
        access_exp,
        refresh_token,
        refresh_max_age,
    })
}
