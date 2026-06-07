use axum::{
    extract::rejection::JsonRejection,
    extract::{FromRequest, FromRequestParts, Multipart, Request},
};

use super::error::{ApiError, codes};

/// Optional `x-socket-id` header: at most one socket participant to exclude from broadcast.
pub struct ExcludeSocketParticipants(pub Option<String>);

impl<S> FromRequestParts<S> for ExcludeSocketParticipants
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let participant = match parts.headers.get("x-socket-id") {
            Some(value) => {
                let id = value
                    .to_str()
                    .map_err(|_| ApiError::bad_request(codes::COMMON_INVALID_SOCKET_ID_HEADER))?
                    .trim()
                    .to_string();

                if id.is_empty() {
                    return Err(ApiError::bad_request(
                        codes::COMMON_INVALID_SOCKET_ID_HEADER,
                    ));
                }

                Some(id)
            }
            None => None,
        };

        Ok(Self(participant))
    }
}

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(_) => Err(ApiError::bad_request(codes::COMMON_INVALID_REQUEST_BODY)),
        }
    }
}

pub async fn read_multipart_file(
    mut multipart: Multipart,
) -> Result<(String, String, Vec<u8>), ApiError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::bad_request(codes::COMMON_INVALID_REQUEST_BODY))?
    {
        if field.name() != Some("file") {
            continue;
        }

        let original_name = field.file_name().unwrap_or("upload.bin").to_string();
        let content_type = field
            .content_type()
            .map(str::to_string)
            .unwrap_or_else(|| "application/octet-stream".to_string());
        let bytes = field
            .bytes()
            .await
            .map_err(|_| ApiError::bad_request(codes::COMMON_INVALID_REQUEST_BODY))?
            .to_vec();

        if original_name.trim().is_empty() {
            return Err(ApiError::bad_request(codes::FILE_INVALID_ORIGINAL_NAME));
        }

        if content_type.trim().is_empty() {
            return Err(ApiError::bad_request(codes::FILE_INVALID_CONTENT_TYPE));
        }

        return Ok((original_name, content_type, bytes));
    }

    Err(ApiError::bad_request(codes::FILE_FIELD_REQUIRED))
}
