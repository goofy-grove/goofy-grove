mod auth;
mod error;
mod macros;
mod person;
mod ports;
mod user;
mod validator;

pub mod prelude {
    pub use crate::domain::error::*;
    pub use crate::domain::validator::Validator;

    pub use crate::domain::user::entities::*;
    pub use crate::domain::user::ports::*;

    pub use crate::domain::auth::entities::*;
    pub use crate::domain::auth::ports::*;

    pub use crate::domain::person::entities::*;
    pub use crate::domain::person::ports::*;

    pub use crate::domain::ports::*;
}
