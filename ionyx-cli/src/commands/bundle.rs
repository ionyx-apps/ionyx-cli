use anyhow::Result;
use colored::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use std::io::{Read, Write};
use zip::write::FileOptions;

pub async fn execute(platform: Option<String>, format: Option<String>) -> Result<()> {
    let target_platform = platform.unwrap_or_else(|| env::consts::OS.to_string());
    let target_format = format.unwrap_or_else(|| {
        if target_platform == "windows" {
            "nsis".to_string()
        } else if target_platform == "macos" {
            "dmg".to_string()
        } else {
            "appimage".to_string()
        }
    });

    println!("📦 Bundling Ionyx project for {}: {}", target_platform.cyan(), target_format.green());

    let (app_name, app_version) = if Path::new("package.json").exists() {
        let content = fs::read_to_string("package.json")?;
        let pkg: serde_json::Value = serde_json::from_str(&content)?;
        let name = pkg["name"].as_str().unwrap_or("ionyx-app").to_string();
        let version = pkg["version"].as_str().unwrap_or("1.0.0").to_string();
        (name, version)
    } else {
        ("ionyx-app".to_string(), "1.0.0".to_string())
    };

    let icon_path = if Path::new("ionyx.config.json").exists() {
        let content = fs::read_to_string("ionyx.config.json")?;
        let cfg: serde_json::Value = serde_json::from_str(&content)?;
        cfg["window"]["icon"].as_str().map(|s| s.to_string())
    } else {
        None
    };
    let icon_path_str = icon_path.unwrap_or_else(|| "icon.png".to_string());

    // 1. Build the project first
    println!("🔨 Running full build...");
    crate::commands::build::execute("all", true).await?;

    // 2. Prepare installer directory
    let installer_dir = Path::new("installers");
    if installer_dir.exists() {
        fs::remove_dir_all(installer_dir)?;
    }
    fs::create_dir_all(installer_dir)?;

    // 3. Find the executable
    let mut exe_path = PathBuf::from("src-ionyx/target/release");
    if target_platform == "windows" {
        exe_path.push("ionyx-host.exe");
    } else {
        exe_path.push("ionyx-host");
    }

    if !exe_path.exists() {
        return Err(anyhow::anyhow!("Binary not found at {:?}. Did the build fail?", exe_path));
    }

    // 4. Platform-specific bundling
    match target_platform.as_str() {
        "windows" => {
            if target_format == "nsis" {
                bundle_windows_nsis(installer_dir, &exe_path, &app_name, &app_version, &icon_path_str).await?;
            } else if target_format == "wix" {
                bundle_windows_wix(installer_dir, &exe_path, &app_name, &app_version, &icon_path_str).await?;
            } else {
                return Err(anyhow::anyhow!("Unsupported format for windows: {}", target_format));
            }
        }
        "macos" => {
            bundle_macos_dmg(installer_dir, &exe_path, &app_name, &app_version, &icon_path_str).await?;
        }
        "linux" => {
            bundle_linux_appimage(installer_dir, &exe_path, &app_name, &app_version, &icon_path_str).await?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported platform: {}", target_platform)),
    }

    println!("\n🎉 Bundling completed successfully!");
    println!("📁 Output directory: {}", "installers/".cyan());

    Ok(())
}

async fn bundle_windows_nsis(installer_dir: &Path, exe_path: &Path, app_name: &str, app_version: &str, icon_path: &str) -> Result<()> {
    println!("🪟 Building Windows EXE installer via NSIS...");
    
    let icon_absolute = if Path::new(icon_path).exists() {
        Some(fs::canonicalize(icon_path)?)
    } else {
        None
    };

    let nsis_script = format!(
        r#"!define APP_NAME "{}"
!define APP_VERSION "{}"
!define APP_PUBLISHER "Ionyx Framework Team"
!define APP_EXE "ionyx-host.exe"
{}

Name "${{APP_NAME}}"
OutFile "{}\IonyxApp-Setup.exe"
InstallDir "$PROGRAMFILES64\${{APP_NAME}}"
RequestExecutionLevel admin

Page directory
Page instfiles

Section "MainSection" SEC01
    SetOutPath "$INSTDIR"
    File "{}"
    CreateShortCut "$DESKTOP\${{APP_NAME}}.lnk" "$INSTDIR\${{APP_EXE}}" "" "{}" 0
SectionEnd
"#,
        app_name,
        app_version,
        if let Some(ref icon) = icon_absolute {
            format!("!define APP_ICON \"{}\"", icon.display())
        } else {
            "".to_string()
        },
        installer_dir.display(),
        exe_path.canonicalize()?.display(),
        icon_absolute.map(|p| p.display().to_string()).unwrap_or_default()
    );

    let script_path = installer_dir.join("installer.nsi");
    fs::write(&script_path, nsis_script)?;

    let status = Command::new("makensis")
        .arg(&script_path)
        .status()
        .await;

    match status {
        Ok(s) if s.success() => println!("✅ NSIS build successful"),
        _ => {
            println!("⚠️  NSIS (makensis) not found or failed. Falling back to structured ZIP.");
            bundle_zip_fallback(installer_dir, exe_path, app_name, app_version).await?;
        }
    }

    Ok(())
}

async fn bundle_windows_wix(installer_dir: &Path, exe_path: &Path, app_name: &str, _app_version: &str, _icon_path: &str) -> Result<()> {
    println!("🪟 Building Windows MSI installer via WiX...");
    // Placeholder for WiX implementation
    Ok(())
}

async fn bundle_macos_dmg(installer_dir: &Path, exe_path: &Path, app_name: &str, app_version: &str, icon_path: &str) -> Result<()> {
    println!("🍎 Building macOS DMG installer...");
    
    let dmg_script = format!(
        r#"#!/bin/bash
APP_NAME="{}"
TEMP_DIR="/tmp/${{APP_NAME}}"
APP_DIR="${{TEMP_DIR}}/${{APP_NAME}}.app"

mkdir -p "${{APP_DIR}}/Contents/MacOS"
mkdir -p "${{APP_DIR}}/Contents/Resources"

cp "{}" "${{APP_DIR}}/Contents/MacOS/ionyx-host"
{}

cat > "${{APP_DIR}}/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>ionyx-host</string>
    <key>CFBundleIdentifier</key>
    <string>com.ionyx-framework.app</string>
    <key>CFBundleName</key>
    <string>${{APP_NAME}}</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    {}
</dict>
</plist>
EOF

hdiutil create -volname "${{APP_NAME}}" -srcfolder "${{TEMP_DIR}}" -ov -format UDZO "{}/{}.dmg"
rm -rf "${{TEMP_DIR}}"
"#,
        app_name,
        exe_path.canonicalize()?.display(),
        if Path::new(icon_path).exists() {
            format!("cp \"{}\" \"${{APP_DIR}}/Contents/Resources/icon.png\"", icon_path)
        } else {
            "".to_string()
        },
        if Path::new(icon_path).exists() {
            "<key>CFBundleIconFile</key><string>icon.png</string>".to_string()
        } else {
            "".to_string()
        },
        installer_dir.display(),
        app_name
    );

    let script_path = installer_dir.join("build-dmg.sh");
    fs::write(&script_path, dmg_script)?;

    let status = Command::new("bash")
        .arg(&script_path)
        .status()
        .await;

    match status {
        Ok(s) if s.success() => println!("✅ DMG build successful"),
        _ => {
            println!("⚠️  macOS bundling tools not available or failed. Falling back to structured ZIP.");
            bundle_zip_fallback(installer_dir, exe_path, app_name, app_version).await?;
        }
    }

    Ok(())
}

async fn bundle_linux_appimage(installer_dir: &Path, exe_path: &Path, app_name: &str, app_version: &str, icon_path: &str) -> Result<()> {
    println!("🐧 Building Linux AppImage...");
    
    let appimage_script = format!(
        r#"#!/bin/bash
APP_NAME="{}"
APP_DIR="{}/${{APP_NAME}}.AppImage"
TEMP_DIR="{}/temp"

mkdir -p "${{TEMP_DIR}}/usr/bin"
mkdir -p "${{TEMP_DIR}}/usr/share/icons/hicolor/256x256/apps"

cp "{}" "${{TEMP_DIR}}/usr/bin/ionyx-host"
chmod +x "${{TEMP_DIR}}/usr/bin/ionyx-host"

{}

cat > "${{TEMP_DIR}}/usr/share/applications/${{APP_NAME}}.desktop" << EOF
[Desktop Entry]
Name=${{APP_NAME}}
Exec=ionyx-host
Icon=icon
Type=Application
Categories=Utility;
EOF

cat > "${{TEMP_DIR}}/AppRun" << EOF
#!/bin/bash
HERE="\$(dirname "\$0")"
exec "\$HERE/usr/bin/ionyx-host"
EOF
chmod +x "${{TEMP_DIR}}/AppRun"

# Fallback to simple tar if appimagetool not found
if command -v appimagetool >/dev/null; then
    appimagetool "${{TEMP_DIR}}" "${{APP_DIR}}"
else
    tar -czf "{}/{}.tar.gz" -C "{}" temp
fi
rm -rf "${{TEMP_DIR}}"
"#,
        app_name,
        installer_dir.display(),
        installer_dir.display(),
        exe_path.canonicalize()?.display(),
        if Path::new(icon_path).exists() {
            format!("cp \"{}\" \"${{TEMP_DIR}}/usr/share/icons/hicolor/256x256/apps/icon.png\"", icon_path)
        } else {
            "".to_string()
        },
        installer_dir.display(),
        app_name,
        installer_dir.display()
    );

    let script_path = installer_dir.join("build-appimage.sh");
    fs::write(&script_path, appimage_script)?;

    let status = Command::new("bash")
        .arg(&script_path)
        .status()
        .await;

    match status {
        Ok(s) if s.success() => println!("✅ Linux bundling complete"),
        _ => {
            println!("⚠️  Linux bundling tools not available or failed. Falling back to structured ZIP.");
            bundle_zip_fallback(installer_dir, exe_path, app_name, app_version).await?;
        }
    }

    Ok(())
}

async fn bundle_zip_fallback(installer_dir: &Path, exe_path: &Path, app_name: &str, app_version: &str) -> Result<()> {
    println!("📦 Creating a professional portable ZIP distribution...");
    let zip_name = format!("{}-{}-portable.zip", app_name, app_version);
    let zip_path = installer_dir.join(zip_name);
    
    let file = fs::File::create(&zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // 1. Add Main Executable
    let exe_name = if cfg!(target_os = "windows") {
        format!("{}.exe", app_name)
    } else {
        app_name.to_string()
    };
    
    zip.start_file(format!("{}/{}", app_name, exe_name), options)?;
    let mut exe_file = fs::File::open(exe_path)?;
    let mut buffer = Vec::new();
    exe_file.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;

    // 2. Add Icon if exists
    if Path::new("icon.png").exists() {
        zip.start_file(format!("{}/icon.png", app_name), options)?;
        let mut icon_file = fs::File::open("icon.png")?;
        let mut icon_buffer = Vec::new();
        icon_file.read_to_end(&mut icon_buffer)?;
        zip.write_all(&icon_buffer)?;
    }

    // 3. Add README.md
    zip.start_file(format!("{}/README.txt", app_name), options)?;
    let readme_content = format!(
        "{} v{}\n\nPortable distribution created by Ionyx Framework 🚀\n\nTo run the application, execute: {}\n",
        app_name, app_version, exe_name
    );
    zip.write_all(readme_content.as_bytes())?;

    zip.finish()?;
    
    println!("✅ Structured portable ZIP created: {}", zip_path.display());
    Ok(())
}
