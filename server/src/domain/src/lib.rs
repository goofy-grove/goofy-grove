mod error;
mod user;
mod validator;
mod macros;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::user::entities::*;
    pub use crate::validator::Validator;
}
