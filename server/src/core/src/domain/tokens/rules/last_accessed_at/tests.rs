use super::*;

#[test]
fn last_accessed_at_accepts_positive_timestamp() {
    assert_eq!(LastAccessedAt::try_new(10).unwrap().inner(), &10);
}

#[test]
fn last_accessed_at_rejects_non_positive_timestamp() {
    assert_eq!(
        LastAccessedAt::try_new(0).unwrap_err(),
        LastAccessedAtValidationError::NonPositive
    );
}
