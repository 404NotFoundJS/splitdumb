use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: usize,
    pub phone: String,
    pub name: String,
    pub token: String,
    #[serde(default)]
    pub current_group_id: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Group {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub members: Vec<User>,
    #[serde(default)]
    pub expenses: Vec<Expense>,
    #[serde(default)]
    pub simplify_debts: bool,
    #[serde(default)]
    pub settled_settlements: Vec<SettledSettlement>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettledSettlement {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub settled_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: usize,
    pub description: String,
    pub amount: f64,
    pub payer: User,
    pub participants: Vec<User>,
    #[serde(default = "default_timestamp")]
    pub created_at: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppData {
    pub groups: Vec<Group>,
    #[serde(default)]
    pub users: Vec<AuthUser>,
}

fn default_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
}
