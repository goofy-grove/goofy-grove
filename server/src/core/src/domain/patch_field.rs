#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatchField<T> {
    Unchanged,
    Clear,
    Set(T),
}

impl<T> PatchField<T> {
    pub fn is_unchanged(&self) -> bool {
        matches!(self, Self::Unchanged)
    }

    pub fn from_optional_option(value: Option<Option<T>>) -> Self {
        match value {
            None => Self::Unchanged,
            Some(None) => Self::Clear,
            Some(Some(value)) => Self::Set(value),
        }
    }
}
