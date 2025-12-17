pub mod expenses;
pub mod groups;
pub mod users;

use crate::models::AppData;
use std::sync::{Arc, RwLock};

pub type SharedState = Arc<RwLock<AppData>>;
