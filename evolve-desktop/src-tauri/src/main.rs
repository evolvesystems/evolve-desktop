// EvolveApp Desktop — Multi-Webview Architecture with Browser Tabs
//
// Three webview regions in one window:
//   - "sidebar" (56px, left column, full height) — local sidebar.html
//   - "tabbar"  (right column, 30px height)      — local tabbar.html
//   - "tab_N"   (right column, remaining height)  — content webviews (one per tab)
//
// Uses Tauri 2.x multi-webview (unstable feature).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::PageLoadEvent,
    Emitter, Manager, WebviewUrl,
};
use tauri_plugin_updater::UpdaterExt;

const APP_URL: &str = "https://evolvepreneuriq.app";
const SIDEBAR_WIDTH: f64 = 56.0;
const SIDEBAR_EXPANDED: f64 = 316.0;
const TABBAR_HEIGHT: f64 = 30.0;

/// Default tabs on first launch (no saved session)
const DEFAULT_STARTUP_TABS: &[(&str, &str)] = &[
    ("Email", "/email-manager"),
    ("Chat", "/chat"),
    ("Dashboard", "/dashboard"),
];

const SETTINGS_FILE: &str = "app_settings.json";

// =====================================================================
//  TAB STATE
// =====================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TabInfo {
    id: String,
    url: String,
    title: String,
    active: bool,
}

struct TabState {
    tabs: Vec<TabInfo>,
    next_id: u32,
}

struct AppTabs(Mutex<TabState>);

impl AppTabs {
    fn new() -> Self {
        Self(Mutex::new(TabState {
            tabs: Vec::new(),
            next_id: 1,
        }))
    }
}

// =====================================================================
//  HELPERS
// =====================================================================

fn run_js(app: &tauri::AppHandle, label: &str, js: &str) {
    if let Some(wv) = app.get_webview(label) {
        let _ = wv.eval(js);
    }
}

/// Emit the full tab list to the tabbar webview so it can re-render.
fn emit_tabs_to_tabbar(app: &tauri::AppHandle, tabs: &[TabInfo]) {
    let json = serde_json::to_string(tabs).unwrap_or_else(|_| "[]".into());
    let _ = app.emit_to("tabbar", "tabs-updated", &json);
}

/// Get the label for a tab webview.
fn tab_label(id: &str) -> String {
    format!("tab_{}", id)
}

/// Navigate a specific content tab to a URL.
fn nav_tab(app: &tauri::AppHandle, label: &str, url: &str) {
    run_js(
        app,
        label,
        &format!(
            "window.location.href='{}'",
            url.replace('\'', "\\'")
        ),
    );
}

/// Get current window dimensions (logical).
fn get_window_size(app: &tauri::AppHandle) -> (f64, f64) {
    if let Some(win) = app.get_window("main") {
        let scale = win.scale_factor().unwrap_or(1.0);
        if let Ok(phys) = win.inner_size() {
            return (phys.width as f64 / scale, phys.height as f64 / scale);
        }
    }
    (1400.0, 900.0)
}

/// Calculate the current sidebar width (check if config panel is open by checking sidebar webview width).
fn get_sidebar_width(app: &tauri::AppHandle) -> f64 {
    // We always use SIDEBAR_WIDTH for tab/content positioning.
    // The config panel is an overlay inside the sidebar webview.
    SIDEBAR_WIDTH
}

/// Build the JS injection for content webviews (fetches tabs, badges, sidebar data, page title).
fn content_injection_js(tab_id: &str) -> String {
    format!(
        r#"
        (async function() {{
            if (!window.__TAURI_INTERNALS__) return;
            // Inject tab ID so we can reference it
            window.__EVOLVE_TAB_ID__ = '{tab_id}';

            // Fetch sidebar tabs
            try {{
                var r = await fetch('/api/v1/desktop/sidebar/tabs', {{credentials:'include'}});
                if (r.ok) {{
                    var d = await r.json();
                    if (d.tabs) window.__TAURI_INTERNALS__.invoke('relay_tabs_to_sidebar', {{tabsJson: JSON.stringify(d.tabs)}});
                }}
            }} catch(e) {{}}

            // Fetch badge data
            try {{
                var r2 = await fetch('/api/v1/desktop/check-notifications', {{credentials:'include'}});
                if (r2.ok) {{
                    var d2 = await r2.json();
                    window.__TAURI_INTERNALS__.invoke('relay_badges_to_sidebar', {{badgesJson: JSON.stringify(d2)}});
                }}
            }} catch(e) {{}}

            // Extract page title and send to tab bar
            var title = document.title || 'New Tab';
            // Strip common suffixes
            title = title.replace(/ \| EvolveApp.*$/i, '').replace(/ - Evolve.*$/i, '').trim() || 'New Tab';
            window.__TAURI_INTERNALS__.invoke('update_tab_title', {{tabId: '{tab_id}', title: title}});

            // Keyboard shortcuts for tab management (Ctrl+T, Ctrl+W, Ctrl+Tab, etc.)
            if (!window.__EVOLVE_SHORTCUTS_BOUND__) {{
                window.__EVOLVE_SHORTCUTS_BOUND__ = true;
                document.addEventListener('keydown', function(e) {{
                    var ctrl = e.ctrlKey || e.metaKey;
                    if (ctrl && e.key === 't') {{ e.preventDefault(); window.__TAURI_INTERNALS__.invoke('create_tab', {{url:''}}); }}
                    if (ctrl && e.key === 'w') {{ e.preventDefault(); window.__TAURI_INTERNALS__.invoke('close_tab', {{tabId: window.__EVOLVE_TAB_ID__}}); }}
                    if (ctrl && e.key === 'n') {{ e.preventDefault(); var btn = document.querySelector('[data-action*="openCompose"]') || document.getElementById('btn-new-email'); if (btn) btn.click(); else window.location.href = '/email-manager?compose=true'; }}
                    if (ctrl && !e.shiftKey && e.key === 'Tab') {{ e.preventDefault(); window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {{event:'tab-shortcut',payload:'next'}}); }}
                    if (ctrl && e.shiftKey && e.key === 'Tab') {{ e.preventDefault(); window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {{event:'tab-shortcut',payload:'prev'}}); }}
                    if (ctrl && e.key >= '1' && e.key <= '9') {{ e.preventDefault(); window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {{event:'tab-shortcut',payload:'switch-'+e.key}}); }}
                }});
            }}
        }})();
        "#,
        tab_id = tab_id
    )
}

