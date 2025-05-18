use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct SavedData {
    pub tracking_numbers: Vec<String>,
}

pub fn get_data_file() -> Option<PathBuf> {
    ProjectDirs::from("io.github", "alrick", "simple_package_tracker").map(|dirs| {
        let data_dir = dirs.data_dir();
        fs::create_dir_all(data_dir).ok();
        let file_path = data_dir.join("saved_numbers.json");
        if !file_path.exists() {
            let default_data = SavedData::default();
            let json = serde_json::to_string(&default_data).unwrap_or_default();
            fs::write(&file_path, json).ok();
        }
        file_path
    })
}

pub fn save_tracking_numbers(numbers: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(file_path) = get_data_file() {
        let data = SavedData {
            tracking_numbers: numbers.to_vec(),
        };
        let json = serde_json::to_string(&data)?;
        fs::write(file_path, json)?;
    }
    Ok(())
}

pub fn load_tracking_numbers() -> Vec<String> {
    get_data_file()
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|content| serde_json::from_str::<SavedData>(&content).ok())
        .map(|data| data.tracking_numbers)
        .unwrap_or_default()
}
