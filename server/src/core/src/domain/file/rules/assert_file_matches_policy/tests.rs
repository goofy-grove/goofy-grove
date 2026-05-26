use super::*;

fn png_only_policy(max_bytes: usize) -> FilePolicy {
    FilePolicy {
        max_size: FileSize::try_new(max_bytes).unwrap(),
        allowed_content_types: vec![FileContentType::try_new("image/png".to_string()).unwrap()],
    }
}

fn policy_with_empty_allowlist(max_bytes: usize) -> FilePolicy {
    FilePolicy {
        max_size: FileSize::try_new(max_bytes).unwrap(),
        allowed_content_types: vec![],
    }
}

#[test]
fn assert_file_matches_policy_accepts_at_max_size() {
    let policy = png_only_policy(100);
    let size = FileSize::try_new(100).unwrap();
    let content_type = FileContentType::try_new("image/png".to_string()).unwrap();

    assert!(assert_file_matches_policy(&size, &content_type, policy).is_ok());
}

#[test]
fn assert_file_matches_policy_rejects_over_max_size() {
    let policy = png_only_policy(100);
    let size = FileSize::try_new(101).unwrap();
    let content_type = FileContentType::try_new("image/png".to_string()).unwrap();

    assert!(matches!(
        assert_file_matches_policy(&size, &content_type, policy),
        Err(FilePolicyViolationError::InvalidFileSize { .. })
    ));
}

#[test]
fn assert_file_matches_policy_rejects_disallowed_content_type() {
    let policy = png_only_policy(100);
    let size = FileSize::try_new(10).unwrap();
    let content_type = FileContentType::try_new("image/jpeg".to_string()).unwrap();

    assert!(matches!(
        assert_file_matches_policy(&size, &content_type, policy),
        Err(FilePolicyViolationError::InvalidContentType { .. })
    ));
}

#[test]
fn assert_file_matches_policy_accepts_any_content_type_when_allowlist_empty() {
    let policy = policy_with_empty_allowlist(100);
    let size = FileSize::try_new(10).unwrap();
    let content_type = FileContentType::try_new("image/jpeg".to_string()).unwrap();

    assert!(assert_file_matches_policy(&size, &content_type, policy).is_ok());
}

#[test]
fn assert_file_matches_policy_empty_allowlist_still_enforces_max_size() {
    let policy = policy_with_empty_allowlist(100);
    let size = FileSize::try_new(101).unwrap();
    let content_type = FileContentType::try_new("application/octet-stream".to_string()).unwrap();

    assert!(matches!(
        assert_file_matches_policy(&size, &content_type, policy),
        Err(FilePolicyViolationError::InvalidFileSize { .. })
    ));
}
