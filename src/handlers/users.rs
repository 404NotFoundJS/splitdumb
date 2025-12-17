use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;

use crate::errors::{AppError, AppResult};
use crate::models::User;
use crate::storage;

use super::SharedState;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

pub async fn create_user(
    State(state): State<SharedState>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<Json<User>> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(AppError::BadRequest(
            "User name cannot be empty".to_string(),
        ));
    }
    if name.len() > 100 {
        return Err(AppError::BadRequest(
            "User name too long (max 100 chars)".to_string(),
        ));
    }

    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let current_id = app_data.current_group_id;
    let group = app_data
        .groups
        .iter_mut()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    if group.members.iter().any(|u| u.name == name) {
        return Err(AppError::BadRequest(format!(
            "User '{}' already exists",
            name
        )));
    }

    let max_id = group.members.iter().map(|u| u.id).max().unwrap_or(0);
    let user = User {
        id: max_id + 1,
        name: name.to_string(),
    };

    group.members.push(user.clone());

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<SharedState>,
    Path(id): Path<usize>,
) -> AppResult<Json<serde_json::Value>> {
    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let current_id = app_data.current_group_id;
    let group = app_data
        .groups
        .iter_mut()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let index = group
        .members
        .iter()
        .position(|u| u.id == id)
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))?;

    let user_name = &group.members[index].name;
    let has_expenses = group
        .expenses
        .iter()
        .any(|e| e.payer.name == *user_name || e.participants.iter().any(|p| p.name == *user_name));

    if has_expenses {
        return Err(AppError::BadRequest(
            "Cannot delete user with existing expenses".to_string(),
        ));
    }

    group.members.remove(index);

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(serde_json::json!({ "success": true })))
}
