use crate::impl_new_type;

#[cfg(test)]
mod tests;

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TokenExpires(usize);
);
