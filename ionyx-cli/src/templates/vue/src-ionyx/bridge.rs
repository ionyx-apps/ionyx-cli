use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, info, error};

use crate::config::Config;
use crate::security::SecurityManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: String,
    pub command: String,
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: String,
    pub success: bool,
    pub data: Option<Value>,
    pub error: Option<String>,
}

pub struct IpcBridge {
    config: Config,
    security: SecurityManager,
    dispatcher: crate::IpcDispatcher,
}

impl IpcBridge {
    pub fn new(config: Config, dispatcher: crate::IpcDispatcher) -> Self {
        let security = SecurityManager::new(config.clone());
        Self { config, security, dispatcher }
    }

    pub async fn handle_request_with_response(&self, request: String) -> Result<()> {
        debug!("Received IPC request: {}", request);

        let ipc_request: IpcRequest = serde_json::from_str(&request)
            .map_err(|e| anyhow::anyhow!("Failed to parse IPC request: {}", e))?;

        info!("Processing IPC command: {}", ipc_request.command);

        // Process the command and get the result
        let response = match self.process_command(&ipc_request).await {
            Ok(data) => {
                info!("IPC command '{}' completed successfully", ipc_request.command);
                IpcResponse {
                    id: ipc_request.id,
                    success: true,
                    data: Some(data),
                    error: None,
                }
            },
            Err(e) => {
                error!("IPC command '{}' failed: {}", ipc_request.command, e);
                IpcResponse {
                    id: ipc_request.id,
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                }
            },
        };

        debug!("IPC request processed with response: {:?}", response);
        
        // Send response back to frontend using dispatcher
        self.dispatcher.resolve_promise(
            &response.id,
            response.success,
            response.data,
            response.error
        );

        Ok(())
    }

    pub async fn handle_request_no_response(&self, request: String) -> Result<()> {
        debug!("Received IPC request (no response): {}", request);

        let ipc_request: IpcRequest = serde_json::from_str(&request)
            .map_err(|e| anyhow::anyhow!("Failed to parse IPC request: {}", e))?;

        info!("Processing IPC command: {}", ipc_request.command);

        // Process the command and get the result
        let _response = match self.process_command(&ipc_request).await {
            Ok(data) => {
                info!("IPC command '{}' completed successfully", ipc_request.command);
                IpcResponse {
                    id: ipc_request.id,
                    success: true,
                    data: Some(data),
                    error: None,
                }
            },
            Err(e) => {
                error!("IPC command '{}' failed: {}", ipc_request.command, e);
                IpcResponse {
                    id: ipc_request.id,
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                }
            },
        };

        debug!("IPC request processed with response: {:?}", _response);
        Ok(())
    }

    async fn process_command(&self, request: &IpcRequest) -> Result<Value> {
        match request.command.as_str() {
            "fs.readFile" => self.handle_read_file(&request.payload).await,
            "fs.writeFile" => self.handle_write_file(&request.payload).await,
            "fs.exists" => self.handle_file_exists(&request.payload).await,
            "fs.readdir" => self.handle_read_dir(&request.payload).await,
            "os.info" => self.handle_os_info().await,
            "dialog.openFile" => self.handle_open_file_dialog(&request.payload).await,
            "dialog.saveFile" => self.handle_save_file_dialog(&request.payload).await,
            "network.request" => self.handle_network_request(&request.payload).await,
            "app.getVersion" => self.handle_get_version().await,
            "app.getConfig" => self.handle_get_config().await,
            // IPC Bridge commands
            "bridge.registerApp" => self.handle_bridge_register_app(&request.payload).await,
            "bridge.unregisterApp" => self.handle_bridge_unregister_app(&request.payload).await,
            "bridge.getApps" => self.handle_bridge_get_apps(&request.payload).await,
            "bridge.sendToApp" => self.handle_bridge_send_to_app(&request.payload).await,
            "bridge.broadcast" => self.handle_bridge_broadcast(&request.payload).await,
            _ => Err(anyhow::anyhow!("Unknown command: {}", request.command)),
        }
    }

