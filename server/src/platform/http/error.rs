use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::{Value, json};

pub mod codes {
    pub const AUTH_INVALID_USERNAME: &str = "auth_invalid_username";
    pub const AUTH_INVALID_PASSWORD: &str = "auth_invalid_password";
    pub const AUTH_INVALID_CREDENTIALS: &str = "auth_invalid_credentials";
    pub const AUTH_TOKEN_NOT_FOUND: &str = "auth_token_not_found";
    pub const AUTH_TOKEN_INVALID: &str = "auth_token_invalid";
    pub const AUTH_REFRESH_TOKEN_NOT_FOUND: &str = "auth_refresh_token_not_found";
    pub const AUTH_USER_NOT_FOUND: &str = "auth_user_not_found";
    pub const AUTH_AUTHENTICATION_FAILED: &str = "auth_authentication_failed";
    pub const AUTH_TOKEN_GENERATION_FAILED: &str = "auth_token_generation_failed";

    pub const USER_NOT_FOUND: &str = "user_not_found";
    pub const USER_UPDATE_FAILED: &str = "user_update_failed";

    pub const PERSONA_NOT_FOUND: &str = "persona_not_found";
    pub const PERSONA_INVALID_NAME: &str = "persona_invalid_name";
    pub const PERSONA_INVALID_UID: &str = "persona_invalid_uid";
    pub const PERSONA_NO_FIELDS_PROVIDED: &str = "persona_no_fields_provided";
    pub const PERSONA_LIST_FAILED: &str = "persona_list_failed";
    pub const PERSONA_CREATE_FAILED: &str = "persona_create_failed";
    pub const PERSONA_UPDATE_FAILED: &str = "persona_update_failed";
    pub const PERSONA_DELETE_FAILED: &str = "persona_delete_failed";

    pub const CHARACTER_NOT_FOUND: &str = "character_not_found";
    pub const CHARACTER_INVALID_NAME: &str = "character_invalid_name";
    pub const CHARACTER_INVALID_UID: &str = "character_invalid_uid";
    pub const CHARACTER_NO_FIELDS_PROVIDED: &str = "character_no_fields_provided";
    pub const CHARACTER_LIST_FAILED: &str = "character_list_failed";
    pub const CHARACTER_CREATE_FAILED: &str = "character_create_failed";
    pub const CHARACTER_UPDATE_FAILED: &str = "character_update_failed";
    pub const CHARACTER_DELETE_FAILED: &str = "character_delete_failed";

    pub const FILE_NOT_FOUND: &str = "file_not_found";
    pub const FILE_ACCESS_DENIED: &str = "file_access_denied";
    pub const FILE_INVALID_SIZE: &str = "file_invalid_size";
    pub const FILE_INVALID_CONTENT_TYPE: &str = "file_invalid_content_type";
    pub const FILE_FIELD_REQUIRED: &str = "file_field_required";
    pub const FILE_INVALID_ORIGINAL_NAME: &str = "file_invalid_original_name";
    pub const FILE_UPLOAD_POLICY_NOT_CONFIGURED: &str = "file_upload_policy_not_configured";
    pub const FILE_GET_FAILED: &str = "file_get_failed";
    pub const FILE_UPLOAD_FAILED: &str = "file_upload_failed";
    pub const FILE_RESPONSE_BUILD_FAILED: &str = "file_response_build_failed";

    pub const CHAT_LIST_FAILED: &str = "chat_list_failed";
    pub const CHAT_INVALID_UID: &str = "chat_invalid_uid";
    pub const CHAT_INVALID_NAME: &str = "chat_invalid_name";
    pub const CHAT_CREATE_FAILED: &str = "chat_create_failed";
    pub const CHAT_UPDATE_FAILED: &str = "chat_update_failed";
    pub const CHAT_NOT_FOUND: &str = "chat_not_found";
    pub const CHAT_DELETE_FAILED: &str = "chat_delete_failed";
    pub const CHAT_INVALID_USER_UID: &str = "chat_invalid_user_uid";
    pub const CHAT_USER_ALREADY_IN_CHAT: &str = "chat_user_already_in_chat";
    pub const CHAT_OR_USER_NOT_FOUND: &str = "chat_or_user_not_found";
    pub const CHAT_MEMBER_ADD_FAILED: &str = "chat_member_add_failed";
    pub const CHAT_MEMBER_REMOVE_FAILED: &str = "chat_member_remove_failed";
    pub const CHAT_CHARACTER_ADD_FAILED: &str = "chat_character_add_failed";
    pub const CHAT_CHARACTER_REMOVE_FAILED: &str = "chat_character_remove_failed";
    pub const CHAT_INVALID_CHARACTER_UID: &str = "chat_invalid_character_uid";
    pub const CHAT_OR_CHARACTER_NOT_FOUND: &str = "chat_or_character_not_found";
    pub const CHAT_CHARACTER_ALREADY_IN_CHAT: &str = "chat_character_already_in_chat";
    pub const CHAT_NO_FIELDS_PROVIDED: &str = "chat_no_fields_provided";

    pub const MESSAGES_GET_FAILED: &str = "messages_get_failed";
    pub const MESSAGES_CHAT_OR_AUTHOR_OR_MESSAGE_NOT_FOUND: &str =
        "messages_chat_or_author_or_message_not_found";
    pub const MESSAGES_SEND_FAILED: &str = "messages_send_failed";

    pub const COMMON_INVALID_SOCKET_ID_HEADER: &str = "common_invalid_socket_id_header";
    pub const COMMON_INVALID_REQUEST_BODY: &str = "common_invalid_request_body";
    pub const COMMON_INVALID_PAGE_DATA: &str = "common_invalid_page_data";
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorBody {
    pub code: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ApiError {
    status: StatusCode,
    pub code: &'static str,
    pub params: Option<Value>,
}

impl ApiError {
    fn new(status: StatusCode, code: &'static str) -> Self {
        Self {
            status,
            code,
            params: None,
        }
    }

    pub fn unauthorized(code: &'static str) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, code)
    }

    pub fn forbidden(code: &'static str) -> Self {
        Self::new(StatusCode::FORBIDDEN, code)
    }

    pub fn bad_request(code: &'static str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code)
    }

    pub fn not_found(code: &'static str) -> Self {
        Self::new(StatusCode::NOT_FOUND, code)
    }

    pub fn internal(code: &'static str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, code)
    }

    pub fn conflict(code: &'static str) -> Self {
        Self::new(StatusCode::CONFLICT, code)
    }

    pub fn with_params(mut self, params: Value) -> Self {
        self.params = Some(params);
        self
    }

    fn body(&self) -> ApiErrorBody {
        ApiErrorBody {
            code: self.code,
            params: self.params.clone(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        let mut data = json!({ "code": self.body().code });

        if let Some(params) = self.body().params {
            data["params"] = params;
        }

        (self.status, Json(json!({ "error": true, "data": data }))).into_response()
    }
}
