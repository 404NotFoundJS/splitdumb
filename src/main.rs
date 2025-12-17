use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod logic;
mod models;
mod tests;

use cli::{Cli, Commands};
use logic::{add_expense, calculate_balances};
use models::{AppData, Expense, Group, User};

const DATA_FILE: &str = "app_data.json";

type SharedState = Arc<RwLock<AppData>>;

// Error handling
#[derive(Debug)]
enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

// Request/Response types
#[derive(serde::Deserialize)]
struct CreateExpenseRequest {
    description: String,
    amount: f64,
    payer: String,
    participants: Vec<String>,
    category: Option<String>,
    notes: Option<String>,
}

#[derive(serde::Deserialize)]
struct CreateUserRequest {
    name: String,
}

#[derive(serde::Deserialize)]
struct CreateGroupRequest {
    name: String,
}

#[derive(serde::Deserialize)]
struct SwitchGroupRequest {
    group_id: usize,
}

#[derive(serde::Deserialize)]
struct UpdateGroupRequest {
    name: String,
}

#[derive(serde::Serialize)]
struct BalanceResponse {
    balances: HashMap<String, f64>,
}

#[derive(serde::Serialize)]
struct Settlement {
    from: String,
    to: String,
    amount: f64,
}

#[derive(serde::Serialize)]
struct SettlementsResponse {
    settlements: Vec<Settlement>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut app_data = load_app_data();

    match &cli.command {
        Commands::Serve => {
            // Initialize logging
            tracing_subscriber::registry()
                .with(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "splitwise_rust=debug,tower_http=debug".into()),
                )
                .with(tracing_subscriber::fmt::layer())
                .init();

            let shared_state = Arc::new(RwLock::new(app_data));

            let app = Router::new()
                .route("/api/groups", get(list_groups).post(create_group))
                .route("/api/groups/current", put(switch_group))
                .route("/api/groups/{id}", put(update_group).delete(delete_group))
                .route("/api/group", get(get_current_group))
                .route("/api/expenses", post(create_expense))
                .route("/api/expenses/{id}", delete(delete_expense))
                .route("/api/users", post(create_user))
                .route("/api/users/{id}", delete(delete_user))
                .route("/api/balances", get(get_balances))
                .route("/api/settlements", get(get_settlements))
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .with_state(shared_state);

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            println!("Server listening on http://0.0.0.0:3000");
            axum::serve(listener, app).await.unwrap();
        }
        Commands::AddExpense {
            description,
            amount,
            payer,
            participants,
        } => {
            // CLI Logic - work with current group
            let group = app_data.groups.iter_mut()
                .find(|g| g.id == app_data.current_group_id)
                .expect("Current group not found");

            let payer_user = group
                .members
                .iter()
                .find(|u| u.name == *payer)
                .expect("Payer not found")
                .clone();

            let participant_users = participants
                .split(',')
                .map(|name| {
                    group
                        .members
                        .iter()
                        .find(|u| u.name.trim() == name.trim())
                        .expect("Participant not found")
                        .clone()
                })
                .collect();

            let expense = Expense {
                id: group.expenses.len() + 1,
                description: description.clone(),
                amount: *amount,
                payer: payer_user,
                participants: participant_users,
                created_at: chrono::Utc::now().to_rfc3339(),
                category: None,
                notes: None,
            };

            add_expense(expense, group);
            if let Err(e) = save_app_data(&app_data) {
                eprintln!("Error saving data: {}", e);
                std::process::exit(1);
            }
            println!("Expense added successfully.");
        }
        Commands::ShowBalances => {
            let group = app_data.groups.iter()
                .find(|g| g.id == app_data.current_group_id)
                .expect("Current group not found");
            let balances = calculate_balances(group);
            println!("Balances for group '{}':", group.name);
            for (user, balance) in balances {
                println!("{}: {:.2}", user, balance);
            }
        }
    }
}

async fn get_current_group(State(state): State<SharedState>) -> Result<Json<Group>, AppError> {
    let app_data = state.read()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    let current_id = app_data.current_group_id;
    let group = app_data.groups.iter()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    Ok(Json(group.clone()))
}

async fn list_groups(State(state): State<SharedState>) -> Result<Json<Vec<Group>>, AppError> {
    let app_data = state.read()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;
    Ok(Json(app_data.groups.clone()))
}

async fn create_group(
    State(state): State<SharedState>,
    Json(payload): Json<CreateGroupRequest>,
) -> Result<Json<Group>, AppError> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(AppError::BadRequest("Group name cannot be empty".to_string()));
    }

    let mut app_data = state.write()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    let max_id = app_data.groups.iter().map(|g| g.id).max().unwrap_or(0);
    let group = Group {
        id: max_id + 1,
        name: name.to_string(),
        members: vec![],
        expenses: vec![],
    };

    app_data.groups.push(group.clone());

    let app_data_clone = app_data.clone();
    drop(app_data);

    save_app_data(&app_data_clone)
        .map_err(|e| AppError::InternalError(format!("Failed to save: {}", e)))?;

    Ok(Json(group))
}

