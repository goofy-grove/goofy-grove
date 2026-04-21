use crate::{domain::prelude::*, generate_entity};

#[cfg(test)]
mod tests;

generate_entity!(TokenData {
    uid: UserId,
    username: Username,
    expires_at: TokenExpires
});

generate_entity!(UserToken {
    uid: TokenId,
    hashed_token: HashedToken,
    user_id: UserId,
    user_agent: UserAgent,
    last_accessed_at: LastAccessedAt
});
