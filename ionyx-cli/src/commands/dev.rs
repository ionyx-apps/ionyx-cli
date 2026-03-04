use anyhow::Result;
use colored::*;
use tokio::process::Command;
use std::path::Path;
use std::net::TcpStream;
use std::time::Duration;

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
    fe_cmd.env("IONYX_DEV_HOST", "127.0.0.1");

    let mut fe_child = fe_cmd.spawn()?;

    // Wait for Vite to be ready
    println!("⏳ Waiting for frontend server to be ready...");
    let mut attempts = 0;
    let max_attempts = 60; // 30 seconds total (60 * 500ms)
    
    while attempts < max_attempts {
        match TcpStream::connect(&format!("127.0.0.1:{}", port)) {
            Ok(_) => {
                println!("✅ Frontend server is ready!");
                break;
            }
            Err(_) => {
                attempts += 1;
                if attempts % 10 == 0 {
                    println!("   Still waiting... ({}s)", (attempts * 500) / 1000);
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }
    
    if attempts >= max_attempts {
        return Err(anyhow::anyhow!("Frontend server failed to start within 30 seconds"));
    }

    println!("🦀 Starting Rust backend host...");
    let mut be_cmd = Command::new("cargo");
    be_cmd.args(&["run", "--bin", "ionyx-host"]);
    
    if Path::new("src-ionyx").exists() {
        be_cmd.current_dir("src-ionyx");
    }
    
    // Set environment variable for backend
    be_cmd.env("IONYX_URL", &format!("http://127.0.0.1:{}", port));

    let mut be_child = be_cmd.spawn()?;

    println!("\n🚀 Ionyx is running!");
    println!("   Frontend: http://127.0.0.1:{}", port);
    println!("   Backend: src-ionyx (ionyx-host)");
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
