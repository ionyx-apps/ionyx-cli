use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{info, error, debug};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcBridgeRequest {
    pub source_app: String,
    pub target_app: String,
    pub command: String,
    pub payload: Value,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcBridgeResponse {
    pub id: String,
    pub success: bool,
    pub data: Option<Value>,
    pub error: Option<String>,
    pub source_app: String,
    pub target_app: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub pid: u32,
    pub capabilities: Vec<String>,
}

pub struct IpcBridgeManager {
    apps: Arc<RwLock<HashMap<String, AppInfo>>>,
    config: Config,
    current_app_id: String,
}

impl IpcBridgeManager {
    pub fn new(config: Config, app_name: String) -> Self {
        let apps = Arc::new(RwLock::new(HashMap::new()));
        
        Self {
            apps,
            config,
            current_app_id: app_name,
        }
    }

    pub async fn register_app(&self, app_info: AppInfo) -> Result<()> {
        info!("Registering app: {} (PID: {})", app_info.name, app_info.pid);
        
        let mut apps = self.apps.write().await;
        apps.insert(app_info.name.clone(), app_info);
        
        info!("App registered successfully. Total apps: {}", apps.len());
        Ok(())
    }

    pub async fn unregister_app(&self, app_name: &str) -> Result<()> {
        info!("Unregistering app: {}", app_name);
        
        let mut apps = self.apps.write().await;
        apps.remove(app_name);
        
        info!("App unregistered successfully. Total apps: {}", apps.len());
        Ok(())
    }

    pub async fn get_registered_apps(&self) -> Result<Vec<AppInfo>> {
        let apps = self.apps.read().await;
        Ok(apps.values().cloned().collect())
    }

    pub async fn send_to_app(&self, request: IpcBridgeRequest) -> Result<IpcBridgeResponse> {
        info!("Sending IPC request from {} to {}: {}", 
              request.source_app, request.target_app, request.command);

        let source_app = request.source_app.clone();
        let target_app = request.target_app.clone();

        // Check if target app exists
        let apps = self.apps.read().await;
        if !apps.contains_key(&target_app) {
            return Err(anyhow::anyhow!("Target app '{}' not found", target_app));
        }

        // Check if source app has permission to send to target app
        if !self.check_permissions(&source_app, &target_app, &request.command).await? {
            return Err(anyhow::anyhow!("Permission denied for command '{}' from '{}' to '{}'", 
                request.command, source_app, target_app));
        }

        // Process the request
        let response = match self.process_bridge_request(&request).await {
            Ok(data) => IpcBridgeResponse {
                id: request.id,
                success: true,
                data: Some(data),
                error: None,
                source_app,
                target_app,
            },
            Err(e) => IpcBridgeResponse {
                id: request.id,
                success: false,
                data: None,
                error: Some(e.to_string()),
                source_app,
                target_app,
            },
        };

        info!("IPC request processed: {} -> {} ({})", 
              response.source_app, response.target_app, response.success);
        
        Ok(response)
    }

    async fn check_permissions(&self, source_app: &str, target_app: &str, command: &str) -> Result<bool> {
        // For now, allow all inter-app communication
        // In a real implementation, this would check permissions from config
        debug!("Checking permissions: {} -> {} ({})", source_app, target_app, command);
        Ok(true)
    }

    async fn process_bridge_request(&self, request: &IpcBridgeRequest) -> Result<Value> {
        match request.command.as_str() {
            "app.getInfo" => {
                let apps = self.apps.read().await;
                if let Some(app_info) = apps.get(&request.target_app) {
                    Ok(json!({
                        "name": app_info.name,
                        "version": app_info.version,
                        "pid": app_info.pid,
                        "capabilities": app_info.capabilities
                    }))
                } else {
                    Err(anyhow::anyhow!("App '{}' not found", request.target_app))
                }
            }
            "app.list" => {
                let apps = self.apps.read().await;
                let app_list: Vec<Value> = apps.values().map(|app| {
                    json!({
                        "name": app.name,
                        "version": app.version,
                        "pid": app.pid,
                        "capabilities": app.capabilities
                    })
                }).collect();
                Ok(json!(app_list))
            }
            "system.broadcast" => {
                // Broadcast message to all registered apps
                info!("Broadcasting message from {}: {:?}", request.source_app, request.payload);
                Ok(json!({
                    "broadcast_sent": true,
                    "message": "Message broadcasted to all apps",
                    "recipients": self.apps.read().await.len()
                }))
            }
            _ => Err(anyhow::anyhow!("Unknown bridge command: {}", request.command))
        }
    }

    pub async fn handle_bridge_request(&self, request: String) -> Result<IpcBridgeResponse> {
        let bridge_request: IpcBridgeRequest = serde_json::from_str(&request)
            .map_err(|e| anyhow::anyhow!("Failed to parse bridge request: {}", e))?;

        self.send_to_app(bridge_request).await
    }
}

// IPC Bridge Commands for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeCommand {
    RegisterApp { name: String, version: String, capabilities: Vec<String> },
    UnregisterApp { name: String },
    GetApps,
    SendToApp { target_app: String, command: String, payload: Value },
    Broadcast { message: Value },
}

impl BridgeCommand {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|e| anyhow::anyhow!("Failed to serialize bridge command: {}", e))
    }
}
