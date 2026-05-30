use crate::application::avatar::{AvatarBindingError, apply_avatar_uid_patch};
use crate::domain::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct GetUserByNameService<L: LoadUserByNamePort> {
    load_user_by_name_port: L,
}

#[derive(Debug, Clone)]
pub struct UpdateUserPrerequisites<L, S, E, LF, A, O> {
    pub load_user_by_id_port: L,
    pub save_user_port: S,
    pub event_publisher: E,
    pub load_file_port: LF,
    pub activate_file_port: A,
    pub orphan_file_port: O,
}

#[derive(Debug, Clone)]
pub struct UserUpdateService<
    L: LoadUserByIdPort,
    S: SaveUserPort,
    E: EventPublisher,
    LF: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
> {
    load_user_by_id_port: L,
    save_user_port: S,
    event_publisher: E,
    load_file_port: LF,
    activate_file_port: A,
    orphan_file_port: O,
}

impl<L: LoadUserByNamePort> GetUserByNameService<L> {
    pub fn new(load_user_by_name_port: L) -> Self {
        Self {
            load_user_by_name_port,
        }
    }
}

impl<
    L: LoadUserByIdPort,
    S: SaveUserPort,
    E: EventPublisher,
    LF: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
> UserUpdateService<L, S, E, LF, A, O>
{
    pub fn new(prerequisites: UpdateUserPrerequisites<L, S, E, LF, A, O>) -> Self {
        let UpdateUserPrerequisites {
            load_user_by_id_port,
            save_user_port,
            event_publisher,
            load_file_port,
            activate_file_port,
            orphan_file_port,
        } = prerequisites;

        Self {
            load_user_by_id_port,
            save_user_port,
            event_publisher,
            load_file_port,
            activate_file_port,
            orphan_file_port,
        }
    }
}

impl<L: LoadUserByNamePort> GetUserByNameQuery for GetUserByNameService<L> {
    async fn get_user_by_name(&self, id: &Username) -> Result<User, GetUserByNameError> {
        self.load_user_by_name_port
            .load_user_by_name(id)
            .await
            .map_err(|err| match err {
                LoadUserByNamePortError::NotFound => GetUserByNameError::UserNotFound,
                err => GetUserByNameError::InternalError(format!("{:?}", err)),
            })
    }
}

impl<
    L: LoadUserByIdPort,
    S: SaveUserPort,
    E: EventPublisher,
    LF: LoadFilePort,
    A: ActivateFilePort,
    O: OrphanFilePort,
> UpdateUserUseCase for UserUpdateService<L, S, E, LF, A, O>
{
    async fn update_user(
        &self,
        command: UpdateUserCommand,
        user_id: UserId,
    ) -> Result<User, UpdateUserError> {
        let UpdateUserCommand {
            avatar_uid,
            exclude_participants,
        } = command;

        let user = self
            .load_user_by_id_port
            .load_user_by_id(&user_id)
            .await
            .map_err(|err| match err {
                LoadUserByIdPortError::NotFound => UpdateUserError::NotFound,
                LoadUserByIdPortError::InternalError(message) => {
                    UpdateUserError::InternalError(message)
                }
            })?;

        let User {
            uid,
            name,
            password,
            avatar_uid: current_avatar_uid,
        } = user;

        let expected_scope = FileScope::UserAvatar {
            user_id: user_id.clone(),
        };

        let next_avatar_uid = apply_avatar_uid_patch(
            &self.load_file_port,
            &self.activate_file_port,
            &self.orphan_file_port,
            current_avatar_uid,
            avatar_uid,
            &expected_scope,
        )
        .await
        .map_err(|err| match err {
            AvatarBindingError::FileNotFound => UpdateUserError::FileNotFound,
            AvatarBindingError::ValidationError(message) => {
                UpdateUserError::ValidationError(message)
            }
            AvatarBindingError::InternalError(message) => UpdateUserError::InternalError(message),
        })?;

        let updated_user = User {
            uid,
            name,
            password,
            avatar_uid: next_avatar_uid,
        };

        let saved_user = self
            .save_user_port
            .save_user(updated_user)
            .await
            .map_err(|err| UpdateUserError::InternalError(format!("{:?}", err)))?;

        self.event_publisher
            .publish(UserUpdatedEvent {
                user: saved_user.clone(),
                exclude_participants,
            })
            .await;

        Ok(saved_user)
    }
}
