use crate::{generate_entity, impl_as_domain_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Secret(String);

impl_as_domain_newtype!(Secret -> String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token(String);

impl_as_domain_newtype!(Token -> String);

generate_entity!(TokenData { username: String });
