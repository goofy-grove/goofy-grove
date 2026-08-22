use base64::{Engine, engine::general_purpose::URL_SAFE};
use chrono::NaiveDateTime;
use serde::Serialize;
use thiserror::Error;

use crate::{
    app::AppDeps,
    messages::db::{self, Message},
    platform::database::PageData,
};

#[derive(Debug, Clone, Error)]
pub enum GetPaginatedError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Invalid page data")]
    InvalidPageData,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetPaginatedResult {
    messages: Vec<Message>,
    next_page: Option<String>,
}

fn parse_page_data(page: Option<String>, limit: u64) -> Result<PageData, GetPaginatedError> {
    if let Some(page) = page {
        let decoded = URL_SAFE
            .decode(page)
            .map_err(|_| GetPaginatedError::InvalidPageData)?;

        let next_page =
            serde_json::from_slice(&decoded).map_err(|_| GetPaginatedError::InvalidPageData)?;

        Ok(PageData {
            limit,
            next_page: Some(next_page),
        })
    } else {
        Ok(PageData {
            limit,
            next_page: None,
        })
    }
}

fn stringify_page_data(
    created_at: NaiveDateTime,
    uid: String,
) -> Result<String, GetPaginatedError> {
    let serialized = serde_json::to_string(&(created_at, uid))
        .map_err(|_| GetPaginatedError::InvalidPageData)?;

    Ok(URL_SAFE.encode(serialized))
}

pub async fn get_paginated(
    deps: &AppDeps,
    chat_uid: String,
    page: Option<String>,
    limit: u64,
) -> Result<GetPaginatedResult, GetPaginatedError> {
    let page_data = parse_page_data(page, limit)?;
    let messages = db::load_messages(&deps.db, chat_uid, page_data)
        .await
        .map_err(|err| GetPaginatedError::Internal(err.to_string()))?;
    let next_page = messages
        .first()
        .map(|message| stringify_page_data(message.created_at.clone(), message.uid.clone()))
        .transpose()?;

    Ok(GetPaginatedResult {
        messages,
        next_page,
    })
}
