use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::AuthUser;
use crate::storage;

const TOKEN_EXPIRY_DAYS: i64 = 30;

use super::{SharedState, validate_phone};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub phone: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub phone: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: AuthUser,
}

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    let phone = validate_phone(&payload.phone)?;
    let name = payload.name.trim();

    if name.is_empty() {
        return Err(AppError::BadRequest("Name is required".to_string()));
    }

    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    if app_data.users.iter().any(|u| u.phone == phone) {
        warn!(phone = %phone, "registration failed: phone already registered");
        return Err(AppError::BadRequest(
            "Phone number already registered".to_string(),
        ));
    }

    let max_id = app_data.users.iter().map(|u| u.id).max().unwrap_or(0);
    let expires_at = Utc::now() + Duration::days(TOKEN_EXPIRY_DAYS);
    let user = AuthUser {
        id: max_id + 1,
        phone: phone.to_string(),
        name: name.to_string(),
        token: Uuid::new_v4().to_string(),
        current_group_id: 0,
        token_expires_at: Some(expires_at.to_rfc3339()),
    };
    app_data.users.push(user.clone());
    storage::save(&app_data)?;

    info!(user_id = user.id, name = %user.name, "user registered");
    Ok(Json(AuthResponse { user }))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let phone = validate_phone(&payload.phone)?;

    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let user = app_data
        .users
        .iter_mut()
        .find(|u| u.phone == phone)
        .ok_or_else(AppError::phone_not_registered)?;

    // Rotate token on login
    user.token = Uuid::new_v4().to_string();
    user.token_expires_at = Some((Utc::now() + Duration::days(TOKEN_EXPIRY_DAYS)).to_rfc3339());
    let user = user.clone();
    storage::save(&app_data)?;

    info!(user_id = user.id, name = %user.name, "user logged in");
    Ok(Json(AuthResponse { user }))
}

pub async fn get_me(user: AuthUser) -> AppResult<Json<AuthUser>> {
    Ok(Json(user))
}
