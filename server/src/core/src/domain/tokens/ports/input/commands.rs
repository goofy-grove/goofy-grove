use crate::domain::prelude::*;

#[derive(Debug, Clone)]
pub struct CreateDeviceCommand {
    pub token: Token,
    pub user_agent: UserAgent,
    pub user_id: UserId,
}

#[derive(Debug, Clone)]
pub struct InvalidateDeviceCommand {
    pub token: Token,
}