async fn switch_group(
    State(state): State<SharedState>,
    Json(payload): Json<SwitchGroupRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut app_data = state.write()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    // Check if group exists
    if !app_data.groups.iter().any(|g| g.id == payload.group_id) {
        return Err(AppError::NotFound(format!("Group with id {} not found", payload.group_id)));
    }

    app_data.current_group_id = payload.group_id;

    let app_data_clone = app_data.clone();
    drop(app_data);

    save_app_data(&app_data_clone)
        .map_err(|e| AppError::InternalError(format!("Failed to save: {}", e)))?;

    Ok(Json(serde_json::json!({ "success": true, "current_group_id": payload.group_id })))
}

async fn update_group(
    State(state): State<SharedState>,
    Path(id): Path<usize>,
    Json(payload): Json<UpdateGroupRequest>,
) -> Result<Json<Group>, AppError> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(AppError::BadRequest("Group name cannot be empty".to_string()));
    }

    let mut app_data = state.write()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    let group = app_data.groups.iter_mut()
        .find(|g| g.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Group with id {} not found", id)))?;

    group.name = name.to_string();
    let updated_group = group.clone();

    let app_data_clone = app_data.clone();
    drop(app_data);

    save_app_data(&app_data_clone)
        .map_err(|e| AppError::InternalError(format!("Failed to save: {}", e)))?;

    Ok(Json(updated_group))
}

async fn delete_group(
    State(state): State<SharedState>,
    Path(id): Path<usize>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut app_data = state.write()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    // Prevent deleting if it's the only group
    if app_data.groups.len() == 1 {
        return Err(AppError::BadRequest("Cannot delete the last group".to_string()));
    }

    // If deleting the current group, switch to another group first
    if app_data.current_group_id == id {
        // Find another group to switch to
        let new_group = app_data.groups.iter()
            .find(|g| g.id != id)
            .ok_or_else(|| AppError::InternalError("No other group found to switch to".to_string()))?;
        app_data.current_group_id = new_group.id;
    }

    let index = app_data.groups.iter().position(|g| g.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Group with id {} not found", id)))?;

    app_data.groups.remove(index);

    let app_data_clone = app_data.clone();
    drop(app_data);

    save_app_data(&app_data_clone)
        .map_err(|e| AppError::InternalError(format!("Failed to save: {}", e)))?;

    Ok(Json(serde_json::json!({ "success": true, "switched_group": app_data_clone.current_group_id })))
}

async fn create_user(
    State(state): State<SharedState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    // Validate input
    let name = payload.name.trim();
    if name.is_empty() {
        return Err(AppError::BadRequest("User name cannot be empty".to_string()));
    }
    if name.len() > 100 {
        return Err(AppError::BadRequest("User name too long (max 100 chars)".to_string()));
    }

    let mut app_data = state.write()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    let current_id = app_data.current_group_id;
    let group = app_data.groups.iter_mut()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    // Check for duplicate names
    if group.members.iter().any(|u| u.name == name) {
        return Err(AppError::BadRequest(format!("User '{}' already exists", name)));
    }

    // Generate proper ID
    let max_id = group.members.iter().map(|u| u.id).max().unwrap_or(0);
    let user = User {
        id: max_id + 1,
        name: name.to_string(),
    };

    group.members.push(user.clone());

    // Save outside of lock to prevent blocking
    let app_data_clone = app_data.clone();
    drop(app_data); // Release lock

    save_app_data(&app_data_clone)
        .map_err(|e| AppError::InternalError(format!("Failed to save: {}", e)))?;

    Ok(Json(user))
}

async fn create_expense(
    State(state): State<SharedState>,
    Json(payload): Json<CreateExpenseRequest>,
) -> Result<Json<Expense>, AppError> {
    // Validate input
    let description = payload.description.trim();
    if description.is_empty() {
        return Err(AppError::BadRequest("Description cannot be empty".to_string()));
    }
    if payload.amount <= 0.0 {
        return Err(AppError::BadRequest("Amount must be positive".to_string()));
    }
    if payload.participants.is_empty() {
        return Err(AppError::BadRequest("Must have at least one participant".to_string()));
    }

    let mut app_data = state.write()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    let current_id = app_data.current_group_id;
    let group = app_data.groups.iter_mut()
        .find(|g| g.id == current_id)
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

    // Generate proper ID
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

    // Save outside of lock to prevent blocking
    let app_data_clone = app_data.clone();
    drop(app_data); // Release lock

    save_app_data(&app_data_clone)
        .map_err(|e| AppError::InternalError(format!("Failed to save: {}", e)))?;

    Ok(Json(expense))
}

async fn delete_user(
    State(state): State<SharedState>,
    Path(id): Path<usize>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut app_data = state.write()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    let current_id = app_data.current_group_id;
    let group = app_data.groups.iter_mut()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let index = group.members.iter().position(|u| u.id == id)
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))?;

    // Check if user is involved in any expenses
    let user_name = &group.members[index].name;
    let has_expenses = group.expenses.iter().any(|e| {
        e.payer.name == *user_name || e.participants.iter().any(|p| p.name == *user_name)
    });

    if has_expenses {
        return Err(AppError::BadRequest(
            "Cannot delete user with existing expenses".to_string()
        ));
    }

    group.members.remove(index);

    let app_data_clone = app_data.clone();
    drop(app_data);

    save_app_data(&app_data_clone)
        .map_err(|e| AppError::InternalError(format!("Failed to save: {}", e)))?;

    Ok(Json(serde_json::json!({ "success": true })))
}

