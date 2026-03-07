use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"Hello from Ionyx + Leptos! 🚀"</h1>
            <p>"This is a Rust WASM frontend compiled to WebAssembly"</p>
            <div class="features">
                <h2>"🚀 Ionyx Framework Features"</h2>
                <ul>
                    <li>"✅ File System Access"</li>
                    <li>"✅ Network Requests"</li>
                    <li>"✅ OS Information"</li>
                    <li>"✅ Cross-platform Desktop Apps"</li>
                    <li>"✅ Rust Backend Performance"</li>
                    <li>"✅ Rust Frontend (WASM)"</li>
                    <li>"✅ WebGPU Support Enabled"</li>
                </ul>
            </div>
            <p>"Edit this file and reload to see changes!"</p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    
    leptos::mount_to_body(App);
}