/// Create a content webview for a tab. Returns the webview label.
fn create_tab_webview(
    app: &tauri::AppHandle,
    tab_id: &str,
    url: &str,
    visible: bool,
) -> Result<String, String> {
    let label = tab_label(tab_id);
    let (w, h) = get_window_size(app);
    let sw = get_sidebar_width(app);
    let content_w = (w - sw).max(100.0);
    let content_h = (h - TABBAR_HEIGHT).max(100.0);

    let webview_url = if url.is_empty() {
        WebviewUrl::External(format!("{}/dashboard", APP_URL).parse().unwrap())
    } else if url.starts_with("http") {
        WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?)
    } else {
        WebviewUrl::External(format!("{}{}", APP_URL, url).parse().unwrap())
    };

    let app_handle = app.clone();
    let tab_id_owned = tab_id.to_string();

    let builder = tauri::webview::WebviewBuilder::new(&label, webview_url)
        .user_agent(&format!(
            "EvolveApp/{} Tauri/2",
            env!("CARGO_PKG_VERSION")
        ))
        .background_color(tauri::window::Color(15, 15, 25, 255))
        .on_page_load(move |webview, payload| {
            let url_str = payload.url().to_string();
            match payload.event() {
                PageLoadEvent::Started => {
                    println!("[page-load] Tab {} Started: {}", tab_id_owned, url_str);
                    let _ = app_handle.emit_to("sidebar", "content-loading", true);
                    return;
                }
                PageLoadEvent::Finished => {
                    println!("[page-load] Tab {} Finished: {}", tab_id_owned, url_str);
                    let _ = app_handle.emit_to("sidebar", "content-loaded", true);
                }
                _ => return,
            }

            // Show window on first load
            if let Some(win) = app_handle.get_window("main") {
                let _ = win.show();
            }

            // Emit URL to sidebar for active tab highlighting
            if let Ok(url) = webview.url() {
                let _ =
                    app_handle.emit_to("sidebar", "content-navigated", url.to_string());
            }

            // Update the tab URL in state
            if let Some(state) = app_handle.try_state::<AppTabs>() {
                if let Ok(mut guard) = state.0.lock() {
                    if let Some(tab) = guard.tabs.iter_mut().find(|t| t.id == tab_id_owned) {
                        if let Ok(u) = webview.url() {
                            tab.url = u.to_string();
                        }
                    }
                    emit_tabs_to_tabbar(&app_handle, &guard.tabs);
                }
            }

            // Inject JS for sidebar data + title extraction
            let js = content_injection_js(&tab_id_owned);
            let _ = webview.eval(&js);
        });

    let window = app
        .get_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    let _webview = window
        .add_child(
            builder,
            tauri::LogicalPosition::new(sw, TABBAR_HEIGHT),
            tauri::LogicalSize::new(content_w, content_h),
        )
        .map_err(|e| e.to_string())?;

    // If not the active tab, hide it
    if !visible {
        if let Some(wv) = app.get_webview(&label) {
            let _ = wv.set_position(tauri::LogicalPosition::new(-9999.0, -9999.0));
            let _ = wv.set_size(tauri::LogicalSize::new(0.0, 0.0));
        }
    }

    Ok(label)
}

/// Show the active tab webview and hide all others.
fn show_active_hide_others(app: &tauri::AppHandle, tabs: &[TabInfo]) {
    let (w, h) = get_window_size(app);
    let sw = get_sidebar_width(app);
    let content_w = (w - sw).max(100.0);
    let content_h = (h - TABBAR_HEIGHT).max(100.0);

    for tab in tabs {
        let label = tab_label(&tab.id);
        if let Some(wv) = app.get_webview(&label) {
            if tab.active {
                let _ = wv.set_position(tauri::LogicalPosition::new(sw, TABBAR_HEIGHT));
                let _ = wv.set_size(tauri::LogicalSize::new(content_w, content_h));
            } else {
                let _ = wv.set_position(tauri::LogicalPosition::new(-9999.0, -9999.0));
                let _ = wv.set_size(tauri::LogicalSize::new(0.0, 0.0));
            }
        }
    }
}

/// Save tab state to app data directory for persistence across restarts.
fn save_tab_state(app: &tauri::AppHandle, tabs: &[TabInfo]) {
    if let Ok(dir) = app.path().app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let json = serde_json::to_string(tabs).unwrap_or_else(|_| "[]".into());
        let _ = fs::write(dir.join("browser_tabs.json"), &json);
    }
}

/// Load tab state from app data directory.
fn load_tab_state(app: &tauri::AppHandle) -> Option<Vec<TabInfo>> {
    let dir = app.path().app_data_dir().ok()?;
    let data = fs::read_to_string(dir.join("browser_tabs.json")).ok()?;
    serde_json::from_str(&data).ok()
}

// =====================================================================
//  LOCAL SETTINGS
// =====================================================================

fn save_settings(app: &tauri::AppHandle, settings: &serde_json::Value) {
    if let Ok(dir) = app.path().app_data_dir() {
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(dir.join(SETTINGS_FILE), serde_json::to_string_pretty(settings).unwrap_or_default());
    }
}

fn load_settings(app: &tauri::AppHandle) -> serde_json::Value {
    app.path()
        .app_data_dir()
        .ok()
        .and_then(|dir| fs::read_to_string(dir.join(SETTINGS_FILE)).ok())
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(|| serde_json::json!({}))
}

// =====================================================================
//  TAURI COMMANDS
// =====================================================================

#[tauri::command]
async fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get a local setting by key
#[tauri::command]
async fn get_setting(app: tauri::AppHandle, key: String) -> Result<serde_json::Value, String> {
    let settings = load_settings(&app);
    Ok(settings.get(&key).cloned().unwrap_or(serde_json::Value::Null))
}

/// Save a local setting
#[tauri::command]
async fn set_setting(app: tauri::AppHandle, key: String, value: serde_json::Value) -> Result<(), String> {
    let mut settings = load_settings(&app);
    if let Some(obj) = settings.as_object_mut() {
        obj.insert(key, value);
    }
    save_settings(&app, &settings);
    Ok(())
}

/// Get all local settings
#[tauri::command]
async fn get_all_settings(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    Ok(load_settings(&app))
}

/// Set startup tabs config
#[tauri::command]
async fn set_startup_tabs(app: tauri::AppHandle, tabs: Vec<String>) -> Result<(), String> {
    let mut settings = load_settings(&app);
    if let Some(obj) = settings.as_object_mut() {
        obj.insert("startup_tabs".into(), serde_json::json!(tabs));
    }
    save_settings(&app, &settings);
    Ok(())
}

#[tauri::command]
async fn save_cached_tabs(app: tauri::AppHandle, tabs_json: String) -> Result<(), String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    fs::write(dir.join("sidebar_tabs.json"), &tabs_json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn load_cached_tabs(app: tauri::AppHandle) -> Result<String, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::read_to_string(dir.join("sidebar_tabs.json")).map_err(|e| e.to_string())
}

