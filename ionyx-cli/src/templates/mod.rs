use anyhow::Result;
use std::fs;
use std::path::Path;
use include_dir::{include_dir, Dir};

// Embed templates in binary
static TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/src/templates");

pub async fn copy_template(project_name: &str, template: &str) -> Result<()> {
    let template_dir = TEMPLATES.get_dir(template)
        .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", template))?;
    
    // Copy template files to project directory
    copy_embedded_dir(template_dir, project_name)?;
    
    // Update package.json with project name
    let package_json_path = format!("{}/package.json", project_name);
    if Path::new(&package_json_path).exists() {
        let mut content = fs::read_to_string(&package_json_path)?;
        // Replace the placeholder with actual project name
        content = content.replace("my-ionyx-app", project_name);
        fs::write(&package_json_path, content)?;
    }
    
    // Update ionyx.config.json with project name
    let config_json_path = format!("{}/ionyx.config.json", project_name);
    if Path::new(&config_json_path).exists() {
        let mut content = fs::read_to_string(&config_json_path)?;
        content = content.replace("{{PROJECT_NAME}}", project_name);
        fs::write(&config_json_path, content)?;
    }
    
    // Update README.md with project name
    let readme_path = format!("{}/README.md", project_name);
    if Path::new(&readme_path).exists() {
        let mut content = fs::read_to_string(&readme_path)?;
        content = content.replace("my-ionyx-app", project_name);
        fs::write(&readme_path, content)?;
    }
    
    // Update index.html title
    let index_html_path = format!("{}/index.html", project_name);
    if Path::new(&index_html_path).exists() {
        let mut content = fs::read_to_string(&index_html_path)?;
        content = content.replace("my-ionyx-app", project_name);
        fs::write(&index_html_path, content)?;
    }
    
    // Update src-ionyx Cargo.toml
    let backend_cargo_path = format!("{}/src-ionyx/Cargo.toml", project_name);
    if Path::new(&backend_cargo_path).exists() {
        let mut content = fs::read_to_string(&backend_cargo_path)?;
        content = content.replace("my-ionyx-app", project_name);
        fs::write(&backend_cargo_path, content)?;
    }
    
    // Update src-ionyx main.rs app name
    let backend_main_path = format!("{}/src-ionyx/src/main.rs", project_name);
    if Path::new(&backend_main_path).exists() {
        let mut content = fs::read_to_string(&backend_main_path)?;
        content = content.replace("my-ionyx-app", project_name);
        fs::write(&backend_main_path, content)?;
    }
    
    // Update src-ionyx Cargo.toml workspace
    let workspace_cargo_path = format!("{}/src-ionyx/Cargo.toml", project_name);
    if Path::new(&workspace_cargo_path).exists() {
        let mut content = fs::read_to_string(&workspace_cargo_path)?;
        content = content.replace("my-ionyx-app", project_name);
        fs::write(&workspace_cargo_path, content)?;
    }
    
    Ok(())
}

fn copy_embedded_dir(dir: &Dir<'static>, dst: &str) -> Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in dir.entries() {
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let dst_path = format!("{}/{}", dst, name);
        
        if let Some(subdir) = entry.as_dir() {
            copy_embedded_dir(subdir, &dst_path)?;
        } else {
            let file = entry.as_file().unwrap();
            let contents = file.contents();
            fs::write(&dst_path, contents)?;
        }
    }
    
    Ok(())
}

// Get available templates
pub fn get_available_templates() -> Vec<&'static str> {
    TEMPLATES.get_dir("basic").map(|_| vec!["basic"]).unwrap_or_default()
        .into_iter()
        .chain(TEMPLATES.get_dir("react").map(|_| vec!["react"]).unwrap_or_default())
        .chain(TEMPLATES.get_dir("svelte").map(|_| vec!["svelte"]).unwrap_or_default())
        .chain(TEMPLATES.get_dir("vue").map(|_| vec!["vue"]).unwrap_or_default())
        .chain(TEMPLATES.get_dir("vanilla").map(|_| vec!["vanilla"]).unwrap_or_default())
        .chain(TEMPLATES.get_dir("leptos").map(|_| vec!["leptos"]).unwrap_or_default())
        .chain(TEMPLATES.get_dir("angular").map(|_| vec!["angular"]).unwrap_or_default())
        .collect()
}
