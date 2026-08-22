use axum::{
    Extension, Router,
    extract::{Path, Query, State},
    response::Response,
    routing::{get, post},
};
use serde::Deserialize;
use tracing::error;

use crate::{
    app::AppDeps,
    auth::{AuthLayerExt, AuthenticatedUser},
    messages::{
        db::MessageAuthorUid,
        services::{
            self,
            get_paginated::GetPaginatedError,
            send::{SendInput, SendMessageError},
        },
    },
    platform::http::{
        error::{ApiError, codes},
        extract::{ExcludeSocketParticipants, ValidatedJson},
        response,
    },
};

#[derive(Debug, Clone, Deserialize)]
pub struct GetMessagesQuery {
    limit: u64,
    page: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageRequest {
    content: String,
    author: MessageAuthorUid,
    reply_to_message_uid: Option<String>,
}

async fn get_paginated_messages(
    Extension(_): Extension<AuthenticatedUser>,
    Path(chat_uid): Path<String>,
    Query(query): Query<GetMessagesQuery>,
    State(deps): State<AppDeps>,
) -> Result<Response, ApiError> {
    let chat_uid = chat_uid.trim();

    if chat_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    let chat_uid = chat_uid.to_string();

    services::get_paginated::get_paginated(&deps, chat_uid, query.page, query.limit)
        .await
        .map_err(|err| match err {
            GetPaginatedError::InvalidPageData => ApiError::bad_request(codes::COMMON_INVALID_PAGE_DATA),
            GetPaginatedError::Internal(_) => {
                error!(target: "messages::api::get_paginated_messages", ?err, "Failed to get messages");

                ApiError::internal(codes::CHAT_CHARACTER_REMOVE_FAILED)
            }
        }).map(response::ok)
}

async fn send_message(
    Extension(_): Extension<AuthenticatedUser>,
    Path(chat_uid): Path<String>,
    State(deps): State<AppDeps>,
    ExcludeSocketParticipants(exclude_participant): ExcludeSocketParticipants,
    ValidatedJson(request): ValidatedJson<SendMessageRequest>,
) -> Result<Response, ApiError> {
    let chat_uid = chat_uid.trim();

    if chat_uid.is_empty() {
        return Err(ApiError::bad_request(codes::CHAT_INVALID_UID));
    }

    let chat_uid = chat_uid.to_string();

    services::send::send(
        &deps,
        SendInput {
            chat_uid,
            content: request.content,
            author_uid: request.author,
            reply_to_message_uid: request.reply_to_message_uid,
            exclude_participants: exclude_participant.into_iter().collect(),
        },
    )
    .await
    .map_err(|err| match err {
        SendMessageError::NotFound => {
            ApiError::not_found(codes::MESSAGES_CHAT_OR_AUTHOR_OR_MESSAGE_NOT_FOUND)
        }
        SendMessageError::Internal(_) => {
            error!(target: "messages::api::send_message", ?err, "Failed to send messages");

            ApiError::internal(codes::MESSAGES_SEND_FAILED)
        }
    })
    .map(response::ok)
}

fn routes() -> Router<AppDeps> {
    Router::new()
        .route("/", get(get_paginated_messages))
        .route("/", post(send_message))
}

pub fn mount(router: Router, deps: &AppDeps) -> Router {
    router.nest(
        "/api/v1/chats/{chat_uid}/messages",
        routes().with_state(deps.clone()).with_auth(deps.into()),
    )
}
