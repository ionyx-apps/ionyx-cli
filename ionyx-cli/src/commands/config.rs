use anyhow::Result;
use colored::*;
use crate::config::ProjectConfig;
use std::fs;

pub async fn execute(action: crate::ConfigAction) -> Result<()> {
    match action {
        crate::ConfigAction::Get { key } => {
            let config = ProjectConfig::load()?;
            if let Some(value) = config.get(&key) {
                println!("{} = {}", key.cyan(), value.green());
            } else {
                println!("{} = {}", key.cyan(), "not found".red());
            }
        }
        crate::ConfigAction::Set { key, value } => {
            let mut config = ProjectConfig::load()?;
            config.set(&key, &value);
            config.save()?;
            println!("✅ Set {} = {}", key.cyan(), value.green());
        }
        crate::ConfigAction::List => {
            let config = ProjectConfig::load()?;
            println!("📋 Configuration:");
            for (key, value) in config.get_all() {
                println!("  {} = {}", key.cyan(), value.green().to_string());
            }
        }
        crate::ConfigAction::Reset => {
            ProjectConfig::reset()?;
            println!("✅ Configuration reset to defaults");
        }
    }
    
    Ok(())
}
