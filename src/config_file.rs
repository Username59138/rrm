use crate::{CONFIG_DIR, CONFIG_FILE};
use dirs::config_dir;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Variables {
    pub allow_root_deletion: Option<bool>,
    pub confirm_deleting: Option<bool>,
}

#[derive(Deserialize)]
pub struct Lists {
    pub blacklist_files: Option<Vec<PathBuf>>,
    pub very_blacklist_files: Option<Vec<PathBuf>>,
    pub confirm_files: Option<Vec<PathBuf>>,
}

#[derive(Deserialize)]
pub struct ConfigFile {
    pub variables: Option<Variables>,
    pub lists: Option<Lists>,
}

impl ConfigFile {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let path: PathBuf = PathBuf::from(format!(
            "{}/{}/{}",
            config_dir()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
            CONFIG_DIR,
            CONFIG_FILE
        ));
        if !path.exists() {
            fs::write(&path, String::new())?;
        }
        let content = fs::read_to_string(&path)?;
        let config: ConfigFile = toml::from_str(&content)?;
        Ok(config)
    }
}
