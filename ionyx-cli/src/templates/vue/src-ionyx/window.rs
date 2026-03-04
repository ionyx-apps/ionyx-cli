use serde::{Deserialize, Serialize};
use tao::dpi::LogicalSize;
use tao::window::WindowBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub fullscreen: bool,
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Ionyx Application".to_string(),
            width: 1200,
            height: 800,
            resizable: true,
            fullscreen: false,
            min_width: Some(800),
            min_height: Some(600),
            max_width: None,
            max_height: None,
        }
    }
}

impl WindowConfig {
    pub fn apply_to_builder(&self, builder: WindowBuilder) -> WindowBuilder {
        let mut builder = builder
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height))
            .with_resizable(self.resizable);

        if let Some(min_width) = self.min_width {
            if let Some(min_height) = self.min_height {
                builder = builder.with_min_inner_size(LogicalSize::new(min_width, min_height));
            }
        }

        if let Some(max_width) = self.max_width {
            if let Some(max_height) = self.max_height {
                builder = builder.with_max_inner_size(LogicalSize::new(max_width, max_height));
            }
        }

        builder
    }

    pub fn apply_to_webview(&self, _webview: &tao::window::Window) -> Result<(), wry::Error> {
        // Apply window-specific configurations to webview
        // This would need to be implemented based on actual webview type
        if self.fullscreen {
            // _webview.set_fullscreen(true);
        }

        Ok(())
    }
}
