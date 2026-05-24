use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct TokenData {
    pub uid: UserId,
    pub username: Username,
    pub expires_at: TokenExpires,
}

#[derive(Debug, Clone)]
pub struct UserToken {
    pub uid: TokenId,
    pub hashed_token: HashedToken,
    pub user_id: UserId,
    pub user_agent: UserAgent,
    pub last_accessed_at: LastAccessedAt,
}
