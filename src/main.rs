use axum::{
    Json, Router,
    routing::{delete, get, post, put},
};
use clap::Parser;
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod errors;
mod handlers;
mod logic;
mod models;
mod storage;
mod tests;

use cli::{Cli, Commands};
use handlers::{auth, expenses, groups, users};
use logic::{add_expense, calculate_balances, calculate_settlements};
use models::{Expense, User};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port, data_file } => {
            run_server(port, &data_file).await;
        }
        Commands::AddExpense {
            description,
            amount,
            payer,
            participants,
            data_file,
        } => {
            storage::init(&data_file);
            let mut app_data = storage::load();

            let group = app_data
                .groups
                .first_mut()
                .expect("No groups found. Create a group first.");

            let payer_user = group
                .members
                .iter()
                .find(|u| u.name == payer)
                .expect("Payer not found")
                .clone();

            let participant_users: Vec<User> = participants
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
                description,
                amount,
                payer: payer_user,
                participants: participant_users,
                created_at: chrono::Utc::now().to_rfc3339(),
                category: None,
                notes: None,
            };

            add_expense(expense, group);

            if let Err(e) = storage::save(&app_data) {
                eprintln!("Error saving data: {}", e);
                std::process::exit(1);
            }
            println!("Expense added successfully.");
        }
        Commands::ShowBalances { data_file } => {
            storage::init(&data_file);
            let app_data = storage::load();

            let group = app_data
                .groups
                .first()
                .expect("No groups found. Create a group first.");

            let balances = calculate_balances(group);
            println!("Balances for group '{}':", group.name);
            for (user, balance) in balances {
                let sign = if balance >= 0.0 { "+" } else { "" };
                println!("  {}: {}${:.2}", user, sign, balance);
            }
        }
        Commands::ShowSettlements { data_file } => {
            storage::init(&data_file);
            let app_data = storage::load();

            let group = app_data
                .groups
                .first()
                .expect("No groups found. Create a group first.");

            let settlements = calculate_settlements(group);
            println!("Settlements for group '{}':", group.name);
            if settlements.is_empty() {
                println!("  All settled up!");
            } else {
                for settlement in settlements {
                    println!(
                        "  {} pays {} ${:.2}",
                        settlement.from, settlement.to, settlement.amount
                    );
                }
            }
        }
    }
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn run_server(port: u16, data_file: &str) {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "splitwise_rust=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize storage
    storage::init(data_file);
    tracing::info!(data_file, "initializing storage");
    let app_data = storage::load();
    tracing::info!(
        groups = app_data.groups.len(),
        users = app_data.users.len(),
        "loaded data"
    );
    let shared_state = Arc::new(RwLock::new(app_data));

    let app = Router::new()
        // Health check
        .route("/api/health", get(health_check))
        // Auth routes
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/me", get(auth::get_me))
        // Group routes
        .route(
            "/api/groups",
            get(groups::list_groups).post(groups::create_group),
        )
        .route("/api/groups/current", put(groups::switch_group))
        .route(
            "/api/groups/{id}",
            put(groups::update_group).delete(groups::delete_group),
        )
        .route("/api/group", get(groups::get_current_group))
        // Expense routes
        .route("/api/expenses", post(expenses::create_expense))
        .route(
            "/api/expenses/{id}",
            put(expenses::update_expense).delete(expenses::delete_expense),
        )
        .route("/api/settle", post(expenses::settle))
        // User routes
        .route("/api/users", post(users::create_user))
        .route("/api/users/{id}", delete(users::delete_user))
        // Balance and settlement routes
        .route("/api/balances", get(groups::get_balances))
        .route("/api/settlements", get(groups::get_settlements))
        .route("/api/simplify", post(groups::toggle_simplify))
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(shared_state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
