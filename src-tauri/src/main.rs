// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use std::fs;

// Database module commands
mod database;
mod sync;
mod diagnostics;

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
    // Initialize logging to file
    init_logging();

    tracing::info!("EvolveApp starting...");
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Collect and log system diagnostics
    let diagnostic_report = diagnostics::collect_diagnostics();
    diagnostics::log_diagnostics(&diagnostic_report);

    // Critical check: WebView2 must be installed on Windows
    #[cfg(target_os = "windows")]
    if !diagnostic_report.webview2_available {
        let error_msg = "WebView2 is not installed. This is required for EvolveApp to run on Windows.";
        diagnostics::create_crash_report(error_msg);

        // Show error dialog on Windows
        use std::process::Command;
        let _ = Command::new("msg")
            .args(["/time:30", "*", &format!("EvolveApp Error: {}\n\nDownload WebView2 from:\nhttps://developer.microsoft.com/en-us/microsoft-edge/webview2/", error_msg)])
            .spawn();

        panic!("{}", error_msg);
    }

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
            tracing::info!("Running setup...");

            // Initialize database on startup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tracing::info!("Initializing database connection...");
                if let Err(e) = database::initialize_database().await {
                    tracing::error!("Failed to initialize database: {}", e);
                    // Show error dialog on Windows
                    #[cfg(target_os = "windows")]
                    {
                        use std::process::Command;
                        let _ = Command::new("msg")
                            .args(["/time:10", "*", &format!("EvolveApp Error: {}", e)])
                            .spawn();
                    }
                } else {
                    tracing::info!("Database initialized successfully");
                }

                // Optionally send diagnostics to EIQ Manager (non-blocking)
                let api_config = database::get_api_config().await;
                let diag_report = diagnostics::collect_diagnostics();

                if let Err(e) = diagnostics::send_diagnostics_to_server(&diag_report, &api_config.base_url).await {
                    tracing::warn!("Failed to send diagnostics to server: {}", e);
                    // Don't fail startup if diagnostics can't be sent
                }
            });

            tracing::info!("Setup completed successfully");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Initialize logging to file
/// Logs are written to: %APPDATA%/com.evolveapp.desktop/logs/ (Windows)
///                      ~/Library/Application Support/com.evolveapp.desktop/logs/ (macOS)
///                      ~/.local/share/com.evolveapp.desktop/logs/ (Linux)
fn init_logging() {
    // Get app data directory for logs
    let app_dir = dirs::data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("com.evolveapp.desktop");

    let log_dir = app_dir.join("logs");

    // Create log directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&log_dir) {
        eprintln!("Failed to create log directory: {}", e);
        return;
    }

    // Create file appender for daily log rotation
    let file_appender = tracing_appender::rolling::daily(log_dir.clone(), "evolveapp.log");

    // Create console writer for stdout
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Build logging layers
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true);

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(false);

    // Initialize subscriber with both file and console output
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tauri=debug,evolve_desktop=debug".into())
        )
        .with(file_layer)
        .with(stdout_layer)
        .init();

    // Log the log file location
    tracing::info!("Logs directory: {}", log_dir.display());
}