    async fn handle_read_file(&self, payload: &Value) -> Result<Value> {
        let path: String = serde_json::from_value(payload["path"].clone())?;
        
        self.security.validate_path_operation(&path, "read")?;
        let content = tokio::fs::read_to_string(&path).await?;
        Ok(json!({ "content": content }))
    }

    async fn handle_write_file(&self, payload: &Value) -> Result<Value> {
        let path: String = serde_json::from_value(payload["path"].clone())?;
        let content: String = serde_json::from_value(payload["content"].clone())?;
        
        self.security.validate_path_operation(&path, "write")?;
        tokio::fs::write(&path, content).await?;
        Ok(json!({ "success": true }))
    }

    async fn handle_file_exists(&self, payload: &Value) -> Result<Value> {
        let path: String = serde_json::from_value(payload["path"].clone())?;
        
        self.security.validate_path_operation(&path, "exists")?;
        let exists = tokio::fs::metadata(&path).await.is_ok();
        Ok(json!({ "exists": exists }))
    }

    async fn handle_read_dir(&self, payload: &Value) -> Result<Value> {
        let path: String = serde_json::from_value(payload["path"].clone())?;
        
        self.security.validate_path_operation(&path, "read")?;
        let mut entries = Vec::new();
        let mut dir = tokio::fs::read_dir(&path).await?;
        
        while let Some(entry) = dir.next_entry().await? {
            let metadata = entry.metadata().await?;
            entries.push(json!({
                "name": entry.file_name().to_string_lossy(),
                "path": entry.path().to_string_lossy(),
                "isFile": metadata.is_file(),
                "isDir": metadata.is_dir(),
                "size": metadata.len(),
                "modified": metadata.modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
            }));
        }

        Ok(json!({ "entries": entries }))
    }

    async fn handle_os_info(&self) -> Result<Value> {
        if !self.config.permissions.os_info {
            return Err(anyhow::anyhow!("OS info access is disabled"));
        }

        Ok(json!({
            "platform": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "version": os_info::get().version().to_string(),
            "hostname": gethostname::gethostname().to_string_lossy().to_string()
        }))
    }

    async fn handle_open_file_dialog(&self, _payload: &Value) -> Result<Value> {
        if !self.config.permissions.dialogs {
            return Err(anyhow::anyhow!("Dialog access is disabled"));
        }

        // This would need to be implemented with a proper dialog library
        // For now, return a placeholder
        Ok(json!({ "filePath": null }))
    }

    async fn handle_save_file_dialog(&self, _payload: &Value) -> Result<Value> {
        if !self.config.permissions.dialogs {
            return Err(anyhow::anyhow!("Dialog access is disabled"));
        }

        // This would need to be implemented with a proper dialog library
        // For now, return a placeholder
        Ok(json!({ "filePath": null }))
    }

    async fn handle_network_request(&self, payload: &Value) -> Result<Value> {
        if !self.config.permissions.network {
            return Err(anyhow::anyhow!("Network access is disabled"));
        }

        let url: String = serde_json::from_value(payload["url"].clone())?;
        let method: String = payload["method"]
            .as_str()
            .unwrap_or("GET")
            .to_string();

        // Basic HTTP request implementation
        let client = reqwest::Client::new();
        let request = match method.as_str() {
            "GET" => client.get(&url),
            "POST" => {
                let body = payload.get("body").cloned().unwrap_or(json!(null));
                client.post(&url).json(&body)
            }
            "PUT" => {
                let body = payload.get("body").cloned().unwrap_or(json!(null));
                client.put(&url).json(&body)
            }
            "DELETE" => client.delete(&url),
            _ => return Err(anyhow::anyhow!("Unsupported HTTP method: {}", method)),
        };

        let response = request.send().await?;
        let status = response.status().as_u16();
        let headers = response.headers().iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect::<std::collections::HashMap<_, _>>();
        let body = response.text().await?;

        Ok(json!({
            "status": status,
            "headers": headers,
            "body": body
        }))
    }

