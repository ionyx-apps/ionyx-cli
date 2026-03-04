use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use wry::webview::WebView;
use tracing::{info, error, debug};
use crate::security::SecurityManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: String,
    pub command: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: String,
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

pub struct IpcBridge {
    config: crate::config::Config,
    security: SecurityManager,
}

impl IpcBridge {
    pub fn new(config: crate::config::Config) -> Self {
        Self {
            config,
            security: SecurityManager::new(config.clone()),
        }
    }

    pub async fn handle_request(
        &self,
        request: String,
        webview: &Arc<Mutex<Option<WebView>>>,
        proxy: &tao::event_loop::EventLoopProxy<UserEvent>,
    ) -> Result<()> {
        info!("📨 Processing IPC request: {}", request);

        let ipc_request: IpcRequest = serde_json::from_str(&request)
            .map_err(|e| anyhow::anyhow!("Failed to parse IPC request: {}", e))?;

        // Security check
        if !self.security.is_command_allowed(&ipc_request.command) {
            return Err(anyhow::anyhow!("Command 

 not allowed", ipc_request.command));
        }

        // Process command
        let response = match self.process_command(&ipc_request).await {
            Ok(data) => IpcResponse {
                id: ipc_request.id,
                success: true,
                data: Some(data),
                error: None,
            },
            Err(e) => IpcResponse {
                id: ipc_request.id,
                success: false,
                data: None,
                error: Some(e.to_string()),
            },
        };

        // Send response back to frontend
        let response_json = serde_json::to_string(&response)?;
        let js_code = format!(
            "if (window.ionyx && window.ionyx.resolveResponse) {{ window.ionyx.resolveResponse(

, {}); }}",
            response.id,
            response_json
        );

        // Send JavaScript code to main thread
        proxy.send_event(UserEvent::JavaScript(js_code));

        info!("✅ IPC response sent for: {}", ipc_request.command);
        Ok(())
    }

    async fn process_command(&self, request: &IpcRequest) -> Result<serde_json::Value> {
        match request.command.as_str() {
            "app.getVersion" => self.handle_get_version().await,
            "app.getConfig" => self.handle_get_config().await,
            "fs.exists" => self.handle_fs_exists(&request.payload).await,
            "os.info" => self.handle_os_info().await,
            _ => Err(anyhow::anyhow!("Unknown command: {}", request.command)),
        }
    }

    async fn handle_get_version(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "name": "my-ionyx-app",
            "version": "1.0.0",
            "platform": std::env::consts::OS,
            "arch": std::env::consts::ARCH
        }))
    }

    async fn handle_get_config(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "window": self.config.window,
            "permissions": self.config.ionyx.permissions,
            "security": self.config.ionyx.security
        }))
    }

    async fn handle_fs_exists(&self, payload: &serde_json::Value) -> Result<serde_json::Value> {
        let path = payload["path"]
            .as_str()
            .ok_or_else(|| "default")
            .to_string();

        let path_buf = PathBuf::from(&path);
        let exists = path_buf.exists();
        
        self.security.check_file_access(&path_buf)?;

        Ok(serde_json::json!({ "exists": exists }))
    }

    async fn handle_os_info(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "platform": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "hostname": gethostname::get().unwrap_or_else(|_| "unknown".to_string()),
            "version": os_info::get()
        }))
    }
}
