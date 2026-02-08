use crate::impl_as_domain_newtype;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Secret(String);

impl_as_domain_newtype!(Secret -> String);