/// Navigate the active tab to a URL. Called from sidebar.
#[tauri::command]
async fn navigate_content(app: tauri::AppHandle, url: String) -> Result<(), String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    if let Some(active) = guard.tabs.iter().find(|t| t.active) {
        let label = tab_label(&active.id);
        drop(guard);
        nav_tab(&app, &label, &url);
    }
    Ok(())
}

/// Navigate the active tab OR switch to an existing tab with that URL.
/// Called from sidebar for smart tab switching.
#[tauri::command]
async fn navigate_or_switch_tab(
    app: tauri::AppHandle,
    url: String,
    force_new: bool,
) -> Result<(), String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;

    // Find existing tab or active tab — all inside a sync block, no .await
    let action = {
        let guard = state.0.lock().map_err(|e| e.to_string())?;

        if !force_new {
            let existing = guard.tabs.iter().find(|t| {
                let tab_path = t.url.replace(APP_URL, "");
                let req_path = url.replace(APP_URL, "");
                tab_path == req_path
                    || t.url == url
                    || (req_path.len() > 1 && tab_path.starts_with(&req_path))
            });
            if let Some(found) = existing {
                Some(found.id.clone()) // switch to this tab
            } else {
                None // navigate active tab
            }
        } else {
            None
        }
    }; // guard dropped here

    if let Some(tab_id) = action {
        return switch_tab(app, tab_id).await;
    }

    // Navigate active tab
    let active_label = {
        let guard = state.0.lock().map_err(|e| e.to_string())?;
        guard.tabs.iter().find(|t| t.active).map(|t| tab_label(&t.id))
    };
    if let Some(label) = active_label {
        nav_tab(&app, &label, &url);
    }
    Ok(())
}

#[tauri::command]
async fn get_content_url(app: tauri::AppHandle) -> Result<String, String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    if let Some(active) = guard.tabs.iter().find(|t| t.active) {
        let label = tab_label(&active.id);
        drop(guard);
        app.get_webview(&label)
            .ok_or_else(|| "Active tab webview not found".to_string())
            .and_then(|wv| wv.url().map(|u| u.to_string()).map_err(|e| e.to_string()))
    } else {
        Err("No active tab".to_string())
    }
}

#[tauri::command]
async fn content_go_back(app: tauri::AppHandle) -> Result<(), String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    if let Some(active) = guard.tabs.iter().find(|t| t.active) {
        let label = tab_label(&active.id);
        drop(guard);
        run_js(&app, &label, "history.back()");
    }
    Ok(())
}

#[tauri::command]
async fn content_reload(app: tauri::AppHandle) -> Result<(), String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    if let Some(active) = guard.tabs.iter().find(|t| t.active) {
        let label = tab_label(&active.id);
        drop(guard);
        run_js(&app, &label, "location.reload()");
    }
    Ok(())
}

#[tauri::command]
async fn toggle_sidebar_config(app: tauri::AppHandle, open: bool) -> Result<(), String> {
    let win = app.get_window("main").ok_or("Window not found")?;
    let phys = win.inner_size().map_err(|e| e.to_string())?;
    let scale = win.scale_factor().unwrap_or(1.0);
    let w = phys.width as f64 / scale;
    let h = phys.height as f64 / scale;
    let sw = if open { SIDEBAR_EXPANDED } else { SIDEBAR_WIDTH };

    if let Some(sb) = app.get_webview("sidebar") {
        let _ = sb.set_size(tauri::LogicalSize::new(sw, h));
    }

    // Reposition tabbar
    if let Some(tb) = app.get_webview("tabbar") {
        let _ = tb.set_position(tauri::LogicalPosition::new(sw, 0.0));
        let _ = tb.set_size(tauri::LogicalSize::new((w - sw).max(100.0), TABBAR_HEIGHT));
    }

    // Reposition all content tab webviews
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    let content_w = (w - sw).max(100.0);
    let content_h = (h - TABBAR_HEIGHT).max(100.0);
    for tab in &guard.tabs {
        let label = tab_label(&tab.id);
        if let Some(wv) = app.get_webview(&label) {
            if tab.active {
                let _ = wv.set_position(tauri::LogicalPosition::new(sw, TABBAR_HEIGHT));
                let _ = wv.set_size(tauri::LogicalSize::new(content_w, content_h));
            }
        }
    }
    Ok(())
}

// --- Tab management commands (called from tabbar.html) ---

#[tauri::command]
async fn create_tab(app: tauri::AppHandle, url: String) -> Result<String, String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;

    let tab_id = format!("{}", guard.next_id);
    guard.next_id += 1;

    // Deactivate all existing tabs
    for t in guard.tabs.iter_mut() {
        t.active = false;
    }

    let resolved_url = if url.is_empty() {
        format!("{}/dashboard", APP_URL)
    } else if url.starts_with("http") {
        url.clone()
    } else {
        format!("{}{}", APP_URL, url)
    };

    let new_tab = TabInfo {
        id: tab_id.clone(),
        url: resolved_url,
        title: "New Tab".into(),
        active: true,
    };
    guard.tabs.push(new_tab);

    let tabs_snapshot = guard.tabs.clone();
    drop(guard);

    // Create the webview
    create_tab_webview(&app, &tab_id, &url, true)?;

    // Hide all other tab webviews
    show_active_hide_others(&app, &tabs_snapshot);

    // Notify tabbar
    emit_tabs_to_tabbar(&app, &tabs_snapshot);

    // Emit URL to sidebar
    let active_url = tabs_snapshot
        .iter()
        .find(|t| t.active)
        .map(|t| t.url.clone())
        .unwrap_or_default();
    let _ = app.emit_to("sidebar", "content-navigated", active_url);

    // Persist
    save_tab_state(&app, &tabs_snapshot);

    Ok(tab_id)
}

#[tauri::command]
async fn close_tab(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;

    // Don't close the last tab
    if guard.tabs.len() <= 1 {
        return Ok(());
    }

    let idx = guard
        .tabs
        .iter()
        .position(|t| t.id == tab_id)
        .ok_or("Tab not found")?;
    let was_active = guard.tabs[idx].active;
    let label = tab_label(&tab_id);

    guard.tabs.remove(idx);

    // If we closed the active tab, activate the nearest one
    if was_active && !guard.tabs.is_empty() {
        let new_idx = if idx >= guard.tabs.len() {
            guard.tabs.len() - 1
        } else {
            idx
        };
        guard.tabs[new_idx].active = true;
    }

    let tabs_snapshot = guard.tabs.clone();
    drop(guard);

    // Destroy the webview
    if let Some(wv) = app.get_webview(&label) {
        // We can't destroy a webview in Tauri 2.x easily, so hide it off-screen
        // and close it. The close() method removes it.
        let _ = wv.close();
    }

    // Show the new active tab
    show_active_hide_others(&app, &tabs_snapshot);

    // Notify tabbar
    emit_tabs_to_tabbar(&app, &tabs_snapshot);

    // Emit URL to sidebar for the newly active tab
    if let Some(active) = tabs_snapshot.iter().find(|t| t.active) {
        let _ = app.emit_to("sidebar", "content-navigated", active.url.clone());
    }

    // Persist
    save_tab_state(&app, &tabs_snapshot);

    Ok(())
}

