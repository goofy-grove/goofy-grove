#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatchField<T> {
    Unchanged,
    Clear,
    Set(T),
}
