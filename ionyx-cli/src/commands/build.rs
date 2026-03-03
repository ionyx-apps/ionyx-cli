use anyhow::Result;
use colored::*;
use tokio::process::Command;
use std::path::Path;

pub async fn execute(target: &str, minify: bool) -> Result<()> {
    println!("🔨 Building Ionyx project for target: {}", target.cyan());
    
    if !Path::new("package.json").exists() {
        return Err(anyhow::anyhow!("No package.json found. Please run this command in an Ionyx project directory."));
    }
    
    // 1. Build Frontend
    println!("🌐 Building frontend...");
    let fe_status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "npm", "run", "build"])
            .env("BUILD_TARGET", target)
            .env("MINIFY", if minify { "true" } else { "false" })
            .status()
            .await?
    } else {
        Command::new("npm")
            .args(&["run", "build"])
            .env("BUILD_TARGET", target)
            .env("MINIFY", if minify { "true" } else { "false" })
            .status()
            .await?
    };

    if !fe_status.success() {
        return Err(anyhow::anyhow!("Frontend build failed"));
    }
    
    // 2. Build Backend
    println!("🦀 Building Rust backend host...");
    let mut be_cmd = Command::new("cargo");
    be_cmd.args(&["build", "--release"]);
    
    if Path::new("src-ionyx").exists() {
        be_cmd.current_dir("src-ionyx");
    }

    let be_status = be_cmd.status().await?;
    if !be_status.success() {
        return Err(anyhow::anyhow!("Backend build failed"));
    }
    
    println!("\n✅ Full build completed successfully!");
    println!("📦 Backend binary: src-ionyx/target/release/ionyx-host");
    
    Ok(())
}
