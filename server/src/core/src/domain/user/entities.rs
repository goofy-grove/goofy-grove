use crate::{domain::prelude::*, generate_entity};

#[cfg(test)]
mod tests;

generate_entity!(User {
    uid: UserId,
    name: Username,
    password: UserPassword
});
