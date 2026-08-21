use axum::{
    Extension, Router,
    extract::{Multipart, Path, State},
    response::Response,
    routing::{delete, get, patch, post, put},
};
use serde::Deserialize;
use tracing::error;

use crate::{
    app::AppDeps,
    auth::{AuthLayerExt, AuthenticatedUser},
    chat::services::{
        self,
        avatar::{
            ClearChatAvatarError, ClearChatAvatarInput, SetChatAvatarError, SetChatAvatarInput,
        },
        character_add::{AddCharacterError, AddCharacterInput},
        character_remove::{RemoveCharacterError, RemoveCharacterInput},
        create::CreateChatInput,
        delete::{DeleteChatError, DeleteChatInput},
        member_add::{AddMemberError, AddMemberInput},
        member_remove::{RemoveMemberFromChatError, RemoveUserFromChatInput},
        update::{UpdateChatError, UpdateChatInput},
    },
    platform::http::{
        error::{ApiError, codes},
        extract::{ExcludeSocketParticipants, ValidatedJson, read_multipart_file},
        response::{self, Empty},
    },
};

#[derive(Clone, Debug, Deserialize)]
struct ChatMemberAddRequest {
    user_uid: String,
}

#[derive(Clone, Debug, Deserialize)]
struct ChatCharacterAddRequest {
    character_uid: String,
}

#[derive(Clone, Debug, Deserialize)]
struct CreateChatRequest {
    name: String,
}

#[derive(Clone, Debug, Deserialize)]
struct UpdateChatRequest {
    name: Option<String>,
}

async fn get_all_user_chats(
    Extension(user): Extension<AuthenticatedUser>,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    services::get::get_chats(&deps, &user.uid)
        .await
        .map_err(|err| {
            error!(target: "chats::api::get_all_user_chats", ?err, "Failed to get chats");

            ApiError::internal(codes::CHAT_LIST_FAILED)
        })
        .map(response::ok)
}

async fn create_chat(
    Extension(user): Extension<AuthenticatedUser>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<CreateChatRequest>,
) -> Result<Response, ApiError> {
    if request.name.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_NAME));
    }

    let input = CreateChatInput {
        name: request.name.trim().to_string(),
        creator_uid: user.uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::create::create_chat(&deps, input)
        .await
        .map_err(|_| ApiError::internal(codes::CHAT_CREATE_FAILED))
        .map(response::ok)
}

async fn update_chat(
    Extension(user): Extension<AuthenticatedUser>,
    Path(chat_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<UpdateChatRequest>,
) -> Result<Response, ApiError> {
    let Some(name) = request.name else {
        return Err(ApiError::bad_request(codes::CHAT_NO_FIELDS_PROVIDED));
    };

    let name = name.trim();
    let chat_uid = chat_uid.trim();

    if name.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_NAME));
    }

    if chat_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    let name = name.to_string();
    let chat_uid = chat_uid.to_string();
    let input = UpdateChatInput {
        chat_uid,
        name,
        initiator_uid: user.uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::update::update_chat(&deps, input)
        .await
        .map_err(|err| match err {
            UpdateChatError::NotFound => ApiError::not_found(codes::CHAT_NOT_FOUND),
            UpdateChatError::Internal(_) => {
                error!(target: "chat::api::update_chat", ?err, "Failed to update chat");

                ApiError::internal(codes::CHAT_UPDATE_FAILED)
            }
        })
        .map(response::ok)
}

