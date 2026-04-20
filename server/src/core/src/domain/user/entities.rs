use crate::{domain::prelude::*, generate_entity};

generate_entity!(User {
    uid: UserId,
    name: Username,
    password: UserPassword
});
