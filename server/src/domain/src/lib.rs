mod error;
mod user;
mod validator;

pub mod prelude {
    pub use crate::error::DomainError;
    pub use crate::user::entities::*;
    pub use crate::validator::{DomainValidationResult, Validator};
}
