#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileStatus {
    Created,
    Activated,
    Orphaned,
}
