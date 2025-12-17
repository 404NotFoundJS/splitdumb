use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};

use crate::errors::{AppError, AppResult};
use crate::logic::{
    Settlement, calculate_balances, calculate_settlements, calculate_simplified_settlements,
};
use crate::models::Group;
use crate::storage;

use super::SharedState;

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct SwitchGroupRequest {
    pub group_id: usize,
}

#[derive(Deserialize)]
pub struct UpdateGroupRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub balances: std::collections::HashMap<String, f64>,
}

#[derive(Serialize)]
pub struct SettlementsResponse {
    pub settlements: Vec<Settlement>,
}

pub async fn get_current_group(State(state): State<SharedState>) -> AppResult<Json<Group>> {
    let app_data = state.read().map_err(|_| AppError::LockError)?;

    if app_data.groups.is_empty() {
        return Err(AppError::NotFound("No groups exist".to_string()));
    }

    let current_id = app_data.current_group_id;
    let group = app_data
        .groups
        .iter()
        .find(|g| g.id == current_id)
        .or_else(|| app_data.groups.first())
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    Ok(Json(group.clone()))
}

pub async fn list_groups(State(state): State<SharedState>) -> AppResult<Json<Vec<Group>>> {
    let app_data = state.read().map_err(|_| AppError::LockError)?;
    Ok(Json(app_data.groups.clone()))
}

pub async fn create_group(
    State(state): State<SharedState>,
    Json(payload): Json<CreateGroupRequest>,
) -> AppResult<Json<Group>> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(AppError::BadRequest(
            "Group name cannot be empty".to_string(),
        ));
    }

    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let max_id = app_data.groups.iter().map(|g| g.id).max().unwrap_or(0);
    let group = Group {
        id: max_id + 1,
        name: name.to_string(),
        members: vec![],
        expenses: vec![],
        simplify_debts: false,
        settled_settlements: vec![],
    };

    let is_first_group = app_data.groups.is_empty();
    app_data.groups.push(group.clone());

    if is_first_group {
        app_data.current_group_id = group.id;
    }

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(group))
}

pub async fn switch_group(
    State(state): State<SharedState>,
    Json(payload): Json<SwitchGroupRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    if !app_data.groups.iter().any(|g| g.id == payload.group_id) {
        return Err(AppError::NotFound(format!(
            "Group with id {} not found",
            payload.group_id
        )));
    }

    app_data.current_group_id = payload.group_id;

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(
        serde_json::json!({ "success": true, "current_group_id": payload.group_id }),
    ))
}

pub async fn update_group(
    State(state): State<SharedState>,
    Path(id): Path<usize>,
    Json(payload): Json<UpdateGroupRequest>,
) -> AppResult<Json<Group>> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(AppError::BadRequest(
            "Group name cannot be empty".to_string(),
        ));
    }

    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let group = app_data
        .groups
        .iter_mut()
        .find(|g| g.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Group with id {} not found", id)))?;

    group.name = name.to_string();
    let updated_group = group.clone();

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(updated_group))
}

pub async fn delete_group(
    State(state): State<SharedState>,
    Path(id): Path<usize>,
) -> AppResult<Json<serde_json::Value>> {
    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let index = app_data
        .groups
        .iter()
        .position(|g| g.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Group with id {} not found", id)))?;

    app_data.groups.remove(index);

    let switched_group = if app_data.current_group_id == id {
        app_data.current_group_id = app_data.groups.first().map(|g| g.id).unwrap_or(0);
        Some(app_data.current_group_id)
    } else {
        None
    };

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(serde_json::json!({
        "success": true,
        "switched_group": switched_group
    })))
}

pub async fn get_balances(State(state): State<SharedState>) -> AppResult<Json<BalanceResponse>> {
    let app_data = state.read().map_err(|_| AppError::LockError)?;

    let current_id = app_data.current_group_id;
    let group = app_data
        .groups
        .iter()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let balances = calculate_balances(group);
    Ok(Json(BalanceResponse { balances }))
}

pub async fn get_settlements(
    State(state): State<SharedState>,
) -> AppResult<Json<SettlementsResponse>> {
    let app_data = state.read().map_err(|_| AppError::LockError)?;

    let current_id = app_data.current_group_id;
    let group = app_data
        .groups
        .iter()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let mut settlements = if group.simplify_debts {
        calculate_simplified_settlements(group)
    } else {
        calculate_settlements(group)
    };

    for settlement in &mut settlements {
        settlement.settled = group
            .settled_settlements
            .iter()
            .any(|s| s.from == settlement.from && s.to == settlement.to);
    }

    Ok(Json(SettlementsResponse { settlements }))
}

pub async fn toggle_simplify(
    State(state): State<SharedState>,
) -> AppResult<Json<serde_json::Value>> {
    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let current_id = app_data.current_group_id;
    let group = app_data
        .groups
        .iter_mut()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    group.simplify_debts = !group.simplify_debts;
    let new_value = group.simplify_debts;

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(serde_json::json!({ "simplify_debts": new_value })))
}