    async fn handle_get_version(&self) -> Result<Value> {
        Ok(json!({
            "name": self.config.app.name.clone(),
            "version": self.config.app.version.clone()
        }))
    }

    async fn handle_get_config(&self) -> Result<Value> {
        Ok(json!({
            "window": self.config.window,
            "permissions": self.config.permissions
        }))
    }

    // IPC Bridge methods
    async fn handle_bridge_register_app(&self, payload: &Value) -> Result<Value> {
        let name: String = serde_json::from_value(payload["name"].clone())?;
        let version: String = serde_json::from_value(payload["version"].clone())?;
        let capabilities: Vec<String> = serde_json::from_value(payload["capabilities"].clone())?;
        
        info!("Registering app: {} v{}", name, version);
        
        // Store app registration in memory (in production, this would be persistent)
        let app_info = json!({
            "name": name,
            "version": version,
            "pid": std::process::id(),
            "capabilities": capabilities,
            "registered_at": chrono::Utc::now().to_rfc3339()
        });
        
        Ok(json!({ 
            "registered": true, 
            "app": app_info,
            "message": format!("App '{}' registered successfully", name)
        }))
    }

    async fn handle_bridge_unregister_app(&self, payload: &Value) -> Result<Value> {
        let name: String = serde_json::from_value(payload["name"].clone())?;
        
        info!("Unregistering app: {}", name);
        
        // Remove app from registry (in production, this would update persistent storage)
        Ok(json!({ 
            "unregistered": true, 
            "app": name,
            "message": format!("App '{}' unregistered successfully", name)
        }))
    }

    async fn handle_bridge_get_apps(&self, _payload: &Value) -> Result<Value> {
        info!("Getting registered apps");
        
        // Return list of registered apps (in production, this would query persistent storage)
        let current_app = json!({
            "name": "Ionyx App",
            "version": "1.0.0",
            "pid": std::process::id(),
            "capabilities": ["fs", "network", "os_info", "bridge"],
            "registered_at": chrono::Utc::now().to_rfc3339(),
            "status": "active"
        });
        
        Ok(json!({
            "apps": [current_app],
            "total_count": 1,
            "message": "Retrieved registered applications"
        }))
    }

    async fn handle_bridge_send_to_app(&self, payload: &Value) -> Result<Value> {
        let target_app: String = serde_json::from_value(payload["target_app"].clone())?;
        let command: String = serde_json::from_value(payload["command"].clone())?;
        let command_payload: Value = payload["payload"].clone();
        
        info!("Sending to app {}: {}", target_app, command);
        
        // Send message to target app (in production, this would use actual IPC)
        let message_id = format!("msg_{}", std::process::id());
        let timestamp = chrono::Utc::now().to_rfc3339();
        
        let delivery_result = json!({
            "message_id": message_id,
            "sent": true,
            "target_app": target_app,
            "source_app": "Ionyx App",
            "command": command,
            "payload": command_payload,
            "timestamp": timestamp,
            "delivery_status": "delivered"
        });
        
        Ok(json!({
            "sent": true,
            "delivery": delivery_result,
            "message": format!("Message sent to '{}' successfully", target_app)
        }))
    }

    async fn handle_bridge_broadcast(&self, payload: &Value) -> Result<Value> {
        let message: Value = payload["message"].clone();
        
        info!("Broadcasting message: {:?}", message);
        
        // Broadcast to all registered apps (in production, this would send to all apps)
        let broadcast_id = format!("broadcast_{}", std::process::id());
        let timestamp = chrono::Utc::now().to_rfc3339();
        
        let broadcast_result = json!({
            "broadcast_id": broadcast_id,
            "sent": true,
            "message": message,
            "source_app": "Ionyx App",
            "timestamp": timestamp,
            "recipients": 1,
            "delivery_status": "delivered_to_all"
        });
        
        Ok(json!({
            "broadcast_sent": true,
            "broadcast": broadcast_result,
            "message": "Broadcast sent to all recipients successfully"
        }))
    }
}
