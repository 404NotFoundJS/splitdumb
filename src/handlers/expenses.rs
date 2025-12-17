use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;

use crate::errors::{AppError, AppResult};
use crate::logic::add_expense;
use crate::models::{AuthUser, Expense, SettledSettlement};
use crate::storage;

use super::SharedState;

#[derive(Deserialize)]
pub struct CreateExpenseRequest {
    pub description: String,
    pub amount: f64,
    pub payer: String,
    pub participants: Vec<String>,
    pub category: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct SettleRequest {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

pub async fn create_expense(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateExpenseRequest>,
) -> AppResult<Json<Expense>> {
    let description = payload.description.trim();
    if description.is_empty() {
        return Err(AppError::BadRequest(
            "Description cannot be empty".to_string(),
        ));
    }
    if payload.amount <= 0.0 {
        return Err(AppError::BadRequest("Amount must be positive".to_string()));
    }
    if payload.participants.is_empty() {
        return Err(AppError::BadRequest(
            "Must have at least one participant".to_string(),
        ));
    }

    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let group = app_data
        .groups
        .iter_mut()
        .find(|g| g.id == auth_user.current_group_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let payer_user = group
        .members
        .iter()
        .find(|u| u.name == payload.payer)
        .ok_or_else(|| AppError::NotFound(format!("Payer '{}' not found", payload.payer)))?
        .clone();

    let mut participant_users = Vec::new();
    for name in &payload.participants {
        let user = group
            .members
            .iter()
            .find(|u| u.name == *name)
            .ok_or_else(|| AppError::NotFound(format!("Participant '{}' not found", name)))?
            .clone();
        participant_users.push(user);
    }

    let max_id = group.expenses.iter().map(|e| e.id).max().unwrap_or(0);
    let expense = Expense {
        id: max_id + 1,
        description: description.to_string(),
        amount: payload.amount,
        payer: payer_user,
        participants: participant_users,
        created_at: chrono::Utc::now().to_rfc3339(),
        category: payload.category,
        notes: payload.notes,
    };

    add_expense(expense.clone(), group);

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(expense))
}

pub async fn delete_expense(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Path(id): Path<usize>,
) -> AppResult<Json<serde_json::Value>> {
    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let group = app_data
        .groups
        .iter_mut()
        .find(|g| g.id == auth_user.current_group_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let index = group
        .expenses
        .iter()
        .position(|e| e.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Expense with id {} not found", id)))?;

    let expense = &group.expenses[index];
    if expense.category.as_deref() == Some("Settlement") {
        let payer_name = &expense.payer.name;
        let participant_name = expense.participants.first().map(|p| &p.name);
        if let Some(to_name) = participant_name {
            group
                .settled_settlements
                .retain(|s| !(s.from == *payer_name && s.to == *to_name));
        }
    }

    group.expenses.remove(index);

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn settle(
    State(state): State<SharedState>,
    auth_user: AuthUser,
    Json(payload): Json<SettleRequest>,
) -> AppResult<Json<Expense>> {
    if payload.amount <= 0.0 {
        return Err(AppError::BadRequest("Amount must be positive".to_string()));
    }
    if payload.from == payload.to {
        return Err(AppError::BadRequest(
            "Cannot settle with yourself".to_string(),
        ));
    }

    let mut app_data = state.write().map_err(|_| AppError::LockError)?;

    let group = app_data
        .groups
        .iter_mut()
        .find(|g| g.id == auth_user.current_group_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let from_user = group
        .members
        .iter()
        .find(|u| u.name == payload.from)
        .ok_or_else(|| AppError::BadRequest(format!("User '{}' not found", payload.from)))?
        .clone();

    let to_user = group
        .members
        .iter()
        .find(|u| u.name == payload.to)
        .ok_or_else(|| AppError::BadRequest(format!("User '{}' not found", payload.to)))?
        .clone();

    let max_id = group.expenses.iter().map(|e| e.id).max().unwrap_or(0);
    let expense = Expense {
        id: max_id + 1,
        description: format!("{} paid {}", payload.from, payload.to),
        amount: payload.amount,
        payer: from_user,
        participants: vec![to_user],
        created_at: chrono::Utc::now().to_rfc3339(),
        category: Some("Settlement".to_string()),
        notes: None,
    };

    add_expense(expense.clone(), group);

    group.settled_settlements.push(SettledSettlement {
        from: payload.from.clone(),
        to: payload.to.clone(),
        amount: payload.amount,
        settled_at: chrono::Utc::now().to_rfc3339(),
    });

    let app_data_clone = app_data.clone();
    drop(app_data);

    storage::save(&app_data_clone)?;

    Ok(Json(expense))
}
