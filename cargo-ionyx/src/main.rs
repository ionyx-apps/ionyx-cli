use clap::{Parser, Subcommand};
use std::path::Path;
use std::fs;
use std::process;
use walkdir::WalkDir;

mod commands;

#[derive(Parser)]
#[command(name = "ionyx")]
#[command(about = "Ionyx Framework CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start development environment (default)
    Dev {
        /// Port number
        #[arg(short, long, default_value = "5173")]
        port: u16,
        /// Hot reload
        #[arg(short = 'H', long)]
        hot: bool,
    },
    /// Create a new Ionyx project
    Create {
        /// Project name
        name: Option<String>,
        /// Project template
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Build the project
    Build {
        /// Build target
        #[arg(short, long, default_value = "release")]
        target: String,
    },
    /// Run the application
    Run {
        /// Port number
        #[arg(short, long, default_value = "5173")]
        port: u16,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for lock file to prevent multiple instances
    let lock_file = "ionyx-dev.lock";
    if Path::new(lock_file).exists() {
        eprintln!("❌ Another Ionyx dev instance is already running. Use Ctrl+C to stop it first.");
        process::exit(1);
    }

    // Create lock file
    if let Err(e) = fs::write(lock_file, "locked") {
        eprintln!("❌ Failed to create lock file: {}", e);
        process::exit(1);
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Parse CLI arguments
    let cli = Cli::parse();
    
    match cli.command.unwrap_or(Commands::Dev { port: 5173, hot: false }) {
        Commands::Dev { port, hot } => {
            if let Err(e) = rt.block_on(commands::dev::execute(port, hot)) {
                // Remove lock file on error
                let _ = fs::remove_file(lock_file);
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Commands::Create { name, template } => {
            let template_name = template.as_deref().unwrap_or("react");
            
            // Get CLI repo path from binary location
            let exe_path = std::env::current_exe()?;
            let cli_repo_path = exe_path.parent().unwrap().parent().unwrap(); // bin/ -> CLI repo
            let template_path = cli_repo_path.join("ionyx-cli/src/templates").join(template_name);

            if !Path::new(&template_path).exists() {
                eprintln!("❌ Template '{}' not found at: {:?}", template_name, template_path);
                process::exit(1);
            }

            let project_name = name.as_deref().unwrap_or("ionyx-app");
            let project_path = if project_name == "." {
                std::env::current_dir()?
            } else {
                Path::new(project_name).to_path_buf()
            };

            if project_name != "." && project_path.exists() {
                eprintln!("❌ Directory '{}' already exists", project_name);
                process::exit(1);
            }

            if project_name != "." {
                fs::create_dir(&project_path)?;
            }

            for entry in WalkDir::new(&template_path) {
                let entry = entry?;
                let path = entry.path();
                let relative_path = path.strip_prefix(&template_path)?;
                let target_path = project_path.join(relative_path);

                if path.is_dir() {
                    fs::create_dir_all(&target_path)?;
                } else {
                    fs::copy(path, &target_path)?;
                }
            }

            println!("✅ Project '{}' created successfully!", project_name);
        }
        Commands::Build { target } => {
            println!("🔨 Building Ionyx project...");
            println!("Target: {}", target);
            // TODO: Implement build command
        }
        Commands::Run { port } => {
            println!("🚀 Running Ionyx application on port {}...", port);
            // TODO: Implement run command
        }
    }

    // Remove lock file on success
    let _ = fs::remove_file(lock_file);
    Ok(())
}
