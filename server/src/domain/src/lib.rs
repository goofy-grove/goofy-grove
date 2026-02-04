mod entities;
mod validator;
mod error;

pub mod prelude {
    pub use crate::entities::User;
    pub use crate::validator::{DomainValidationResult, Validator};
    pub use crate::error::DomainError;
}
