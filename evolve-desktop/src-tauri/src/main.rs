// EvolveApp Desktop — Dual Webview Architecture
//
// Two webviews in one window:
//   - "sidebar" (56px, left) — local sidebar.html, persists across all page loads
//   - "content" (rest of width) — loads evolvepreneuriq.app
//
// Uses Tauri 2.x multi-webview (unstable feature).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::PageLoadEvent,
    Emitter, Manager, WebviewUrl,
};
use std::fs;

const APP_URL: &str = "https://evolvepreneuriq.app";
const SIDEBAR_WIDTH: f64 = 56.0;
const SIDEBAR_EXPANDED: f64 = 316.0;

// Helper: eval JS in a named webview
fn run_js(app: &tauri::AppHandle, label: &str, js: &str) {
    if let Some(wv) = app.get_webview(label) {
        let _ = wv.eval(js);
    }
}

fn nav_content(app: &tauri::AppHandle, url: &str) {
    run_js(app, "content", &format!("window.location.href='{}'", url.replace('\'', "\\'")));
}

// =====================================================================
//  TAURI COMMANDS
// =====================================================================

#[tauri::command]
async fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
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

#[tauri::command]
async fn navigate_content(app: tauri::AppHandle, url: String) -> Result<(), String> {
    nav_content(&app, &url);
    Ok(())
}

#[tauri::command]
async fn get_content_url(app: tauri::AppHandle) -> Result<String, String> {
    app.get_webview("content")
        .ok_or_else(|| "Content webview not found".to_string())
        .and_then(|wv| wv.url().map(|u| u.to_string()).map_err(|e| e.to_string()))
}

#[tauri::command]
async fn content_go_back(app: tauri::AppHandle) -> Result<(), String> {
    run_js(&app, "content", "history.back()");
    Ok(())
}

#[tauri::command]
async fn content_reload(app: tauri::AppHandle) -> Result<(), String> {
    run_js(&app, "content", "location.reload()");
    Ok(())
}

#[tauri::command]
async fn toggle_sidebar_config(app: tauri::AppHandle, open: bool) -> Result<(), String> {
    let win = app.get_window("main").ok_or("Window not found")?;
    let size = win.inner_size().map_err(|e| e.to_string())?;
    let sw = if open { SIDEBAR_EXPANDED } else { SIDEBAR_WIDTH };

    if let Some(sb) = app.get_webview("sidebar") {
        let _ = sb.set_size(tauri::LogicalSize::new(sw, size.height as f64));
    }
    if let Some(ct) = app.get_webview("content") {
        let _ = ct.set_position(tauri::LogicalPosition::new(sw, 0.0));
        let _ = ct.set_size(tauri::LogicalSize::new((size.width as f64 - sw).max(100.0), size.height as f64));
    }
    Ok(())
}

#[tauri::command]
async fn save_tabs_via_content(app: tauri::AppHandle, tabs_json: String) -> Result<(), String> {
    let js = format!(
        "fetch('/api/v1/desktop/sidebar/user-overrides',{{method:'POST',headers:{{'Content-Type':'application/json'}},credentials:'include',body:JSON.stringify({{overrides:{}.map(function(t,i){{return{{tab_id:t.id,hidden:!t.enabled,sort_order:i}}}})}})}}).catch(function(){{}});",
        tabs_json
    );
    run_js(&app, "content", &js);
    Ok(())
}

// =====================================================================
//  MAIN
// =====================================================================

