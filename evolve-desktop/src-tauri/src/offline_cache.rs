//! Durable key/value store for the web app's read-only offline email cache.
//!
//! The web UI keeps its offline copy in IndexedDB, which a webview/OS may evict. These
//! commands let the page mirror that copy to files in the app data dir, so saved mail
//! survives. One directory per signed-in user; one small JSON file per entry.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;

const MAX_VALUE_BYTES: usize = 8 * 1024 * 1024;
const MAX_ENTRIES: usize = 800;

/// Stable across Rust versions (DefaultHasher is not), so entries survive app updates.
fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Only the EIQ app's own pages may use the cache. Any other site the content webview
/// shows (custom tabs) must not be able to read or fill it.
fn from_app_origin(webview: &tauri::Webview) -> bool {
    match webview.url() {
        Ok(u) => {
            u.scheme() == "https"
                && matches!(u.host_str(), Some(h) if h == "evolvepreneuriq.app" || h.ends_with(".evolvepreneuriq.app"))
        }
        Err(_) => false,
    }
}

fn user_dir(app: &tauri::AppHandle, user_id: &str) -> Result<PathBuf, String> {
    // user_id becomes a directory name: digits only, so it cannot traverse paths.
    if user_id.is_empty() || user_id.len() > 20 || !user_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("invalid user id".to_string());
    }
    let base = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(base.join("offline-cache").join(user_id))
}

fn entry_path(dir: &PathBuf, key: &str) -> PathBuf {
    dir.join(format!("{:016x}.json", fnv1a(key)))
}

fn prune(dir: &PathBuf) {
    let mut files: Vec<(SystemTime, PathBuf)> = match fs::read_dir(dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
            .filter_map(|e| {
                let t = e.metadata().ok()?.modified().ok()?;
                Some((t, e.path()))
            })
            .collect(),
        Err(_) => return,
    };
    if files.len() <= MAX_ENTRIES {
        return;
    }
    files.sort_by_key(|(t, _)| *t);
    let excess = files.len() - MAX_ENTRIES;
    for (_, p) in files.into_iter().take(excess) {
        let _ = fs::remove_file(p);
    }
}

#[tauri::command]
pub async fn offline_cache_put(
    app: tauri::AppHandle,
    webview: tauri::Webview,
    user_id: String,
    key: String,
    value: String,
) -> Result<(), String> {
    if !from_app_origin(&webview) {
        return Err("offline_cache: not allowed from this page".to_string());
    }
    if value.len() > MAX_VALUE_BYTES || key.len() > 2048 {
        return Err("offline_cache: entry too large".to_string());
    }
    let data: serde_json::Value = serde_json::from_str(&value).map_err(|e| e.to_string())?;
    let dir = user_dir(&app, &user_id)?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let wrapped = serde_json::json!({ "key": key, "ts": ts, "data": data });
    let path = entry_path(&dir, &key);
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, wrapped.to_string()).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
    prune(&dir);
    Ok(())
}

/// Returns the stored `{"key","ts","data"}` JSON text, or None when there is no entry.
#[tauri::command]
pub async fn offline_cache_get(
    app: tauri::AppHandle,
    webview: tauri::Webview,
    user_id: String,
    key: String,
) -> Result<Option<String>, String> {
    if !from_app_origin(&webview) {
        return Err("offline_cache: not allowed from this page".to_string());
    }
    let dir = user_dir(&app, &user_id)?;
    let text = match fs::read_to_string(entry_path(&dir, &key)) {
        Ok(t) => t,
        Err(_) => return Ok(None),
    };
    // Guard against hash collisions: only return an entry that really is for this key.
    let parsed: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };
    if parsed.get("key").and_then(|k| k.as_str()) != Some(key.as_str()) {
        return Ok(None);
    }
    Ok(Some(text))
}

/// Delete every cached entry for this user (called on logout).
#[tauri::command]
pub async fn offline_cache_clear(
    app: tauri::AppHandle,
    webview: tauri::Webview,
    user_id: String,
) -> Result<(), String> {
    if !from_app_origin(&webview) {
        return Err("offline_cache: not allowed from this page".to_string());
    }
    let dir = user_dir(&app, &user_id)?;
    match fs::remove_dir_all(&dir) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
