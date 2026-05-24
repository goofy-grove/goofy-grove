use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct User {
    pub uid: UserId,
    pub name: Username,
    pub password: UserPassword,
}
