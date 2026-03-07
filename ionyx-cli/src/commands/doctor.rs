use anyhow::Result;
use colored::*;
use std::process::Command;
use std::path::Path;

pub async fn execute() -> Result<()> {
    println!("\n{}", "⚕️ Ionyx Doctor - Diagnostic Report".bold().bright_blue());
    println!("{}", "=====================================".bright_blue());

    let mut issues = 0;
    let mut warnings = 0;

    // 1. Operating System
    print_check("Operating System", true, "");
    let info = os_info::get();
    println!("   {} {} {} {}", info.os_type(), info.version(), info.bitness(), info.architecture().unwrap_or(""));

    // 2. Rust Environment
    println!("\n{}", "🦀 Rust Environment".bold());
    let rustc = check_version("rustc", &["--version"]);
    let cargo = check_version("cargo", &["--version"]);
    
    match (rustc.as_ref(), cargo.as_ref()) {
        (Some(r), Some(c)) => {
            print_check("Rust/Cargo", true, "Detected");
            println!("   rustc: {}", r);
            println!("   cargo: {}", c);
        }
        _ => {
            if rustc.is_none() { print_check("rustc", false, "Not found"); }
            if cargo.is_none() { print_check("cargo", false, "Not found"); }
            println!("   {} Please install Rust from https://rustup.rs", "!".yellow());
            issues += 1;
        }
    }

    // 3. Node Environment
    println!("\n{}", "🟢 Node.js Environment".bold());
    let node = check_version("node", &["--version"]);
    let npm = check_version("npm", &["--version"]);
    
    match (node.as_ref(), npm.as_ref()) {
        (Some(n), Some(m)) => {
            print_check("Node/NPM", true, "Detected");
            println!("   node: {}", n);
            println!("   npm:  {}", m);
        }
        _ => {
            if node.is_none() { print_check("node", false, "Not found"); }
            if npm.is_none() { print_check("npm", false, "Not found (Ensure it's in your PATH)"); }
            println!("   {} Please install Node.js from https://nodejs.org", "!".yellow());
            issues += 1;
        }
    }

    // 4. Bundling Tools
    println!("\n{}", "📦 Bundling Tools".bold());
    
    #[cfg(target_os = "windows")]
    {
        // NSIS
        if let Some(v) = check_version("makensis", &["/VERSION"]) {
            print_check("NSIS (for EXE)", true, &format!("Version {}", v));
        } else {
            print_warning("NSIS (for EXE)", "Not found. Required for building .exe installers. Install from https://nsis.sourceforge.io");
            warnings += 1;
        }

        // WiX
        if let Some(_) = check_version("candle", &["-?"]) {
            print_check("WiX Toolset (for MSI)", true, "Detected");
        } else {
            print_warning("WiX Toolset (for MSI)", "Not found. Required for building .msi installers. Install from https://wixtoolset.org");
            warnings += 1;
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(_) = check_version("hdiutil", &["help"]) {
            print_check("hdiutil (for DMG)", true, "Detected");
        } else {
            print_warning("hdiutil (for DMG)", "Not found. Required for building .dmg installers.");
            warnings += 1;
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(_) = check_version("appimagetool", &["--version"]) {
            print_check("appimagetool (for AppImage)", true, "Detected");
        } else {
            print_warning("appimagetool (for AppImage)", "Not found. Required for building .AppImage. Install from https://appimage.org");
            warnings += 1;
        }
    }

    // 5. Project Context
    println!("\n{}", "🏠 Project Context".bold());
    if Path::new("ionyx.config.json").exists() || Path::new("ionyx.config.toml").exists() {
        print_check("Ionyx Project", true, "You are inside an Ionyx project directory.");
    } else {
        println!("   {} {}", "!".yellow(), "You are NOT in an Ionyx project directory. Some commands like 'build' or 'dev' will not work.");
    }

    // Summary
    println!("\n{}", "Summary".bold());
    if issues == 0 && warnings == 0 {
        println!("  {} No issues found! Your environment is perfectly configured for Ionyx. 🚀", "✓".green());
    } else {
        if issues > 0 {
            println!("  {} Found {} critical issue(s). Review the report above for fixes.", "✗".red(), issues);
        }
        if warnings > 0 {
            println!("  {} Found {} warning(s). You can still develop, but some features (like bundling) might be limited.", "!".yellow(), warnings);
        }
    }

    println!();
    Ok(())
}

fn print_check(title: &str, success: bool, msg: &str) {
    if success {
        println!(" {} {}: {}", "✓".green(), title.bold(), msg.dimmed());
    } else {
        println!(" {} {}: {}", "✗".red(), title.bold(), msg.bright_red());
    }
}

fn print_warning(title: &str, msg: &str) {
    println!(" {} {}: {}", "!".yellow(), title.bold(), msg.yellow());
}

fn check_version(cmd: &str, args: &[&str]) -> Option<String> {
    // Platform specific variations
    let cmd_variations = if cfg!(target_os = "windows") {
        match cmd {
            "npm" => vec!["npm.cmd", "npm"],
            "makensis" => vec!["makensis.exe", "makensis"],
            "candle" => vec!["candle.exe", "candle"],
            _ => vec![cmd],
        }
    } else {
        vec![cmd]
    };

    for c in cmd_variations {
        match Command::new(c).args(args).output() {
            Ok(output) if output.status.success() => {
                let out = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return Some(out.lines().next().unwrap_or("").trim().to_string());
            }
            _ => continue,
        }
    }
    None
}
