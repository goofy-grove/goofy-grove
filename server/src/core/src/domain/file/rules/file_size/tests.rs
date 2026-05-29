use super::*;

#[test]
fn file_size_accepts_positive() {
    assert_eq!(FileSize::try_new(1).unwrap().inner(), &1);
}

#[test]
fn file_size_rejects_zero() {
    assert_eq!(
        FileSize::try_new(0).unwrap_err(),
        FileSizeValidationError::ZeroSize
    );
}
