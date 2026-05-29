use super::*;

#[test]
fn uploaded_at_accepts_positive_timestamp() {
    assert_eq!(UploadedAt::try_new(42).unwrap().inner(), &42);
}

#[test]
fn uploaded_at_rejects_zero_timestamp() {
    assert_eq!(
        UploadedAt::try_new(0).unwrap_err(),
        UploadedAtValidationError::NonPositive
    );
}

#[test]
fn uploaded_at_rejects_negative_timestamp() {
    assert_eq!(
        UploadedAt::try_new(-1).unwrap_err(),
        UploadedAtValidationError::NonPositive
    );
}
