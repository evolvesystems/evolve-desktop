// System diagnostics and crash reporting module

use serde::{Deserialize, Serialize};
use sysinfo::System;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub timestamp: String,
    pub app_version: String,
    pub os: String,
    pub os_version: String,
    pub architecture: String,
    pub total_memory_mb: u64,
    pub available_memory_mb: u64,
    pub webview2_available: bool,
    pub startup_errors: Vec<String>,
    pub log_path: String,
}

/// Collect comprehensive system diagnostics
pub fn collect_diagnostics() -> DiagnosticReport {
    let mut sys = System::new_all();
    sys.refresh_all();

    let timestamp = chrono::Utc::now().to_rfc3339();
    let app_version = env!("CARGO_PKG_VERSION").to_string();

    // OS information
    let os = std::env::consts::OS.to_string();
    let os_version = System::long_os_version().unwrap_or_else(|| "Unknown".to_string());
    let architecture = std::env::consts::ARCH.to_string();

    // Memory information
    let total_memory_mb = sys.total_memory() / 1024 / 1024;
    let available_memory_mb = sys.available_memory() / 1024 / 1024;

    // WebView2 check (Windows-specific)
    let webview2_available = check_webview2();

    // Log file location
    let log_path = dirs::data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("com.evolveapp.desktop")
        .join("logs")
        .join("evolveapp.log")
        .to_string_lossy()
        .to_string();

    DiagnosticReport {
        timestamp,
        app_version,
        os,
        os_version,
        architecture,
        total_memory_mb,
        available_memory_mb,
        webview2_available,
        startup_errors: vec![],
        log_path,
    }
}

/// Check if WebView2 is available (Windows only)
fn check_webview2() -> bool {
    #[cfg(target_os = "windows")]
    {
        // Check common WebView2 installation paths
        let webview2_paths = [
            r"C:\Program Files (x86)\Microsoft\EdgeWebView\Application",
            r"C:\Program Files\Microsoft\EdgeWebView\Application",
        ];

        for path in &webview2_paths {
            if std::path::Path::new(path).exists() {
                tracing::info!("WebView2 found at: {}", path);
                return true;
            }
        }

        tracing::warn!("WebView2 not found in standard installation paths");
        false
    }

    #[cfg(not(target_os = "windows"))]
    {
        true // WebView2 only required on Windows
    }
}

/// Log all diagnostics at startup
pub fn log_diagnostics(report: &DiagnosticReport) {
    tracing::info!("=== System Diagnostics ===");
    tracing::info!("App Version: {}", report.app_version);
    tracing::info!("OS: {} {}", report.os, report.os_version);
    tracing::info!("Architecture: {}", report.architecture);
    tracing::info!("Memory: {} MB total, {} MB available",
        report.total_memory_mb,
        report.available_memory_mb
    );
    tracing::info!("WebView2 Available: {}", report.webview2_available);
    tracing::info!("Log Location: {}", report.log_path);
    tracing::info!("==========================");

    // Warn if WebView2 is missing on Windows
    #[cfg(target_os = "windows")]
    if !report.webview2_available {
        tracing::error!("CRITICAL: WebView2 is not installed!");
        tracing::error!("Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/");
    }
}

/// Send diagnostics to EIQ Manager (optional, with user consent)
pub async fn send_diagnostics_to_server(
    report: &DiagnosticReport,
    api_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = format!("{}/api/v1/app-diagnostics", api_url);

    tracing::info!("Sending diagnostics to: {}", endpoint);

    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .json(report)
        .send()
        .await?;

    if response.status().is_success() {
        tracing::info!("Diagnostics sent successfully");
        Ok(())
    } else {
        let error = format!("Failed to send diagnostics: {}", response.status());
        tracing::error!("{}", error);
        Err(error.into())
    }
}

/// Create a crash report file
pub fn create_crash_report(error_message: &str) {
    let mut report = collect_diagnostics();
    report.startup_errors.push(error_message.to_string());

    // Save to crash report file
    let crash_report_path = dirs::data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("com.evolveapp.desktop")
        .join("logs")
        .join("crash_report.json");

    if let Ok(json) = serde_json::to_string_pretty(&report) {
        if let Err(e) = std::fs::write(&crash_report_path, json) {
            tracing::error!("Failed to write crash report: {}", e);
        } else {
            tracing::info!("Crash report saved to: {}", crash_report_path.display());
        }
    }
}
