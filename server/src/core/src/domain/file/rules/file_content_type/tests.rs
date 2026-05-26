use super::*;

#[test]
fn file_content_type_normalizes_to_lowercase() {
    assert_eq!(
        FileContentType::try_new("IMAGE/PNG".to_string())
            .unwrap()
            .inner(),
        "image/png"
    );
}

#[test]
fn file_content_type_strips_parameters_after_semicolon() {
    assert_eq!(
        FileContentType::try_new("image/png; charset=utf-8".to_string())
            .unwrap()
            .inner(),
        "image/png"
    );
}

#[test]
fn file_content_type_trims_main_mime_segment() {
    assert_eq!(
        FileContentType::try_new("image/jpeg ; charset=utf-8".to_string())
            .unwrap()
            .inner(),
        "image/jpeg"
    );
}

#[test]
fn file_content_type_rejects_empty_after_sanitize() {
    assert_eq!(
        FileContentType::try_new("   ".to_string()).unwrap_err(),
        FileContentTypeValidationError::Empty
    );
}

#[test]
fn file_content_type_rejects_only_parameters() {
    assert_eq!(
        FileContentType::try_new("; charset=utf-8".to_string()).unwrap_err(),
        FileContentTypeValidationError::Empty
    );
}
