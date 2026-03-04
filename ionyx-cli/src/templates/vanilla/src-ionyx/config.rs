use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub ionyx: IonyxConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub fullscreen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonyxConfig {
    pub permissions: Vec<String>,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub allowed_paths: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = "ionyx.config.json";
        
        if !Path::new(config_path).exists() {
            return Ok(Config::default());
        }
        
        let content = std::fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                title: "my-ionyx-app".to_string(),
                width: 1200,
                height: 800,
                resizable: true,
                fullscreen: false,
            },
            ionyx: IonyxConfig {
                permissions: vec![
                    "fs".to_string(),
                    "network".to_string(),
                    "os_info".to_string(),
                ],
                security: SecurityConfig {
                    allowed_paths: vec!["./app-data".to_string()],
                },
            },
        }
    }
}
