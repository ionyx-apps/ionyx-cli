use anyhow::Result;
use colored::*;
use std::process::Command;
use std::path::Path;

pub async fn execute(port: u16) -> Result<()> {
    println!("🚀 Running Ionyx application on port {}", port.to_string().cyan());
    
    if !Path::new("package.json").exists() {
        return Err(anyhow::anyhow!("No package.json found. Please run this command in an Ionyx project directory."));
    }
    
    // Check if node_modules exists, if not, install dependencies
    if !Path::new("node_modules").exists() {
        println!("📦 Installing dependencies...");
        let output = Command::new("npm")
            .arg("install")
            .output()
            .expect("Failed to run npm install");
        
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    // Check if build exists, if not, build first
    if !Path::new("dist").exists() {
        println!("📦 No build found, building first...");
        let output = Command::new("npm")
            .arg("run")
            .arg("build")
            .output()
            .expect("Failed to run build command");
        
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    
    // Run application
    let mut cmd = Command::new("npm");
    cmd.arg("run").arg("start");
    
    // Set port if different from default
    if port != 5173 {
        cmd.env("PORT", port.to_string());
    }
    
    println!("🌐 Application started at: http://localhost:{}", port);
    
    let status = cmd.status()?;
    
    if !status.success() {
        return Err(anyhow::anyhow!("Application failed to start"));
    }
    
    Ok(())
}