async fn put_chat_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    Path(chat_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    multipart: Multipart,
) -> Result<Response, ApiError> {
    let chat_uid = chat_uid.trim();

    if chat_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    let chat_uid = chat_uid.to_string();

    let max_file_bytes = deps
        .config
        .policies
        .files
        .chat_avatar
        .max_file_size
        .to_bytes() as usize;

    let (original_name, content_type, content) =
        read_multipart_file(multipart, max_file_bytes).await?;

    let input = SetChatAvatarInput {
        chat_uid,
        initiator_uid: user.uid,
        content_type,
        original_name,
        content,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::avatar::set_chat_avatar(&deps, input)
        .await
        .map_err(|err| match err {
            SetChatAvatarError::NotFound => ApiError::not_found(codes::CHAT_NOT_FOUND),
            SetChatAvatarError::ReplaceAvatar(replace_err) => {
                error!(target: "chat::api::put_chat_avatar", ?replace_err, "Failed to replace chat avatar");

                ApiError::from(replace_err)
            }
            SetChatAvatarError::Internal(_) => {
                error!(target: "chat::api::put_chat_avatar", ?err, "Failed to set chat avatar");

                ApiError::internal(codes::CHAT_UPDATE_FAILED)
            }
        }).map(response::ok)
}

async fn delete_chat_avatar(
    Extension(user): Extension<AuthenticatedUser>,
    Path(chat_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    let chat_uid = chat_uid.trim();

    if chat_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    let chat_uid = chat_uid.to_string();

    let input = ClearChatAvatarInput {
        chat_uid,
        initiator_uid: user.uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::avatar::clear_chat_avatar(&deps, input)
        .await
        .map_err(|err| match err {
            ClearChatAvatarError::NotFound => ApiError::not_found(codes::CHAT_NOT_FOUND),
            ClearChatAvatarError::Internal(_) => {
                error!(target: "chat::api::delete_chat_avatar", ?err, "Failed to clear chat avatar");

                ApiError::internal(codes::CHAT_UPDATE_FAILED)
            }
        })
        .map(response::ok)
}

async fn delete_chat(
    Extension(user): Extension<AuthenticatedUser>,
    Path(chat_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    if chat_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    let input = DeleteChatInput {
        chat_uid: chat_uid.trim().to_string(),
        user_uid: user.uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::delete::delete_chat(&deps, input)
        .await
        .map_err(|err| match err {
            DeleteChatError::NotFound => ApiError::not_found(codes::CHAT_NOT_FOUND),
            DeleteChatError::Internal(_) => {
                error!(target: "chat::api::delete_chat", ?err, "Failed to delete chat");

                ApiError::internal(codes::CHAT_DELETE_FAILED)
            }
        })
        .map(|_| response::ok(Empty {}))
}

async fn member_add(
    Extension(user): Extension<AuthenticatedUser>,
    Path(chat_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<ChatMemberAddRequest>,
) -> Result<Response, ApiError> {
    if chat_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    if request.user_uid.trim().is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_USER_UID));
    }

    let input = AddMemberInput {
        chat_uid: chat_uid.trim().to_string(),
        user_uid: request.user_uid.trim().to_string(),
        initiator_uid: user.uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::member_add::add_member(&deps, input)
        .await
        .map_err(|err| match err {
            AddMemberError::AlreadyInChat => ApiError::conflict(codes::CHAT_USER_ALREADY_IN_CHAT),
            AddMemberError::NotFound => ApiError::not_found(codes::CHAT_OR_USER_NOT_FOUND),
            AddMemberError::Internal(_) => {
                error!(target: "chat::api::member_add", ?err, "Failed to add member to chat");

                ApiError::internal(codes::CHAT_MEMBER_ADD_FAILED)
            }
        })
        .map(response::ok)
}

async fn member_remove(
    Extension(user): Extension<AuthenticatedUser>,
    Path((chat_uid, user_uid)): Path<(String, String)>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    let chat_uid = chat_uid.trim();
    let user_uid = user_uid.trim();

    if chat_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    if user_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_USER_UID));
    }

    let chat_uid = chat_uid.to_string();
    let user_uid = user_uid.to_string();
    let input = RemoveUserFromChatInput {
        chat_uid,
        user_uid,
        initiator_uid: user.uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::member_remove::remove_member_from_chat(&deps, input)
        .await
        .map_err(|err| match err {
            RemoveMemberFromChatError::NotFound => {
                ApiError::not_found(codes::CHAT_OR_USER_NOT_FOUND)
            }
            RemoveMemberFromChatError::Internal(_) => {
                error!(target: "chat::api::member_remove", ?err, "Failed to remove user from chat");

                ApiError::internal(codes::CHAT_MEMBER_REMOVE_FAILED)
            }
        })
        .map(response::ok)
}

async fn character_add(
    Extension(user): Extension<AuthenticatedUser>,
    Path(chat_uid): Path<String>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
    ValidatedJson(request): ValidatedJson<ChatCharacterAddRequest>,
) -> Result<Response, ApiError> {
    let chat_uid = chat_uid.trim();
    let character_uid = request.character_uid.trim();

    if chat_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    if character_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_CHARACTER_UID));
    }

    let chat_uid = chat_uid.to_string();
    let character_uid = character_uid.to_string();
    let input = AddCharacterInput {
        chat_uid,
        character_uid,
        initiator_uid: user.uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::character_add::add_character(&deps, input)
        .await
        .map_err(|err| match err {
            AddCharacterError::NotFound => ApiError::not_found(codes::CHAT_OR_CHARACTER_NOT_FOUND),
            AddCharacterError::Internal(_) => {
                error!(target: "chat::api::character_add", ?err, "Failed to add character to chat");

                ApiError::internal(codes::CHAT_CHARACTER_ADD_FAILED)
            }
            AddCharacterError::AlreadyInChat => {
                ApiError::conflict(codes::CHAT_CHARACTER_ALREADY_IN_CHAT)
            }
        })
        .map(response::ok)
}

async fn character_remove(
    Extension(user): Extension<AuthenticatedUser>,
    Path((chat_uid, character_uid)): Path<(String, String)>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    let chat_uid = chat_uid.trim();
    let character_uid = character_uid.trim();

    if chat_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    if character_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_CHARACTER_UID));
    }

    let chat_uid = chat_uid.to_string();
    let character_uid = character_uid.to_string();
    let input = RemoveCharacterInput {
        chat_uid,
        character_uid,
        initiator_uid: user.uid,
        exclude_participants: exclude_participant.into_iter().collect(),
    };

    services::character_remove::remove_character(&deps, input).await.map_err(|err| match err {
        RemoveCharacterError::NotFound => ApiError::not_found(codes::CHAT_OR_CHARACTER_NOT_FOUND),
        RemoveCharacterError::Internal(_) => {
            error!(target: "chat::api::character_remove", ?err, "Failed to remove character from chat");

            ApiError::internal(codes::CHAT_CHARACTER_REMOVE_FAILED)
        }
    }).map(response::ok)
}

fn routes() -> Router<AppDeps> {
    Router::new()
        .route("/", get(get_all_user_chats))
        .route("/", post(create_chat))
        .route("/{chat_uid}", delete(delete_chat))
        .route("/{chat_uid}", patch(update_chat))
        .route("/{chat_uid}/avatar", put(put_chat_avatar))
        .route("/{chat_uid}/avatar", delete(delete_chat_avatar))
        .route("/{chat_uid}/members", post(member_add))
        .route("/{chat_uid}/members/{user_uid}", delete(member_remove))
        .route("/{chat_uid}/characters", post(character_add))
        .route(
            "/{chat_uid}/characters/{character_uid}",
            delete(character_remove),
        )
}

pub fn mount(router: Router, deps: &AppDeps) -> Router {
    router.nest(
        "/api/v1/chats",
        routes().with_state(deps.clone()).with_auth(deps.into()),
    )
}
