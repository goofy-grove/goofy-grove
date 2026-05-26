use super::*;

#[test]
fn file_id_trims_value() {
    assert_eq!(
        FileId::try_new("  f-1 ".to_string()).unwrap().inner(),
        "f-1"
    );
}

#[test]
fn file_id_rejects_empty() {
    assert_eq!(
        FileId::try_new(" ".to_string()).unwrap_err(),
        FileIdValidationError::Empty
    );
}
