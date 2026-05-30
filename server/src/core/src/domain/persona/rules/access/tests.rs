use super::*;

fn user(val: &str) -> UserId {
    UserId::try_new(val.to_string()).unwrap()
}

fn persona(creator: UserId) -> Persona {
    Persona {
        uid: PersonaId::try_new("p1".to_string()).unwrap(),
        creator_id: creator,
        name: PersonaName::try_new("Guide".to_string()).unwrap(),
        description: PersonaDescription::new("desc".to_string()),
        avatar_uid: None,
    }
}

#[test]
fn can_update_persona_allows_creator() {
    let actor = user("u1");
    let p = persona(actor.clone());

    assert!(can_update_persona(&actor, &p).is_ok());
}

#[test]
fn can_update_persona_denies_non_creator() {
    let p = persona(user("u1"));

    assert_eq!(
        can_update_persona(&user("u2"), &p).unwrap_err(),
        PersonaAccessError::AccessDenied
    );
}

#[test]
fn can_delete_persona_denies_non_creator() {
    let p = persona(user("u1"));

    assert_eq!(
        can_delete_persona(&user("intruder"), &p).unwrap_err(),
        PersonaAccessError::AccessDenied
    );
}
