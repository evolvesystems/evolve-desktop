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
    let phys = win.inner_size().map_err(|e| e.to_string())?;
    let scale = win.scale_factor().unwrap_or(1.0);
    let w = phys.width as f64 / scale;
    let h = phys.height as f64 / scale;
    let sw = if open { SIDEBAR_EXPANDED } else { SIDEBAR_WIDTH };

    if let Some(sb) = app.get_webview("sidebar") {
        let _ = sb.set_size(tauri::LogicalSize::new(sw, h));
    }
    if let Some(ct) = app.get_webview("content") {
        let _ = ct.set_position(tauri::LogicalPosition::new(sw, 0.0));
        let _ = ct.set_size(tauri::LogicalSize::new((w - sw).max(100.0), h));
    }
    Ok(())
}

/// Show info modal centered in content webview
#[tauri::command]
async fn show_info_modal(app: tauri::AppHandle) -> Result<(), String> {
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
    + '</div>';
  document.body.appendChild(overlay);
  overlay.onclick = function(e) {{ if (e.target === overlay) overlay.remove(); }};
  document.getElementById('evolve-info-close').onclick = function() {{ overlay.remove(); }};
  document.getElementById('evolve-info-close').onmouseenter = function() {{ this.style.background='rgba(255,255,255,0.15)'; this.style.color='#fff'; }};
  document.getElementById('evolve-info-close').onmouseleave = function() {{ this.style.background='rgba(255,255,255,0.08)'; this.style.color='rgba(255,255,255,0.6)'; }};
  fetch('https://evolvepreneuriq.app/api/v1/desktop/version').then(function(r){{ return r.json(); }}).then(function(d){{
    var el = document.getElementById('evolve-info-latest');
    if (el && d.version) {{ el.textContent = 'v' + d.version; }}
  }}).catch(function(){{}});
}})();
"##, ver = version);
    run_js(&app, "content", &js);
    Ok(())
}

/// Called from content webview JS to relay tabs data to sidebar
#[tauri::command]
async fn relay_tabs_to_sidebar(app: tauri::AppHandle, tabs_json: String) -> Result<(), String> {
    let _ = app.emit_to("sidebar", "tabs-loaded", &tabs_json);
    Ok(())
}

/// Called from content webview JS to relay badge data to sidebar
#[tauri::command]
async fn relay_badges_to_sidebar(app: tauri::AppHandle, badges_json: String) -> Result<(), String> {
    let _ = app.emit_to("sidebar", "badge-update", &badges_json);
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
            show_info_modal,
            save_tabs_via_content,
            relay_tabs_to_sidebar,
            relay_badges_to_sidebar,
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

            // Get logical size (physical / scale factor)
            let scale = window.scale_factor().unwrap_or(1.0);
            let phys = window.inner_size()?;
            let w = phys.width as f64 / scale;
            let h = phys.height as f64 / scale;

            // Sidebar webview — local HTML, never navigates
            let sidebar_builder = tauri::webview::WebviewBuilder::new(
                "sidebar",
                WebviewUrl::App("sidebar.html".into()),
            );

            let _sidebar = window.add_child(
                sidebar_builder,
                tauri::LogicalPosition::new(0.0, 0.0),
                tauri::LogicalSize::new(SIDEBAR_WIDTH, h),
            )?;

            // Content webview — loads the web app
            // on_page_load must be set on the builder BEFORE add_child()
            let app_handle = app.handle().clone();
            let content_builder = tauri::webview::WebviewBuilder::new(
                "content",
                WebviewUrl::External(APP_URL.parse().unwrap()),
            )
            .user_agent(&format!("EvolveApp/{} Tauri/2", env!("CARGO_PKG_VERSION")))
            .on_page_load(move |webview, payload| {
                match payload.event() {
                    PageLoadEvent::Started => {
                        // Tell sidebar navigation started (show loading bar)
                        let _ = app_handle.emit_to("sidebar", "content-loading", true);
                        return;
                    }
                    PageLoadEvent::Finished => {}
                    _ => return,
                }

                // Show window on first load
                if let Some(win) = app_handle.get_window("main") {
                    let _ = win.show();
                }

                // Emit URL to sidebar for active tab highlighting
                if let Ok(url) = webview.url() {
                    let _ = app_handle.emit_to("sidebar", "content-navigated", url.to_string());
                }

                // Tiny injection: fetch tabs + badges via Tauri commands (not plugin:event|emit)
                let _ = webview.eval(r#"
                    (async function() {
                        if (!window.__TAURI_INTERNALS__) return;
                        try {
                            var r = await fetch('/api/v1/desktop/sidebar/tabs', {credentials:'include'});
                            if (r.ok) {
                                var d = await r.json();
                                if (d.tabs) window.__TAURI_INTERNALS__.invoke('relay_tabs_to_sidebar', {tabsJson: JSON.stringify(d.tabs)});
                            }
                        } catch(e) {}
                        try {
                            var r2 = await fetch('/api/v1/desktop/check-notifications', {credentials:'include'});
                            if (r2.ok) {
                                var d2 = await r2.json();
                                window.__TAURI_INTERNALS__.invoke('relay_badges_to_sidebar', {badgesJson: JSON.stringify(d2)});
                            }
                        } catch(e) {}
                    })();
                "#);
            });

            let _content = window.add_child(
                content_builder,
                tauri::LogicalPosition::new(SIDEBAR_WIDTH, 0.0),
                tauri::LogicalSize::new(w - SIDEBAR_WIDTH, h),
            )?;

            // Window resize handler
            let app_handle2 = app.handle().clone();
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Resized(phys) = event {
                    // Convert physical pixels to logical using scale factor
                    let scale = window_clone.scale_factor().unwrap_or(1.0);
                    let w = phys.width as f64 / scale;
                    let h = phys.height as f64 / scale;
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