#[tauri::command]
async fn switch_tab(app: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;

    // Check the tab exists
    if !guard.tabs.iter().any(|t| t.id == tab_id) {
        return Err("Tab not found".to_string());
    }

    // Deactivate all, activate target
    for t in guard.tabs.iter_mut() {
        t.active = t.id == tab_id;
    }

    let tabs_snapshot = guard.tabs.clone();
    drop(guard);

    show_active_hide_others(&app, &tabs_snapshot);
    emit_tabs_to_tabbar(&app, &tabs_snapshot);

    // Emit URL to sidebar
    if let Some(active) = tabs_snapshot.iter().find(|t| t.active) {
        let _ = app.emit_to("sidebar", "content-navigated", active.url.clone());
    }

    // Persist
    save_tab_state(&app, &tabs_snapshot);

    Ok(())
}

#[tauri::command]
async fn update_tab_title(app: tauri::AppHandle, tab_id: String, title: String) -> Result<(), String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;

    if let Some(tab) = guard.tabs.iter_mut().find(|t| t.id == tab_id) {
        tab.title = if title.is_empty() {
            "New Tab".into()
        } else {
            title
        };
    }

    let tabs_snapshot = guard.tabs.clone();
    drop(guard);

    emit_tabs_to_tabbar(&app, &tabs_snapshot);
    save_tab_state(&app, &tabs_snapshot);

    Ok(())
}

#[tauri::command]
async fn get_tabs(app: tauri::AppHandle) -> Result<Vec<TabInfo>, String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    Ok(guard.tabs.clone())
}

#[tauri::command]
async fn reorder_tabs(app: tauri::AppHandle, tab_ids: Vec<String>) -> Result<(), String> {
    let state = app
        .try_state::<AppTabs>()
        .ok_or("Tab state not found")?;
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;

    let mut reordered: Vec<TabInfo> = Vec::new();
    for id in &tab_ids {
        if let Some(tab) = guard.tabs.iter().find(|t| &t.id == id) {
            reordered.push(tab.clone());
        }
    }
    // Add any tabs not in the reorder list (shouldn't happen, but safe)
    for tab in &guard.tabs {
        if !tab_ids.contains(&tab.id) {
            reordered.push(tab.clone());
        }
    }
    guard.tabs = reordered;

    let tabs_snapshot = guard.tabs.clone();
    drop(guard);

    emit_tabs_to_tabbar(&app, &tabs_snapshot);
    save_tab_state(&app, &tabs_snapshot);

    Ok(())
}

