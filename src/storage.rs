use std::fs;
use std::io::Read;
use std::sync::OnceLock;

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
    if let Ok(mut file) = fs::File::open(path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok()
            && let Ok(data) = serde_json::from_str(&contents)
        {
            return data;
        }
    }
    AppData {
        groups: vec![],
        users: vec![],
    }
}

pub fn save(app_data: &AppData) -> Result<(), AppError> {
    let path = get_data_file();
    let json = serde_json::to_string_pretty(app_data)
        .map_err(|e| AppError::StorageError(std::io::Error::other(e)))?;
    fs::write(path, json)?;
    Ok(())
}
