mod auth;
mod error;
mod macros;
mod person;
mod user;
mod validator;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::validator::Validator;

    pub use crate::user::entities::*;
    pub use crate::user::ports::*;

    pub use crate::auth::entities::*;
    pub use crate::auth::ports::*;

    pub use crate::person::entities::*;
    pub use crate::person::ports::*;
}