/// Show info modal centered in the active tab webview
#[tauri::command]
async fn show_info_modal(app: tauri::AppHandle) -> Result<(), String> {
    let state = app.try_state::<AppTabs>();
    let active_label = if let Some(state) = state {
        let guard = state.0.lock().map_err(|e| e.to_string())?;
        guard
            .tabs
            .iter()
            .find(|t| t.active)
            .map(|t| tab_label(&t.id))
    } else {
        Some("content".to_string())
    };

    let label = active_label.unwrap_or_else(|| "content".to_string());
    let version = env!("CARGO_PKG_VERSION").to_string();
    let js = format!(r##"
(function() {{
  if (document.getElementById('evolve-info-modal')) {{
    document.getElementById('evolve-info-modal').remove();
    return;
  }}
  var overlay = document.createElement('div');
  overlay.id = 'evolve-info-modal';
  overlay.style.cssText = 'position:fixed;inset:0;z-index:99999;display:flex;align-items:center;justify-content:center;background:rgba(0,0,0,0.5);font-family:system-ui,-apple-system,sans-serif;';
  overlay.innerHTML = '<div style="background:#1e1e2e;border:1px solid rgba(255,255,255,0.1);border-radius:16px;padding:32px;min-width:340px;max-width:420px;color:#fff;box-shadow:0 20px 60px rgba(0,0,0,0.5);position:relative;">'
    + '<button id="evolve-info-close" style="position:absolute;top:12px;right:12px;width:32px;height:32px;border-radius:8px;border:none;background:rgba(255,255,255,0.08);color:rgba(255,255,255,0.6);font-size:18px;cursor:pointer;display:flex;align-items:center;justify-content:center;transition:all 0.15s;">&times;</button>'
    + '<div style="display:flex;align-items:center;gap:14px;margin-bottom:20px;">'
    + '<div style="width:48px;height:48px;border-radius:12px;background:linear-gradient(135deg,#9333ea,#3b82f6);display:flex;align-items:center;justify-content:center;color:#fff;font-weight:700;font-size:20px;flex-shrink:0;">E</div>'
    + '<div><div style="font-size:18px;font-weight:600;">EvolveApp Desktop</div><div style="font-size:13px;color:rgba(255,255,255,0.5);margin-top:2px;">Business Management Suite</div></div></div>'
    + '<div style="background:rgba(255,255,255,0.05);border-radius:10px;padding:14px;display:flex;flex-direction:column;gap:8px;">'
    + '<div style="display:flex;justify-content:space-between;font-size:13px;"><span style="color:rgba(255,255,255,0.5);">Version</span><span style="font-weight:500;">v{ver}</span></div>'
    + '<div style="display:flex;justify-content:space-between;font-size:13px;"><span style="color:rgba(255,255,255,0.5);">Latest</span><span id="evolve-info-latest" style="font-weight:500;">checking...</span></div>'
    + '</div>'
    + '<button id="evolve-info-check-update" style="width:100%;margin-top:16px;padding:10px;border-radius:10px;border:1px solid rgba(255,255,255,0.15);background:none;color:rgba(255,255,255,0.7);font-size:13px;cursor:pointer;font-family:system-ui;transition:all 0.15s;">Check for Updates</button>'
    + '</div>';
  document.body.appendChild(overlay);
  overlay.onclick = function(e) {{ if (e.target === overlay) overlay.remove(); }};
  document.getElementById('evolve-info-close').onclick = function() {{ overlay.remove(); }};
  document.getElementById('evolve-info-close').onmouseenter = function() {{ this.style.background='rgba(255,255,255,0.15)'; this.style.color='#fff'; }};
  document.getElementById('evolve-info-close').onmouseleave = function() {{ this.style.background='rgba(255,255,255,0.08)'; this.style.color='rgba(255,255,255,0.6)'; }};
  document.getElementById('evolve-info-check-update').onclick = function() {{
    this.textContent = 'Checking...';
    this.disabled = true;
    overlay.remove();
    window.__TAURI_INTERNALS__.invoke('check_for_updates');
  }};
  fetch('https://evolvepreneuriq.app/api/v1/desktop/version').then(function(r){{ return r.json(); }}).then(function(d){{
    var el = document.getElementById('evolve-info-latest');
    if (el && d.version) {{ el.textContent = 'v' + d.version; }}
  }}).catch(function(){{}});
}})();
"##, ver = version);
    run_js(&app, &label, &js);
    Ok(())
}

/// Check for updates using tauri-plugin-updater, show progress in active tab webview
#[tauri::command]
async fn check_for_updates(app: tauri::AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater.check().await.map_err(|e| e.to_string())?;

    // Find the active tab label
    let active_label = {
        let state = app.try_state::<AppTabs>();
        if let Some(state) = state {
            let guard = state.0.lock().map_err(|e| e.to_string())?;
            guard
                .tabs
                .iter()
                .find(|t| t.active)
                .map(|t| tab_label(&t.id))
                .unwrap_or_else(|| "content".to_string())
        } else {
            "content".to_string()
        }
    };

    match update {
        Some(update) => {
            let version = update.version.clone();
            let body = update.body.clone().unwrap_or_default();

            let js = format!(
                r##"
(function() {{
  if (document.getElementById('evolve-update-modal')) document.getElementById('evolve-update-modal').remove();
  var o = document.createElement('div');
  o.id = 'evolve-update-modal';
  o.style.cssText = 'position:fixed;inset:0;z-index:99999;display:flex;align-items:center;justify-content:center;background:rgba(0,0,0,0.5);font-family:system-ui,-apple-system,sans-serif;';
  o.innerHTML = '<div style="background:#1e1e2e;border:1px solid rgba(255,255,255,0.1);border-radius:16px;padding:32px;min-width:380px;max-width:460px;color:#fff;box-shadow:0 20px 60px rgba(0,0,0,0.5);position:relative;">'
    + '<div style="font-size:18px;font-weight:600;margin-bottom:6px;">Update Available</div>'
    + '<div style="font-size:13px;color:rgba(255,255,255,0.5);margin-bottom:16px;">Version {ver} is ready to install</div>'
    + '<div id="evolve-update-notes" style="background:rgba(255,255,255,0.05);border-radius:10px;padding:12px;font-size:12px;color:rgba(255,255,255,0.6);margin-bottom:16px;max-height:120px;overflow-y:auto;white-space:pre-wrap;">{notes}</div>'
    + '<div id="evolve-update-progress" style="display:none;margin-bottom:16px;">'
    + '<div style="font-size:12px;color:rgba(255,255,255,0.5);margin-bottom:6px;" id="evolve-update-status">Downloading...</div>'
    + '<div style="height:6px;background:rgba(255,255,255,0.1);border-radius:3px;overflow:hidden;"><div id="evolve-update-bar" style="height:100%;background:#3b82f6;border-radius:3px;width:0%;transition:width 0.3s;"></div></div>'
    + '</div>'
    + '<div style="display:flex;gap:10px;">'
    + '<button id="evolve-update-install" style="flex:1;padding:10px;border-radius:10px;border:none;background:#3b82f6;color:#fff;font-size:13px;font-weight:500;cursor:pointer;font-family:system-ui;">Install &amp; Restart</button>'
    + '<button id="evolve-update-later" style="flex:1;padding:10px;border-radius:10px;border:1px solid rgba(255,255,255,0.15);background:none;color:rgba(255,255,255,0.6);font-size:13px;cursor:pointer;font-family:system-ui;">Later</button>'
    + '</div></div>';
  document.body.appendChild(o);
  o.onclick = function(e) {{ if (e.target === o) o.remove(); }};
  document.getElementById('evolve-update-later').onclick = function() {{ o.remove(); }};
  document.getElementById('evolve-update-install').onclick = function() {{
    document.getElementById('evolve-update-install').disabled = true;
    document.getElementById('evolve-update-install').textContent = 'Downloading...';
    document.getElementById('evolve-update-install').style.opacity = '0.6';
    document.getElementById('evolve-update-later').style.display = 'none';
    document.getElementById('evolve-update-progress').style.display = 'block';
    window.__TAURI_INTERNALS__.invoke('install_update');
  }};
}})();
"##,
                ver = version,
                notes = body
                    .replace('\\', "\\\\")
                    .replace('\'', "\\'")
                    .replace('\n', "\\n")
                    .replace('"', "&quot;")
            );
            run_js(&app, &active_label, &js);

            app.manage(PendingUpdate(std::sync::Mutex::new(Some(update))));
        }
        None => {
            run_js(
                &app,
                &active_label,
                r##"
(function() {
  var t = document.createElement('div');
  t.style.cssText = 'position:fixed;top:20px;right:20px;z-index:99999;background:#1e1e2e;border:1px solid rgba(255,255,255,0.1);border-radius:10px;padding:12px 20px;color:#fff;font-size:13px;font-family:system-ui;box-shadow:0 8px 24px rgba(0,0,0,0.4);';
  t.textContent = 'You are on the latest version!';
  document.body.appendChild(t);
  setTimeout(function() { t.remove(); }, 3000);
})();
"##,
            );
        }
    }

    Ok(())
}

struct PendingUpdate(std::sync::Mutex<Option<tauri_plugin_updater::Update>>);

/// Called when user clicks "Install & Restart"
#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    let update = {
        let state = app
            .try_state::<PendingUpdate>()
            .ok_or("No pending update")?;
        let mut guard = state.0.lock().map_err(|e| e.to_string())?;
        guard.take().ok_or("Update already consumed")?
    };

    // Find active tab label for progress updates
    let active_label = {
        let state = app.try_state::<AppTabs>();
        if let Some(state) = state {
            let guard = state.0.lock().unwrap_or_else(|e| e.into_inner());
            guard
                .tabs
                .iter()
                .find(|t| t.active)
                .map(|t| tab_label(&t.id))
                .unwrap_or_else(|| "content".to_string())
        } else {
            "content".to_string()
        }
    };

    let app2 = app.clone();
    let app3 = app.clone();
    let label2 = active_label.clone();
    let label3 = active_label.clone();
    let mut downloaded: u64 = 0;

    update
        .download_and_install(
            move |chunk_len, total_len| {
                downloaded += chunk_len as u64;
                let total_str = match total_len {
                    Some(t) if t > 0 => format!(" / {:.1} MB", t as f64 / 1_048_576.0),
                    _ => String::new(),
                };
                let bar_width = match total_len {
                    Some(t) if t > 0 => format!("{}%", (downloaded * 100) / t),
                    _ => "50%".into(),
                };
                let js = format!(
                    "document.getElementById('evolve-update-status').textContent='Downloading... {:.1} MB{}';document.getElementById('evolve-update-bar').style.width='{}';",
                    downloaded as f64 / 1_048_576.0,
                    total_str,
                    bar_width
                );
                run_js(&app2, &label2, &js);
            },
            move || {
                run_js(
                    &app3,
                    &label3,
                    "document.getElementById('evolve-update-status').textContent='Installing... app will restart';document.getElementById('evolve-update-bar').style.width='100%';",
                );
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    app.restart();
}

/// Called from content webview JS to relay tabs data to sidebar
#[tauri::command]
async fn relay_tabs_to_sidebar(
    app: tauri::AppHandle,
    tabs_json: String,
) -> Result<(), String> {
    let _ = app.emit_to("sidebar", "tabs-loaded", &tabs_json);
    Ok(())
}

/// Called from content webview JS to relay badge data to sidebar
#[tauri::command]
async fn relay_badges_to_sidebar(
    app: tauri::AppHandle,
    badges_json: String,
) -> Result<(), String> {
    let _ = app.emit_to("sidebar", "badge-update", &badges_json);
    Ok(())
}

#[tauri::command]
async fn save_tabs_via_content(app: tauri::AppHandle, tabs_json: String) -> Result<(), String> {
    // Find active tab to run fetch through
    let active_label = {
        let state = app.try_state::<AppTabs>();
        if let Some(state) = state {
            let guard = state.0.lock().unwrap_or_else(|e| e.into_inner());
            guard
                .tabs
                .iter()
                .find(|t| t.active)
                .map(|t| tab_label(&t.id))
                .unwrap_or_else(|| "content".to_string())
        } else {
            "content".to_string()
        }
    };

    let js = format!(
        "fetch('/api/v1/desktop/sidebar/user-overrides',{{method:'POST',headers:{{'Content-Type':'application/json'}},credentials:'include',body:JSON.stringify({{overrides:{}.map(function(t,i){{return{{tab_id:t.id,hidden:!t.enabled,sort_order:i}}}})}})}}).catch(function(){{}});",
        tabs_json
    );
    run_js(&app, &active_label, &js);
    Ok(())
}

// =====================================================================
//  MAIN
// =====================================================================

fn main() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("evolve", |_ctx, request| {
            let path = request.uri().path().trim_start_matches('/');
            let assets_dir = if cfg!(debug_assertions) {
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
            } else {
                let exe = std::env::current_exe().unwrap_or_default();
                #[cfg(target_os = "macos")]
                let base = exe
                    .parent()
                    .unwrap_or(&exe)
                    .parent()
                    .unwrap_or(&exe)
                    .join("Resources")
                    .join("assets");
                #[cfg(not(target_os = "macos"))]
                let base = exe.parent().unwrap_or(&exe).join("assets");
                base
            };
            let file_path = assets_dir.join(path);
            match fs::read(&file_path) {
                Ok(data) => {
                    let mime = if path.ends_with(".html") {
                        "text/html"
                    } else if path.ends_with(".js") {
                        "application/javascript"
                    } else if path.ends_with(".css") {
                        "text/css"
                    } else if path.ends_with(".svg") {
                        "image/svg+xml"
                    } else {
                        "application/octet-stream"
                    };
                    tauri::http::Response::builder()
                        .header("Content-Type", mime)
                        .body(data)
                        .unwrap()
                }
                Err(_) => tauri::http::Response::builder()
                    .status(404)
                    .body(b"Not Found".to_vec())
                    .unwrap(),
            }
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            get_app_version,
            save_cached_tabs,
            load_cached_tabs,
            navigate_content,
            navigate_or_switch_tab,
            get_content_url,
            content_go_back,
            content_reload,
            toggle_sidebar_config,
            show_info_modal,
            check_for_updates,
            install_update,
            save_tabs_via_content,
            relay_tabs_to_sidebar,
            relay_badges_to_sidebar,
            create_tab,
            close_tab,
            switch_tab,
            update_tab_title,
            get_tabs,
            reorder_tabs,
            get_setting,
            set_setting,
            get_all_settings,
            set_startup_tabs,
        ])
        .setup(|app| {
            // Manage tab state
            app.manage(AppTabs::new());

            // Create a bare Window
            let window = tauri::window::WindowBuilder::new(app, "main")
                .title("EvolveApp")
                .inner_size(1400.0, 900.0)
                .min_inner_size(1024.0, 600.0)
                .center()
                .visible(false)
                .build()?;

            let scale = window.scale_factor().unwrap_or(1.0);
            let phys = window.inner_size()?;
            let w = phys.width as f64 / scale;
            let h = phys.height as f64 / scale;

            // --- Sidebar webview (left column, full height) ---
            let sidebar_builder = tauri::webview::WebviewBuilder::new(
                "sidebar",
                WebviewUrl::CustomProtocol(
                    "evolve://localhost/sidebar.html".parse().unwrap(),
                ),
            );

            let _sidebar = window.add_child(
                sidebar_builder,
                tauri::LogicalPosition::new(0.0, 0.0),
                tauri::LogicalSize::new(SIDEBAR_WIDTH, h),
            )?;

            // --- Tab bar webview (top of right column, 30px) ---
            let tabbar_builder = tauri::webview::WebviewBuilder::new(
                "tabbar",
                WebviewUrl::CustomProtocol(
                    "evolve://localhost/tabbar.html".parse().unwrap(),
                ),
            );

            let _tabbar = window.add_child(
                tabbar_builder,
                tauri::LogicalPosition::new(SIDEBAR_WIDTH, 0.0),
                tauri::LogicalSize::new(w - SIDEBAR_WIDTH, TABBAR_HEIGHT),
            )?;

            // --- Create initial tab(s) ---
            // Try to restore saved tabs, otherwise create a single dashboard tab
            let app_handle = app.handle().clone();
            let saved_tabs = load_tab_state(&app_handle);

            let initial_tabs: Vec<(String, String)> = if let Some(ref saved) = saved_tabs {
                if saved.is_empty() {
                    // First launch — open default startup tabs
                    DEFAULT_STARTUP_TABS
                        .iter()
                        .enumerate()
                        .map(|(i, (_name, path))| ((i + 1).to_string(), format!("{}{}", APP_URL, path)))
                        .collect()
                } else {
                    saved
                        .iter()
                        .map(|t| (t.id.clone(), t.url.clone()))
                        .collect()
                }
            } else {
                // No saved state — use defaults
                DEFAULT_STARTUP_TABS
                    .iter()
                    .enumerate()
                    .map(|(i, (_name, path))| ((i + 1).to_string(), format!("{}{}", APP_URL, path)))
                    .collect()
            };

            // Determine which tab was active (or default to first)
            let active_id = saved_tabs
                .as_ref()
                .and_then(|tabs| tabs.iter().find(|t| t.active).map(|t| t.id.clone()))
                .unwrap_or_else(|| initial_tabs[0].0.clone());

            // Compute next_id from saved tabs
            let max_id = initial_tabs
                .iter()
                .filter_map(|(id, _)| id.parse::<u32>().ok())
                .max()
                .unwrap_or(0);

            {
                let state = app.state::<AppTabs>();
                let mut guard = state.0.lock().unwrap();
                guard.next_id = max_id + 1;

                for (id, url) in &initial_tabs {
                    let title = saved_tabs
                        .as_ref()
                        .and_then(|tabs| {
                            tabs.iter()
                                .find(|t| &t.id == id)
                                .map(|t| t.title.clone())
                        })
                        .unwrap_or_else(|| "New Tab".into());

                    guard.tabs.push(TabInfo {
                        id: id.clone(),
                        url: url.clone(),
                        title,
                        active: *id == active_id,
                    });
                }
            }

            // Create webviews for each tab
            for (id, url) in &initial_tabs {
                let is_active = *id == active_id;
                if let Err(e) = create_tab_webview(&app_handle, id, url, is_active) {
                    eprintln!("[tab] Failed to create tab {}: {}", id, e);
                }
            }

            // Emit initial tab state to tabbar after a brief delay
            {
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    if let Some(state) = handle.try_state::<AppTabs>() {
                        if let Ok(guard) = state.0.lock() {
                            emit_tabs_to_tabbar(&handle, &guard.tabs);
                        }
                    }
                });
            }

            // --- Window resize handler ---
            let app_handle2 = app.handle().clone();
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Resized(phys) = event {
                    let scale = window_clone.scale_factor().unwrap_or(1.0);
                    let w = phys.width as f64 / scale;
                    let h = phys.height as f64 / scale;
                    let sw = SIDEBAR_WIDTH;

                    // Sidebar
                    if let Some(sb) = app_handle2.get_webview("sidebar") {
                        let _ = sb.set_size(tauri::LogicalSize::new(sw, h));
                    }

                    // Tab bar
                    if let Some(tb) = app_handle2.get_webview("tabbar") {
                        let _ = tb.set_position(tauri::LogicalPosition::new(sw, 0.0));
                        let _ = tb.set_size(tauri::LogicalSize::new(
                            (w - sw).max(100.0),
                            TABBAR_HEIGHT,
                        ));
                    }

                    // All content tab webviews
                    if let Some(state) = app_handle2.try_state::<AppTabs>() {
                        if let Ok(guard) = state.0.lock() {
                            let content_w = (w - sw).max(100.0);
                            let content_h = (h - TABBAR_HEIGHT).max(100.0);
                            for tab in &guard.tabs {
                                let label = tab_label(&tab.id);
                                if let Some(wv) = app_handle2.get_webview(&label) {
                                    if tab.active {
                                        let _ = wv.set_position(
                                            tauri::LogicalPosition::new(sw, TABBAR_HEIGHT),
                                        );
                                        let _ = wv.set_size(tauri::LogicalSize::new(
                                            content_w, content_h,
                                        ));
                                    }
                                    // Hidden tabs stay off-screen, no resize needed
                                }
                            }
                        }
                    }
                }
            });

            // --- Keyboard shortcuts via JS injection into tabbar ---
            // We inject a keydown listener into the tabbar that forwards to Rust.
            // For content webviews, we inject shortcuts in on_page_load.
            {
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(800));
                    // Inject keyboard shortcut listener into tabbar
                    run_js(&handle, "tabbar", r#"
                        document.addEventListener('keydown', function(e) {
                            if (!window.__TAURI_INTERNALS__) return;
                            var ctrl = e.ctrlKey || e.metaKey;
                            if (ctrl && e.key === 't') { e.preventDefault(); window.__TAURI_INTERNALS__.invoke('create_tab', {url:''}); }
                            if (ctrl && e.key === 'w') { e.preventDefault(); window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {event:'tab-shortcut',payload:'close'}); }
                            if (ctrl && !e.shiftKey && e.key === 'Tab') { e.preventDefault(); window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {event:'tab-shortcut',payload:'next'}); }
                            if (ctrl && e.shiftKey && e.key === 'Tab') { e.preventDefault(); window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {event:'tab-shortcut',payload:'prev'}); }
                            if (ctrl && e.key >= '1' && e.key <= '9') { e.preventDefault(); window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {event:'tab-shortcut',payload:'switch-'+e.key}); }
                        });
                    "#);
                });
            }

            // --- Deep link handler ---
            #[cfg(desktop)]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                let handle = app.handle().clone();
                app.deep_link().on_open_url(move |event| {
                    for url in event.urls() {
                        let url_str = url.as_str();

                        // Handle evolveapp:// deep links
                        let full_url = if let Some(path) = url_str.strip_prefix("evolveapp://") {
                            let path = if path.starts_with('/') { path.to_string() } else { format!("/{}", path) };
                            Some(format!("{}{}", APP_URL, path))
                        }
                        // Handle mailto: links — open compose modal
                        else if let Some(mailto) = url_str.strip_prefix("mailto:") {
                            let email = mailto.split('?').next().unwrap_or(mailto);
                            Some(format!("{}/email-manager?compose=true&to={}", APP_URL, email))
                        }
                        else {
                            None
                        };

                        if let Some(target_url) = full_url {
                            // Navigate active tab
                            if let Some(state) = handle.try_state::<AppTabs>() {
                                if let Ok(guard) = state.0.lock() {
                                    if let Some(active) = guard.tabs.iter().find(|t| t.active) {
                                        let label = tab_label(&active.id);
                                        nav_tab(&handle, &label, &target_url);
                                    }
                                }
                            }
                            if let Some(win) = handle.get_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                    }
                });
            }

            // --- Auto-update check (5s after startup) ---
            {
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    tauri::async_runtime::block_on(async {
                        let updater = match handle.updater() {
                            Ok(u) => u,
                            Err(_) => return,
                        };
                        match updater.check().await {
                            Ok(Some(update)) => {
                                let version = update.version.clone();
                                let body = update.body.clone().unwrap_or_default();

                                // Update sidebar version label
                                let sidebar_js = format!(
                                    "document.getElementById('version-label').textContent='v{} \u{2B06}\u{FE0F}';document.getElementById('btn-info').title='Update available: v{}';",
                                    version, version
                                );
                                run_js(&handle, "sidebar", &sidebar_js);

                                // Find active tab label
                                let active_label = {
                                    if let Some(state) = handle.try_state::<AppTabs>() {
                                        if let Ok(guard) = state.0.lock() {
                                            guard.tabs.iter().find(|t| t.active)
                                                .map(|t| tab_label(&t.id))
                                                .unwrap_or_else(|| "content".to_string())
                                        } else {
                                            "content".to_string()
                                        }
                                    } else {
                                        "content".to_string()
                                    }
                                };

                                let modal_js = format!(
                                    r##"
(function() {{
  if (document.getElementById('evolve-update-modal')) document.getElementById('evolve-update-modal').remove();
  var o = document.createElement('div');
  o.id = 'evolve-update-modal';
  o.style.cssText = 'position:fixed;inset:0;z-index:99999;display:flex;align-items:center;justify-content:center;background:rgba(0,0,0,0.5);font-family:system-ui,-apple-system,sans-serif;';
  o.innerHTML = '<div style="background:#1e1e2e;border:1px solid rgba(255,255,255,0.1);border-radius:16px;padding:32px;min-width:380px;max-width:460px;color:#fff;box-shadow:0 20px 60px rgba(0,0,0,0.5);position:relative;">'
    + '<div style="font-size:18px;font-weight:600;margin-bottom:6px;">Update Available</div>'
    + '<div style="font-size:13px;color:rgba(255,255,255,0.5);margin-bottom:16px;">Version {ver} is ready to install</div>'
    + '<div id="evolve-update-notes" style="background:rgba(255,255,255,0.05);border-radius:10px;padding:12px;font-size:12px;color:rgba(255,255,255,0.6);margin-bottom:16px;max-height:120px;overflow-y:auto;white-space:pre-wrap;">{notes}</div>'
    + '<div id="evolve-update-progress" style="display:none;margin-bottom:16px;">'
    + '<div style="font-size:12px;color:rgba(255,255,255,0.5);margin-bottom:6px;" id="evolve-update-status">Downloading...</div>'
    + '<div style="height:6px;background:rgba(255,255,255,0.1);border-radius:3px;overflow:hidden;"><div id="evolve-update-bar" style="height:100%;background:#3b82f6;border-radius:3px;width:0%;transition:width 0.3s;"></div></div>'
    + '</div>'
    + '<div style="display:flex;gap:10px;">'
    + '<button id="evolve-update-install" style="flex:1;padding:10px;border-radius:10px;border:none;background:#3b82f6;color:#fff;font-size:13px;font-weight:500;cursor:pointer;font-family:system-ui;">Install &amp; Restart</button>'
    + '<button id="evolve-update-later" style="flex:1;padding:10px;border-radius:10px;border:1px solid rgba(255,255,255,0.15);background:none;color:rgba(255,255,255,0.6);font-size:13px;cursor:pointer;font-family:system-ui;">Later</button>'
    + '</div></div>';
  document.body.appendChild(o);
  o.onclick = function(e) {{ if (e.target === o) o.remove(); }};
  document.getElementById('evolve-update-later').onclick = function() {{ o.remove(); }};
  document.getElementById('evolve-update-install').onclick = function() {{
    document.getElementById('evolve-update-install').disabled = true;
    document.getElementById('evolve-update-install').textContent = 'Downloading...';
    document.getElementById('evolve-update-install').style.opacity = '0.6';
    document.getElementById('evolve-update-later').style.display = 'none';
    document.getElementById('evolve-update-progress').style.display = 'block';
    window.__TAURI_INTERNALS__.invoke('install_update');
  }};
}})();
"##,
                                    ver = version,
                                    notes = body
                                        .replace('\\', "\\\\")
                                        .replace('\'', "\\'")
                                        .replace('\n', "\\n")
                                        .replace('"', "&quot;")
                                );
                                run_js(&handle, &active_label, &modal_js);

                                handle.manage(PendingUpdate(std::sync::Mutex::new(Some(
                                    update,
                                ))));
                            }
                            _ => {}
                        }
                    });
                });
            }

            // --- System tray ---
            let tray_handle = app.handle().clone();
            let menu = Menu::with_items(
                app,
                &[
                    &MenuItem::with_id(app, "email", "Email", true, None::<&str>)?,
                    &MenuItem::with_id(app, "chat", "Team Chat", true, None::<&str>)?,
                    &MenuItem::with_id(app, "docs", "Evolve Docs", true, None::<&str>)?,
                    &MenuItem::with_id(app, "va", "VA Assistant", true, None::<&str>)?,
                    &MenuItem::with_id(app, "sep1", "---", false, None::<&str>)?,
                    &MenuItem::with_id(app, "dashboard", "Dashboard", true, None::<&str>)?,
                    &MenuItem::with_id(app, "crm", "CRM Contacts", true, None::<&str>)?,
                    &MenuItem::with_id(app, "calendar", "Calendar", true, None::<&str>)?,
                    &MenuItem::with_id(app, "books", "Books", true, None::<&str>)?,
                    &MenuItem::with_id(app, "sep2", "---", false, None::<&str>)?,
                    &MenuItem::with_id(app, "quit", "Quit EvolveApp", true, None::<&str>)?,
                ],
            )?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("EvolveApp")
                .on_menu_event(move |app, event| {
                    let path = match event.id.as_ref() {
                        "email" => "/email",
                        "chat" => "/chat",
                        "docs" => "/evolve-docs",
                        "va" => "/workspace/va",
                        "dashboard" => "/dashboard",
                        "crm" => "/crm-marketing/contacts",
                        "calendar" => "/scheduling",
                        "books" => "/books",
                        "quit" => {
                            app.exit(0);
                            return;
                        }
                        _ => return,
                    };
                    let full_url = format!("{}{}", APP_URL, path);
                    // Navigate active tab
                    if let Some(state) = app.try_state::<AppTabs>() {
                        if let Ok(guard) = state.0.lock() {
                            if let Some(active) = guard.tabs.iter().find(|t| t.active) {
                                let label = tab_label(&active.id);
                                nav_tab(app, &label, &full_url);
                            }
                        }
                    }
                    if let Some(win) = app.get_window("main") {
                        let _ = win.show();
                        let _ = win.set_focus();
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if let Some(win) = tray.app_handle().get_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running EvolveApp");
}
