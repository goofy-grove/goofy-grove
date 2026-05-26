use super::*;

#[test]
fn file_original_name_trims_value() {
    assert_eq!(
        FileOriginalName::try_new("  avatar.png ".to_string())
            .unwrap()
            .inner(),
        "avatar.png"
    );
}

#[test]
fn file_original_name_rejects_empty() {
    assert_eq!(
        FileOriginalName::try_new(" ".to_string()).unwrap_err(),
        FileOriginalNameValidationError::Empty
    );
}
