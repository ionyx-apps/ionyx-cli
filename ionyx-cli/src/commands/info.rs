use anyhow::Result;
use colored::*;
use std::process::Command;

pub async fn execute() -> Result<()> {
    println!("\n{}", "Ionyx Environment".bold().bright_blue());

    println!("\n{}", "Operating System".bold());
    let info = os_info::get();
    println!("  {}: {} {} {}", 
        "OS".green(), 
        info.os_type(), 
        info.version(), 
        info.bitness()
    );
    
    // Get Rust version
    println!("\n{}", "Rust Environment".bold());
    let mut rustc_cmd = Command::new("rustc");
    rustc_cmd.arg("--version");
    
    match rustc_cmd.output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  {}: {}", "rustc".green(), version.trim());
        }
        _ => {
            println!("  {}: {}", "rustc".red(), "Not installed or not in PATH");
        }
    }

    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.arg("--version");
    
    match cargo_cmd.output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  {}: {}", "cargo".green(), version.trim());
        }
        _ => {
            println!("  {}: {}", "cargo".red(), "Not installed or not in PATH");
        }
    }

    // Node environment
    println!("\n{}", "Node Environment".bold());
    let mut node_cmd = Command::new("node");
    node_cmd.arg("--version");
    
    match node_cmd.output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  {}: {}", "node".green(), version.trim());
        }
        _ => {
            println!("  {}: {}", "node".red(), "Not installed or not in PATH");
        }
    }

    let mut npm_cmd = Command::new("npm");
    npm_cmd.arg("--version");
    
    match npm_cmd.output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  {}: {}", "npm".green(), version.trim());
        }
        _ => {
            println!("  {}: {}", "npm".red(), "Not installed or not in PATH");
        }
    }

    // CLI version
    println!("\n{}", "Ionyx Framework".bold());
    println!("  {}: {}", "ionyx-cli".green(), env!("CARGO_PKG_VERSION"));
    
    println!();
    Ok(())
}
