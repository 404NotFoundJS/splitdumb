pub mod auth;
pub mod expenses;
pub mod groups;
pub mod users;

use crate::errors::AppError;
use crate::models::{AppData, AuthUser};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use std::sync::{Arc, RwLock};
use tracing::warn;

pub type SharedState = Arc<RwLock<AppData>>;

pub fn validate_phone(phone: &str) -> Result<String, AppError> {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 10 {
        return Err(AppError::BadRequest(
            "Phone number must have at least 10 digits".to_string(),
        ));
    }
    Ok(digits)
}

impl FromRequestParts<SharedState> for AuthUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization token"))?;

        let app_data = state
            .read()
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read state"))?;

        let user = app_data
            .users
            .iter()
            .find(|u| u.token == token)
            .cloned()
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid token"))?;

        // Check token expiry
        if let Some(expires_at) = &user.token_expires_at
            && let Ok(expires) = chrono::DateTime::parse_from_rfc3339(expires_at)
            && chrono::Utc::now() > expires
        {
            warn!(user_id = user.id, "token expired");
            return Err((StatusCode::UNAUTHORIZED, "Token expired"));
        }

        Ok(user)
    }
}
