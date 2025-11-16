// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Database module commands
mod database;
mod sync;

// Tauri commands
#[tauri::command]
async fn greet(name: String) -> Result<String, String> {
    Ok(format!("Hello, {}! Welcome to EvolveApp.", name))
}

#[tauri::command]
async fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
async fn execute_sql(query: String) -> Result<String, String> {
    database::execute_query(&query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn sync_data(module_id: String) -> Result<String, String> {
    sync::sync_module(&module_id)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_app_version,
            execute_sql,
            sync_data
        ])
        .setup(|app| {
            // Initialize database on startup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = database::initialize_database().await {
                    eprintln!("Failed to initialize database: {}", e);
                    // Show error dialog on Windows
                    #[cfg(target_os = "windows")]
                    {
                        use std::process::Command;
                        let _ = Command::new("msg")
                            .args(["/time:10", "*", &format!("EvolveApp Error: {}", e)])
                            .spawn();
                    }
                }
            });

            // Log startup success
            println!("EvolveApp started successfully");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
