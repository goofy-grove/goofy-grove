#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainValidationError {
    IdValidationError(String),
    NameValidationError(String),
    PasswordValidationError(String),
}

pub type DomainValidationResult = Result<(), DomainValidationError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainQueryError {
    NotFound,
    InternalError(String),
}

pub type DomainQueryResult<T> = Result<T, DomainQueryError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError<T> {
    ValidationError(DomainValidationError),
    QueryError(DomainQueryError),
    UseCaseError(T),
    ExternalServiceError(T),
}

pub type DomainResult<T, E> = Result<T, DomainError<E>>;
