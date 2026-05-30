use gg_core::domain::prelude::*;

fn map_avatar_uid(value: Option<String>) -> Result<Option<FileId>, String> {
    match value {
        None => Ok(None),
        Some(uid) => FileId::try_new(uid)
            .map(Some)
            .map_err(|err| err.to_string()),
    }
}

pub fn persona_from_model(
    uid: String,
    creator_id: String,
    name: String,
    description: String,
    avatar_uid: Option<String>,
) -> Result<Persona, String> {
    Ok(Persona {
        uid: PersonaId::try_new(uid).map_err(|err| err.to_string())?,
        creator_id: UserId::try_new(creator_id).map_err(|err| err.to_string())?,
        name: PersonaName::try_new(name).map_err(|err| err.to_string())?,
        description: PersonaDescription::new(description),
        avatar_uid: map_avatar_uid(avatar_uid)?,
    })
}

pub fn user_from_model(
    uid: String,
    name: String,
    password: String,
    avatar_uid: Option<String>,
) -> Result<User, String> {
    Ok(User {
        uid: UserId::try_new(uid).map_err(|err| err.to_string())?,
        name: Username::try_new(name).map_err(|err| err.to_string())?,
        password: UserPassword::try_new(password).map_err(|err| err.to_string())?,
        avatar_uid: map_avatar_uid(avatar_uid)?,
    })
}
