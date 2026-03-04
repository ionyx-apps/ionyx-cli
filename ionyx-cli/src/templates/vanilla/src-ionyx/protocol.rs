use anyhow::Result;
use std::borrow::Cow;
use tracing::debug;
use wry::http::{Request, Response, StatusCode};
use include_dir::include_dir;

// For development mode, we'll use a simple fallback
#[cfg(debug_assertions)]
static EMBEDDED_ASSETS: Option<include_dir::Dir<'static>> = None;

// For release mode, embed the frontend assets
#[cfg(not(debug_assertions))]
static EMBEDDED_ASSETS: Option<include_dir::Dir<'static>> = {
    Some(include_dir!("$CARGO_MANIFEST_DIR/../frontend/dist"))
};

pub struct CustomProtocolHandler;

impl CustomProtocolHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_request(&self, request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
        let path = request.uri().path();
        
        // Remove leading slash and decode
        let clean_path = path.trim_start_matches('/');
        let file_path = if clean_path.is_empty() || clean_path == "/" {
            "index.html"
        } else {
            clean_path
        };

        // In development mode, return a simple HTML page
        if cfg!(debug_assertions) {
            let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Ionyx Development Mode</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #1a1a1a; color: white; }
        .container { max-width: 800px; margin: 0 auto; }
        h1 { color: #61dafb; }
        .status { background: #282c34; padding: 20px; border-radius: 8px; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 Ionyx Framework - Development Mode</h1>
        <div class="status">
            <h2>Backend Status: ✅ Connected</h2>
            <p>The Rust backend is running successfully.</p>
            <p>Frontend should be served from Vite dev server at http://localhost:5173</p>
        </div>
        <p>This is a fallback page. Start the frontend development server:</p>
        <pre><code>cd frontend && npm run dev</code></pre>
    </div>
</body>
</html>
            "#;
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Cow::Owned(html.as_bytes().to_vec()))
                .unwrap();
        }

        // Try to find the file in embedded assets
        if let Some(assets) = &EMBEDDED_ASSETS {
            match assets.get_file(file_path) {
                Some(file) => {
                    let content_type = Self::get_content_type(file_path);
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", content_type)
                        .body(Cow::Borrowed(file.contents()))
                        .unwrap()
                }
                None => {
                    // Try to find index.html for SPA routing
                    if let Some(index_file) = assets.get_file("index.html") {
                        Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "text/html")
                            .body(Cow::Borrowed(index_file.contents()))
                            .unwrap()
                    } else {
                        Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Cow::Borrowed(&b"404 Not Found"[..]))
                            .unwrap()
                    }
                }
            }
        } else {
            // No embedded assets available
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Cow::Borrowed(&b"No embedded assets available"[..]))
                .unwrap()
        }
    }

    fn get_content_type(file_path: &str) -> &'static str {
        match std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
        {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("woff") => "font/woff",
            Some("woff2") => "font/woff2",
            Some("ttf") => "font/ttf",
            Some("eot") => "application/vnd.ms-fontobject",
            _ => "application/octet-stream",
        }
    }
}
