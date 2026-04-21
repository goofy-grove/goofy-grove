use super::*;

#[test]
fn participant_id_trims_value() {
    assert_eq!(
        ParticipantId::try_new("  user-2 ".to_string())
            .unwrap()
            .inner(),
        "user-2"
    );
}

#[test]
fn participant_id_rejects_empty() {
    assert_eq!(
        ParticipantId::try_new(" ".to_string()).unwrap_err(),
        ParticipantIdValidationError::Empty
    );
}
