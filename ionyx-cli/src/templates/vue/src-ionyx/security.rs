use std::path::PathBuf;
use anyhow::Result;

pub struct SecurityManager {
    config: crate::config::Config,
}

impl SecurityManager {
    pub fn new(config: crate::config::Config) -> Self {
        Self { config }
    }

    pub fn is_command_allowed(&self, command: &str) -> bool {
        self.config.ionyx.permissions.contains(&command.to_string())
    }

    pub fn check_file_access(&self, path: &PathBuf) -> Result<()> {
        let path_str = path.to_string_lossy();
        
        // Check if path is in allowed paths
        for allowed_path in &self.config.ionyx.security.allowed_paths {
            if path_str.starts_with(allowed_path) {
                return Ok(());
            }
        }
        
        Err(anyhow::anyhow!("Access to 

 not allowed", path_str))
    }

    pub fn sanitize_filename(&self, filename: &str) -> String {
        // Basic filename sanitization
        filename
            .replace("..", "")
            .replace("/", "_")
            .replace("\\", "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || c == 
_ || c == - || c == .)
            .collect()
    }

    pub fn get_allowed_directories(&self) -> Vec<String> {
        self.config.ionyx.security.allowed_paths.clone()
    }

    pub fn has_network_permission(&self) -> bool {
        self.config.ionyx.permissions.contains(&"network".to_string())
    }

    pub fn has_os_info_permission(&self) -> bool {
        self.config.ionyx.permissions.contains(&"os_info".to_string())
    }

    pub fn has_dialogs_permission(&self) -> bool {
        self.config.ionyx.permissions.contains(&"dialogs".to_string())
    }
}
