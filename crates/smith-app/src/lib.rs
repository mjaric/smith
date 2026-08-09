//! Smith application shell (Slice 0 skeleton).
//!
//! Wires logging and the Tauri host. The model API, store, analysis engine,
//! and MCP server land in later slices per `docs/99-implementation-guide.md`.

use tracing::info;

/// Initializes logging and launches the Tauri host.
///
/// Startup order will grow per REQ-ARCH-012 (open store, hydrate projection,
/// start MCP, launch webview); Slice 0 only proves the window opens.
pub fn run() {
    tracing_subscriber::fmt::init();
    info!(app = "smith", "starting Smith");

    let result = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!());
    if let Err(error) = result {
        tracing::error!(%error, "Smith failed to start");
    }
}

/// IPC smoke command: proves the webview can call into the Rust core.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name} — from Smith's Rust core.")
}
