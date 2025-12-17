use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use tracing::info;

use crate::errors::{AppError, AppResult};
use crate::models::{AuthUser, User};
use crate::storage;

use super::{SharedState, validate_phone};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub phone: String,
}

pub async fn create_user(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<Json<User>> {
    let phone = validate_phone(&payload.phone)?;

    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let registered_user = app_data
        .users
        .iter()
        .find(|u| u.phone == phone)
        .ok_or_else(AppError::phone_not_registered)?;

    let name = registered_user.name.clone();

    let group = app_data
        .groups
        .iter_mut()
        .find(|g| g.id == auth_user.current_group_id)
        .ok_or_else(AppError::group_not_found)?;

    if group.members.iter().any(|u| u.name == name) {
        return Err(AppError::BadRequest(format!(
            "User '{}' is already in this group",
            name
        )));
    }

    let max_id = group.members.iter().map(|u| u.id).max().unwrap_or(0);
    let user = User {
        id: max_id + 1,
        name,
    };

    group.members.push(user.clone());
    storage::save(&app_data)?;

    info!(
        user_id = user.id,
        name = %user.name,
        group_id = auth_user.current_group_id,
        "member added to group"
    );
    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<usize>,
) -> AppResult<Json<serde_json::Value>> {
    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let group = app_data
        .groups
        .iter_mut()
        .find(|g| g.id == auth_user.current_group_id)
        .ok_or_else(AppError::group_not_found)?;

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

    let removed_name = group.members[index].name.clone();
    group.members.remove(index);
    storage::save(&app_data)?;

    info!(
        user_id = id,
        name = %removed_name,
        group_id = auth_user.current_group_id,
        "member removed from group"
    );
    Ok(Json(serde_json::json!({ "success": true })))
}