fn main() {
    tauri::Builder::default()
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
            get_content_url,
            content_go_back,
            content_reload,
            toggle_sidebar_config,
            save_tabs_via_content,
        ])
        .setup(|app| {
            // Create a bare Window (not WebviewWindow) so we can add multiple webviews
            let window = tauri::window::WindowBuilder::new(app, "main")
                .title("EvolveApp")
                .inner_size(1400.0, 900.0)
                .min_inner_size(1024.0, 600.0)
                .center()
                .visible(false)
                .build()?;

            let size = window.inner_size()?;
            let w = size.width as f64;
            let h = size.height as f64;

            // Sidebar webview — local HTML, never navigates
            let sidebar_builder = tauri::webview::WebviewBuilder::new(
                "sidebar",
                WebviewUrl::App("sidebar.html".into()),
            ).auto_resize();

            let _sidebar = window.add_child(
                sidebar_builder,
                tauri::LogicalPosition::new(0.0, 0.0),
                tauri::LogicalSize::new(SIDEBAR_WIDTH, h),
            )?;

            // Content webview — loads the web app
            let content_builder = tauri::webview::WebviewBuilder::new(
                "content",
                WebviewUrl::External(APP_URL.parse().unwrap()),
            )
            .user_agent(&format!("EvolveApp/{} Tauri/2", env!("CARGO_PKG_VERSION")));

            let content = window.add_child(
                content_builder,
                tauri::LogicalPosition::new(SIDEBAR_WIDTH, 0.0),
                tauri::LogicalSize::new(w - SIDEBAR_WIDTH, h),
            )?;

            // On content page load: emit URL + fetch badges/tabs
            let app_handle = app.handle().clone();
            content.on_page_load(move |webview: &tauri::Webview, payload: &tauri::webview::PageLoadPayload<'_>| {
                if payload.event() != PageLoadEvent::Finished {
                    return;
                }

                // Show window on first load
                if let Some(win) = app_handle.get_window("main") {
                    let _ = win.show();
                }

                // Emit URL to sidebar for active tab highlighting
                if let Ok(url) = webview.url() {
                    let _ = app_handle.emit_to("sidebar", "content-navigated", url.to_string());
                }

                // Tiny injection: fetch tabs + badges (content has auth cookies)
                let _ = webview.eval(r#"
                    (async function() {
                        try {
                            var r = await fetch('/api/v1/desktop/sidebar/tabs', {credentials:'include'});
                            if (r.ok) {
                                var d = await r.json();
                                if (d.tabs) window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {event:'tabs-loaded', payload:JSON.stringify(d.tabs)});
                            }
                        } catch(e) {}
                        try {
                            var r2 = await fetch('/api/v1/desktop/check-notifications', {credentials:'include'});
                            if (r2.ok) {
                                var d2 = await r2.json();
                                window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {event:'badge-update', payload:JSON.stringify(d2)});
                            }
                        } catch(e) {}
                    })();
                "#);
            });

            // Window resize handler
            let app_handle2 = app.handle().clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Resized(size) = event {
                    let h = size.height as f64;
                    let w = size.width as f64;
                    if let Some(sb) = app_handle2.get_webview("sidebar") {
                        let _ = sb.set_size(tauri::LogicalSize::new(SIDEBAR_WIDTH, h));
                    }
                    if let Some(ct) = app_handle2.get_webview("content") {
                        let _ = ct.set_position(tauri::LogicalPosition::new(SIDEBAR_WIDTH, 0.0));
                        let _ = ct.set_size(tauri::LogicalSize::new((w - SIDEBAR_WIDTH).max(100.0), h));
                    }
                }
            });

            // --- Deep link handler ---
            #[cfg(desktop)]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                let handle = app.handle().clone();
                app.deep_link().on_open_url(move |event| {
                    for url in event.urls() {
                        if let Some(path) = url.as_str().strip_prefix("evolveapp://") {
                            let path = if path.starts_with('/') { path.to_string() } else { format!("/{}", path) };
                            nav_content(&handle, &format!("{}{}", APP_URL, path));
                            if let Some(win) = handle.get_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                    }
                });
            }

            // --- System tray ---
            let menu = Menu::with_items(app, &[
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
            ])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("EvolveApp")
                .on_menu_event(|app, event| {
                    let path = match event.id.as_ref() {
                        "email" => "/email", "chat" => "/chat", "docs" => "/evolve-docs",
                        "va" => "/workspace/va", "dashboard" => "/dashboard",
                        "crm" => "/crm-marketing/contacts", "calendar" => "/scheduling",
                        "books" => "/books",
                        "quit" => { app.exit(0); return; }
                        _ => return,
                    };
                    nav_content(app, &format!("{}{}", APP_URL, path));
                    if let Some(win) = app.get_window("main") { let _ = win.show(); let _ = win.set_focus(); }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        if let Some(win) = tray.app_handle().get_window("main") { let _ = win.show(); let _ = win.set_focus(); }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running EvolveApp");
}
