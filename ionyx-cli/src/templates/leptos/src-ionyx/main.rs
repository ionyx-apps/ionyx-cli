//! my-ionyx-app backend - Ionyx Framework

use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use tao::event_loop::{EventLoop, EventLoopProxy};
use wry::application::Application;
use wry::webview::WebView;
use wry::webview::WebViewBuilder;
use tracing::{info, error, debug};
use tracing_subscriber;

mod bridge;
mod config;
mod security;

use crate::bridge::IpcBridge;

#[derive(Debug, Clone)]
pub enum UserEvent {
    JavaScript(String),
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🚀 Starting Ionyx Framework Application");

    // Load configuration
    let config = config::Config::load()?;
    info!("📋 Configuration loaded: {:?}", config);

    // Create event loop
    let event_loop = EventLoop::new()?;
    let proxy = event_loop.create_proxy();

    // Wrap webview for thread-safe access
    let webview = Arc::new(Mutex::new(None::<WebView>));

    // Create IPC bridge
    let ipc_bridge = IpcBridge::new(config.clone());

    // Create application
    let mut app = Application::new()
        .with_window(|window| {
            let window_builder = window
                .with_title(&config.window.title)
                .with_inner_size(config.window.width, config.window.height)
                .with_resizable(config.window.resizable)
                .with_fullscreen(config.window.fullscreen);

            let webview_builder = WebViewBuilder::new()
                .with_url("http://localhost:5173")
                .with_initialization_script(include_str!("../frontend/src/ionyx.js"))
                .with_ipc_handler(move |request| {
                    let bridge = ipc_bridge.clone();
                    let webview = webview.clone();
                    let proxy = proxy.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = bridge.handle_request(request.body().to_string(), &webview, &proxy).await {
                            error!("IPC request failed: {}", e);
                        }
                    });
                });

            window_builder.with_webview(webview_builder)
        });

    // Run the event loop
    app.run(|event, _, control_flow| {
        match event {
            UserEvent::JavaScript(js_code) => {
                if let Some(ref mut webview) = *webview.lock() {
                    debug!("Executing JavaScript: {}", js_code);
                    if let Err(e) = webview.evaluate_script(&js_code) {
                        error!("Failed to execute JavaScript: {}", e);
                    }
                }
            }
            _ => {}
        }
    });

    Ok(())
}
