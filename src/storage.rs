use std::fs;
use std::io::Read;
use std::sync::OnceLock;
use tracing::{debug, warn};

use crate::errors::AppError;
use crate::models::AppData;

static DATA_FILE: OnceLock<String> = OnceLock::new();

pub fn init(path: &str) {
    DATA_FILE.get_or_init(|| path.to_string());
}

pub fn get_data_file() -> &'static str {
    DATA_FILE.get_or_init(|| "app_data.json".to_string())
}

pub fn load() -> AppData {
    let path = get_data_file();
    let backup_path = format!("{}.bak", path);

    // Try main file first
    if let Some(data) = try_load_file(path) {
        return data;
    }

    // Fall back to backup if main file is corrupted
    if let Some(data) = try_load_file(&backup_path) {
        warn!("loaded from backup file (main file was corrupted)");
        return data;
    }

    AppData {
        groups: vec![],
        users: vec![],
    }
}

fn try_load_file(path: &str) -> Option<AppData> {
    let mut file = fs::File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    serde_json::from_str(&contents).ok()
}

pub fn save(app_data: &AppData) -> Result<(), AppError> {
    use std::path::Path;

    let path = get_data_file();
    let tmp_path = format!("{}.tmp", path);
    let backup_path = format!("{}.bak", path);

    let json = serde_json::to_string_pretty(app_data)
        .map_err(|e| AppError::StorageError(std::io::Error::other(e)))?;

    // Write to temp file first
    fs::write(&tmp_path, &json)?;

    // Backup existing file
    if Path::new(path).exists() {
        fs::copy(path, &backup_path)?;
    }

    // Atomic rename
    fs::rename(&tmp_path, path)?;

    debug!(path, "data saved");
    Ok(())
}
