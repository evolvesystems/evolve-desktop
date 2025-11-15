// Database module - Remote API only
// All database operations go through eiq-manager API

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub api_key: Option<String>,
}

// No local database initialization needed
pub async fn initialize_database() -> Result<(), Box<dyn std::error::Error>> {
    println!("Using remote database via eiq-manager API");
    println!("No local database initialization required");
    Ok(())
}

// API configuration
pub async fn get_api_config() -> ApiConfig {
    ApiConfig {
        base_url: std::env::var("API_URL")
            .unwrap_or_else(|_| "http://localhost:8000".to_string()),
        api_key: std::env::var("API_KEY").ok(),
    }
}

// Helper to check API connection
pub async fn check_api_connection() -> Result<bool, Box<dyn std::error::Error>> {
    let config = get_api_config().await;

    // TODO: Make actual API health check request
    println!("Checking API connection to: {}", config.base_url);

    // For now, return true
    // In production, make actual HTTP request to /api/health
    Ok(true)
}

// Execute query is not needed - frontend uses axios for API calls
pub async fn execute_query(_query: &str) -> Result<String, Box<dyn std::error::Error>> {
    Err("Local database queries not supported. Use API endpoints instead.".into())
}
