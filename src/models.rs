use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: usize,
    pub name: String,
    pub members: Vec<User>,
    pub expenses: Vec<Expense>,
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
    pub current_group_id: usize,
}

fn default_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
}
