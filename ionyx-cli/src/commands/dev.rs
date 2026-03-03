use anyhow::Result;
use colored::*;
use tokio::process::Command;
use std::path::Path;

pub async fn execute(port: u16, hot: bool) -> Result<()> {
    println!("🔥 Starting Ionyx development environment on port {}", port.to_string().cyan());
    
    if !Path::new("package.json").exists() {
        return Err(anyhow::anyhow!("No package.json found. Please run this command in an Ionyx project directory."));
    }
    
    // Check if node_modules exists, if not, install dependencies
    if !Path::new("node_modules").exists() {
        println!("📦 Installing dependencies...");
        let status = Command::new("npm")
            .arg("install")
            .status()
            .await?;
        
        if !status.success() {
            return Err(anyhow::anyhow!("npm install failed"));
        }
    }
    
    println!("🌐 Starting frontend dev server...");
    let mut fe_cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(&["/C", "npm", "run", "dev"]);
        c
    } else {
        let mut c = Command::new("npm");
        c.args(&["run", "dev"]);
        c
    };

    if port != 5173 {
        fe_cmd.env("PORT", port.to_string());
    }
    if hot {
        fe_cmd.env("HOT_RELOAD", "true");
    }

    let mut fe_child = fe_cmd.spawn()?;

    println!("🦀 Starting Rust backend host...");
    let mut be_cmd = Command::new("cargo");
    be_cmd.args(&["run", "--bin", "ionyx-host"]);
    
    if Path::new("src-ionyx").exists() {
        be_cmd.current_dir("src-ionyx");
    }

    let mut be_child = be_cmd.spawn()?;

    println!("\n🚀 Ionyx is running!");
    println!("   Frontend: http://localhost:{}", port);
    println!("   Backend:  src-ionyx (ionyx-host)");
    println!("\nPress Ctrl+C to stop both processes.\n");

    tokio::select! {
        res = fe_child.wait() => {
            let status = res?;
            println!("Frontend process exited with status: {}", status);
        }
        res = be_child.wait() => {
            let status = res?;
            println!("Backend process exited with status: {}", status);
        }
    }

    // Kill the other process if one exits
    let _ = fe_child.kill().await;
    let _ = be_child.kill().await;
    
    Ok(())
}
