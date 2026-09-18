use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub klavye_duzeni: String,
    pub tema: String,
    pub ses_efektleri: bool,
    pub varsayilan_sure_sn: u64,
    pub varsayilan_zorluk: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            klavye_duzeni: "Q".to_string(),
            tema: "koyu".to_string(),
            ses_efektleri: false,
            varsayilan_sure_sn: 60,
            varsayilan_zorluk: "orta".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(config) = serde_json::from_str(&data) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, data);
        }
    }

    fn config_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "typing_trainer") {
            proj_dirs.config_dir().join("config.json")
        } else {
            PathBuf::from("config.json")
        }
    }
}
