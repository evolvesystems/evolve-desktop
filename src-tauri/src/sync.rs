// Sync module - Remote API only
// All data operations go through eiq-manager API via frontend axios calls
// This module is mainly for future background sync features

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStatus {
    pub connected: bool,
    pub last_sync: Option<String>,
    pub pending_operations: usize,
}

// Check sync status (API connection)
pub async fn get_sync_status() -> Result<SyncStatus, Box<dyn std::error::Error>> {
    // Check if API is reachable
    let connected = crate::database::check_api_connection().await?;

    Ok(SyncStatus {
        connected,
        last_sync: Some(chrono::Utc::now().to_rfc3339()),
        pending_operations: 0,
    })
}

// Module sync is handled by frontend API calls
pub async fn sync_module(module_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Sync requested for module: {}", module_id);
    println!("Data synced automatically via API calls - no local storage");

    let status = get_sync_status().await?;
    Ok(serde_json::to_string(&status)?)
}

// Background sync can be implemented here in the future
// For now, all operations are immediate via API
pub async fn start_background_sync() -> Result<(), Box<dyn std::error::Error>> {
    println!("Background sync not needed - using direct API calls");
    Ok(())
}
