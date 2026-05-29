use super::*;

fn user(val: &str) -> UserId {
    UserId::try_new(val.to_string()).unwrap()
}

fn persona(val: &str) -> PersonaId {
    PersonaId::try_new(val.to_string()).unwrap()
}

fn meta(scope: FileScope) -> FileMeta {
    FileMeta {
        id: FileId::try_new("f1".to_string()).unwrap(),
        filename: Filename::try_new("a.bin".to_string()).unwrap(),
        uploaded_by: user("u1"),
        scope,
        original_name: FileOriginalName::try_new("a.png".to_string()).unwrap(),
        content_type: FileContentType::try_new("image/png".to_string()).unwrap(),
        size: FileSize::try_new(1).unwrap(),
        status: FileStatus::Activated,
        uploaded_at: UploadedAt::try_new(1).unwrap(),
    }
}

#[test]
fn can_create_user_avatar_when_actor_matches_scope_owner() {
    let actor = user("u1");
    let scope = FileScope::UserAvatar {
        user_id: actor.clone(),
    };
    let ctx = FileCreateAccessContext {
        persona_owned_by_actor: true,
    };

    assert!(can_create_file(&actor, &scope, &ctx).is_ok());
}

#[test]
fn can_create_user_avatar_denied_when_actor_mismatch() {
    let scope = FileScope::UserAvatar {
        user_id: user("u1"),
    };
    let ctx = FileCreateAccessContext {
        persona_owned_by_actor: true,
    };

    assert_eq!(
        can_create_file(&user("u2"), &scope, &ctx).unwrap_err(),
        FileAccessError::AccessDenied
    );
}

#[test]
fn can_create_persona_avatar_requires_persona_ownership() {
    let actor = user("u1");
    let scope = FileScope::PersonaAvatar {
        user_id: actor.clone(),
        persona_id: persona("p1"),
    };

    assert!(
        can_create_file(
            &actor,
            &scope,
            &FileCreateAccessContext {
                persona_owned_by_actor: true,
            }
        )
        .is_ok()
    );

    assert_eq!(
        can_create_file(
            &actor,
            &scope,
            &FileCreateAccessContext {
                persona_owned_by_actor: false,
            }
        )
        .unwrap_err(),
        FileAccessError::AccessDenied
    );
}

#[test]
fn can_read_file_denied_for_wrong_actor() {
    let scope = FileScope::UserAvatar {
        user_id: user("u1"),
    };
    let file = meta(scope);

    assert_eq!(
        can_read_file(
            &user("intruder"),
            &file,
            &FileMetaAccessContext {
                persona_owned_by_actor: true,
            }
        )
        .unwrap_err(),
        FileAccessError::AccessDenied
    );
}

#[test]
fn can_delete_file_denied_when_persona_not_owned() {
    let scope = FileScope::PersonaAvatar {
        user_id: user("u1"),
        persona_id: persona("p1"),
    };
    let file = meta(scope);

    assert_eq!(
        can_delete_file(
            &user("u1"),
            &file,
            &FileMetaAccessContext {
                persona_owned_by_actor: false,
            }
        )
        .unwrap_err(),
        FileAccessError::AccessDenied
    );
}
