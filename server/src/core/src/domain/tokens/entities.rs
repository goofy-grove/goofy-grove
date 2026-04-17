use crate::{domain::prelude::UserId, generate_entity, impl_as_domain_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token(String);

impl_as_domain_newtype!(Token -> String);

generate_entity!(TokenData {
    uid: String,
    username: String,
    expires_at: usize
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashedToken(String);

impl_as_domain_newtype!(HashedToken -> String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenId(String);

impl_as_domain_newtype!(TokenId -> String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserAgent(String);

impl_as_domain_newtype!(UserAgent -> String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LastAccessedAt(i64);

impl_as_domain_newtype!(LastAccessedAt -> i64);

generate_entity!(UserToken {
    uid: TokenId,
    hashed_token: HashedToken,
    user_id: UserId,
    user_agent: UserAgent,
    last_accessed_at: LastAccessedAt
});
