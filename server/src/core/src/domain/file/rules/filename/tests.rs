use super::*;

#[test]
fn filename_trims_value() {
    assert_eq!(
        Filename::try_new("  f-1.bin ".to_string()).unwrap().inner(),
        "f-1.bin"
    );
}

#[test]
fn filename_rejects_empty() {
    assert_eq!(
        Filename::try_new(" ".to_string()).unwrap_err(),
        FilenameValidationError::Empty
    );
}
