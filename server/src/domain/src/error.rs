#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    IdValidationError(String),
    NameValidationError(String),
    PasswordValidationError(String),
}
