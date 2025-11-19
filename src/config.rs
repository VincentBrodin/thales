use std::{
    fs::{self, OpenOptions},
    io::Read,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{APP_NAME, CONFIG_FILE};

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub name: String,
    pub on_added: Option<Vec<String>>,
    pub on_removed: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub monitors: Vec<Monitor>,
}

impl Config {
    pub fn load_from_file() -> Result<Self, crate::Error> {
        let config_dir = config_dir()?;
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(config_dir.join(CONFIG_FILE))?;
        let mut raw = String::new();
        let _ = file.read_to_string(&mut raw)?;
        toml::from_str(&raw).map_err(|err| crate::Error::TomlDeError(err))
    }
}

fn config_dir() -> Result<PathBuf, crate::Error> {
    if let Some(config) = dirs::config_dir() {
        let app_config = config.join(APP_NAME);
        if !app_config.exists() {
            fs::create_dir_all(app_config.clone())?
        }
        Ok(app_config)
    } else {
        Err(crate::Error::ConfigDirectoryError)
    }
}
