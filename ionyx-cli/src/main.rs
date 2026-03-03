use clap::{Parser, Subcommand};
use colored::*;
use anyhow::Result;

mod commands;
mod config;
mod templates;

#[derive(Parser)]
#[command(name = "ionyx")]
#[command(about = "Ionyx Framework CLI Tool")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Ionyx project
    Create {
        /// Project name
        name: Option<String>,
        /// Project template (basic, react, svelte, vue, vanilla, leptos, angular)
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Start development server
    Dev {
        /// Port number
        #[arg(short, long, default_value = "5173")]
        port: u16,
        /// Hot reload
        #[arg(short, long)]
        hot: bool,
    },
    /// Build the project
    Build {
        /// Build target
        #[arg(short, long, default_value = "web")]
        target: String,
        /// Minify output
        #[arg(short, long)]
        minify: bool,
    },
    /// Run the application
    Run {
        /// Port number
        #[arg(short, long, default_value = "5173")]
        port: u16,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Plugin management
    Plugin {
        #[command(subcommand)]
        action: PluginAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Get configuration value
    Get { key: String },
    /// Set configuration value
    Set { key: String, value: String },
    /// List all configuration
    List,
    /// Reset configuration
    Reset,
}

#[derive(Subcommand)]
enum PluginAction {
    /// Install a plugin
    Install { name: String },
    /// Uninstall a plugin
    Uninstall { name: String },
    /// List installed plugins
    List,
    /// Update a plugin
    Update { name: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name, template } => {
            use dialoguer::{Input, Select};
            use crate::templates::get_available_templates;

            let final_name = match name {
                Some(n) => n,
                None => Input::<String>::new()
                    .with_prompt("Project name")
                    .default("my-ionyx-app".into())
                    .interact_text()?,
            };

            let final_template = match template {
                Some(t) => t,
                None => {
                    let templates = get_available_templates();
                    let selection = Select::new()
                        .with_prompt("Choose your frontend framework")
                        .items(&templates)
                        .default(0)
                        .interact()?;
                    templates[selection].to_string()
                }
            };

            commands::create::execute(final_name, &final_template).await?;
        }
        Commands::Dev { port, hot } => {
            commands::dev::execute(port, hot).await?;
        }
        Commands::Build { target, minify } => {
            commands::build::execute(&target, minify).await?;
        }
        Commands::Run { port } => {
            commands::run::execute(port).await?;
        }
        Commands::Config { action } => {
            commands::config::execute(action).await?;
        }
        Commands::Plugin { action } => {
            commands::plugin::execute(action).await?;
        }
    }

    Ok(())
}
