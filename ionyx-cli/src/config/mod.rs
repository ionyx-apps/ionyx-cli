use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub r#type: String,
    pub src: String,
    pub dist: String,
    pub dev: DevConfig,
    pub build: BuildConfig,
    pub ionyx: IonyxConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevConfig {
    pub port: u16,
    pub hot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub target: String,
    pub assets: String,
    pub minify: bool,
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

impl ProjectConfig {
    pub fn load() -> Result<Self, anyhow::Error> {
        let config_path = "ionyx.config.json";
        
        if !Path::new(config_path).exists() {
            return Ok(ProjectConfig::default());
        }
        
        let content = fs::read_to_string(config_path)?;
        let config: ProjectConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<(), anyhow::Error> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write("ionyx.config.json", content)?;
        Ok(())
    }
    
    pub fn reset() -> Result<(), anyhow::Error> {
        let default_config = ProjectConfig::default();
        default_config.save()
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "name" => Some(self.name.clone()),
            "version" => Some(self.version.clone()),
            "description" => Some(self.description.clone()),
            "type" => Some(self.r#type.clone()),
            "src" => Some(self.src.clone()),
            "dist" => Some(self.dist.clone()),
            "dev.port" => Some(self.dev.port.to_string()),
            "dev.hot" => Some(self.dev.hot.to_string()),
            "build.target" => Some(self.build.target.clone()),
            "build.assets" => Some(self.build.assets.clone()),
            "build.minify" => Some(self.build.minify.to_string()),
            _ => None,
        }
    }
    
    pub fn set(&mut self, key: &str, value: &str) {
        match key {
            "name" => self.name = value.to_string(),
            "version" => self.version = value.to_string(),
            "description" => self.description = value.to_string(),
            "type" => self.r#type = value.to_string(),
            "src" => self.src = value.to_string(),
            "dist" => self.dist = value.to_string(),
            "dev.port" => {
                if let Ok(port) = value.parse() {
                    self.dev.port = port;
                }
            }
            "dev.hot" => {
                if let Ok(hot) = value.parse() {
                    self.dev.hot = hot;
                }
            }
            "build.target" => self.build.target = value.to_string(),
            "build.assets" => self.build.assets = value.to_string(),
            "build.minify" => {
                if let Ok(minify) = value.parse() {
                    self.build.minify = minify;
                }
            }
            _ => {}
        }
    }
    
    pub fn get_all(&self) -> Vec<(String, String)> {
        vec![
            ("name".to_string(), self.name.clone()),
            ("version".to_string(), self.version.clone()),
            ("description".to_string(), self.description.clone()),
            ("type".to_string(), self.r#type.clone()),
            ("src".to_string(), self.src.clone()),
            ("dist".to_string(), self.dist.clone()),
            ("dev.port".to_string(), self.dev.port.to_string()),
            ("dev.hot".to_string(), self.dev.hot.to_string()),
            ("build.target".to_string(), self.build.target.clone()),
            ("build.assets".to_string(), self.build.assets.clone()),
            ("build.minify".to_string(), self.build.minify.to_string()),
        ]
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "my-ionyx-app".to_string(),
            version: "1.0.0".to_string(),
            description: "Ionyx Framework Application".to_string(),
            r#type: "app".to_string(),
            src: "src".to_string(),
            dist: "dist".to_string(),
            dev: DevConfig {
                port: 5173,
                hot: true,
            },
            build: BuildConfig {
                target: "web".to_string(),
                assets: "dist".to_string(),
                minify: false,
            },
            ionyx: IonyxConfig {
                permissions: vec!["fs".to_string(), "network".to_string(), "os_info".to_string()],
                security: SecurityConfig {
                    allowed_paths: vec!["./app-data".to_string()],
                },
            },
        }
    }
}
