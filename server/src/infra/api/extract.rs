use axum::{body::Body, extract::FromRequestParts, http::Response, http::request::Parts};

use gg_core::domain::prelude::ParticipantId;

use super::response;

/// Optional `x-socket-id` header: at most one socket participant to exclude from broadcast (e.g. the tab that initiated the request).
pub struct ExcludeSocketParticipants(pub Option<ParticipantId>);

impl<S> FromRequestParts<S> for ExcludeSocketParticipants
where
    S: Send + Sync,
{
    type Rejection = Response<Body>;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let participant = match parts.headers.get("x-socket-id") {
            Some(value) => {
                let id = value
                    .to_str()
                    .map_err(|_| response::bad_request(&["Invalid x-socket-id header"]))?;
                Some(
                    ParticipantId::try_new(id.to_owned())
                        .map_err(|_| response::bad_request(&["Invalid x-socket-id header"]))?,
                )
            }
            None => None,
        };

        Ok(Self(participant))
    }
}
