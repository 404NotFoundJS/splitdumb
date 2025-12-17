use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::models::AuthUser;
use crate::storage;

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
        return Err(AppError::BadRequest(
            "Phone number already registered".to_string(),
        ));
    }

    let max_id = app_data.users.iter().map(|u| u.id).max().unwrap_or(0);
    let user = AuthUser {
        id: max_id + 1,
        phone: phone.to_string(),
        name: name.to_string(),
        token: Uuid::new_v4().to_string(),
        current_group_id: 0,
    };
    app_data.users.push(user.clone());

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(AuthResponse { user }))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let phone = validate_phone(&payload.phone)?;

    let app_data = state.read().map_err(|_| AppError::LockError)?;

    let user = app_data
        .users
        .iter()
        .find(|u| u.phone == phone)
        .cloned()
        .ok_or_else(|| AppError::NotFound("Phone number not registered".to_string()))?;

    Ok(Json(AuthResponse { user }))
}

pub async fn get_me(State(state): State<SharedState>, user: AuthUser) -> AppResult<Json<AuthUser>> {
    let _ = state;
    Ok(Json(user))
}
