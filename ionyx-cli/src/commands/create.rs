use anyhow::Result;
use colored::*;
use crate::templates;
use std::fs;
use std::path::Path;

pub async fn execute(name: String, template: &str) -> Result<()> {
    println!("🚀 Creating new Ionyx project: {}", name.green());
    
    // Check if directory already exists
    if Path::new(&name).exists() {
        return Err(anyhow::anyhow!("Directory '{}' already exists", name));
    }
    
    // Create project directory
    fs::create_dir_all(&name)?;
    println!("✅ Created directory: {}", name);
    
    // Show template info
    println!("📋 Template: {}", template.cyan());
    
    // Copy template files
    templates::copy_template(&name, template).await?;
    
    // Show next steps
    println!("\n✅ Project '{}' created successfully!", name.green());
    println!("\n📋 Next steps:");
    println!("   cd {}", name);
    println!("   npm run ionyx:dev");
    
    // Show framework-specific info
    match template {
        "react" => println!("\n🎯 React + TypeScript + Vite setup ready!"),
        "svelte" => println!("\n🎯 Svelte + TypeScript + Vite setup ready!"),
        "vue" => println!("\n🎯 Vue + TypeScript + Vite setup ready!"),
        "leptos" => println!("\n🎯 Rust + Leptos (WASM) setup ready!"),
        "angular" => println!("\n🎯 Angular + TypeScript + Vite setup ready!"),
        "vanilla" => println!("\n🎯 Vanilla JavaScript + Vite setup ready!"),
        _ => println!("\n🎯 Basic Ionyx setup ready!"),
    }
    
    println!("\n🚀 Available commands:");
    println!("   npm run ionyx:dev    # Start Ionyx application");
    println!("   npm run ionyx:build   # Build for production");
    println!("   npm run dev           # Frontend only");
    println!("   npm run build         # Frontend build only");
    
    println!("\n🎉 Happy coding with Ionyx Framework!");
    
    Ok(())
}