async fn delete_expense(
    State(state): State<SharedState>,
    Path(id): Path<usize>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut app_data = state.write()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    let current_id = app_data.current_group_id;
    let group = app_data.groups.iter_mut()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let index = group.expenses.iter().position(|e| e.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Expense with id {} not found", id)))?;

    group.expenses.remove(index);

    let app_data_clone = app_data.clone();
    drop(app_data);

    save_app_data(&app_data_clone)
        .map_err(|e| AppError::InternalError(format!("Failed to save: {}", e)))?;

    Ok(Json(serde_json::json!({ "success": true })))
}

async fn get_balances(State(state): State<SharedState>) -> Result<Json<BalanceResponse>, AppError> {
    let app_data = state.read()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    let current_id = app_data.current_group_id;
    let group = app_data.groups.iter()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let balances = calculate_balances(group);
    Ok(Json(BalanceResponse { balances }))
}

async fn get_settlements(State(state): State<SharedState>) -> Result<Json<SettlementsResponse>, AppError> {
    let app_data = state.read()
        .map_err(|e| AppError::InternalError(format!("Failed to acquire lock: {}", e)))?;

    let current_id = app_data.current_group_id;
    let group = app_data.groups.iter()
        .find(|g| g.id == current_id)
        .ok_or_else(|| AppError::NotFound("Current group not found".to_string()))?;

    let settlements = calculate_settlements(group);
    Ok(Json(SettlementsResponse { settlements }))
}

fn calculate_settlements(group: &Group) -> Vec<Settlement> {
    let balances = calculate_balances(group);
    let mut settlements = Vec::new();

    // Separate debtors and creditors
    let mut debtors: Vec<(String, f64)> = balances
        .iter()
        .filter(|(_, balance)| **balance < -0.01)
        .map(|(name, balance)| (name.clone(), -balance))
        .collect();

    let mut creditors: Vec<(String, f64)> = balances
        .iter()
        .filter(|(_, balance)| **balance > 0.01)
        .map(|(name, balance)| (name.clone(), *balance))
        .collect();

    // Sort for consistent results
    debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Match debtors with creditors
    let mut i = 0;
    let mut j = 0;

    while i < debtors.len() && j < creditors.len() {
        let debt = debtors[i].1;
        let credit = creditors[j].1;
        let amount = debt.min(credit);

        if amount > 0.01 {
            settlements.push(Settlement {
                from: debtors[i].0.clone(),
                to: creditors[j].0.clone(),
                amount: (amount * 100.0).round() / 100.0, // Round to 2 decimals
            });
        }

        debtors[i].1 -= amount;
        creditors[j].1 -= amount;

        if debtors[i].1 < 0.01 {
            i += 1;
        }
        if creditors[j].1 < 0.01 {
            j += 1;
        }
    }

    settlements
}

fn load_app_data() -> AppData {
    if let Ok(mut file) = fs::File::open(DATA_FILE) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap_or_else(|_| create_default_app_data())
    } else {
        create_default_app_data()
    }
}

fn create_default_app_data() -> AppData {
    let default_group = Group {
        id: 1,
        name: "Trip to Paris".to_string(),
        members: vec![
            User {
                id: 1,
                name: "Alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
            },
        ],
        expenses: vec![],
    };

    let app_data = AppData {
        groups: vec![default_group],
        current_group_id: 1,
    };

    let _ = save_app_data(&app_data); // Ignore errors when creating default
    app_data
}

fn save_app_data(app_data: &AppData) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(app_data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(DATA_FILE, json)?;
    Ok(())
}
