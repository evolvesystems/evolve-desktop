// EvolveApp Desktop — Dual Webview Architecture
//
// Two webviews in one window:
//   - "sidebar" (56px, left) — local sidebar.html, never navigates, persists across all page loads
//   - "content" (rest of width) — loads evolvepreneuriq.app, handles all navigation
//
// IPC commands bridge communication between the two webviews.
// The sidebar never flashes or disappears during page navigation.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::PageLoadEvent,
    Emitter, Listener, Manager, WebviewBuilder, WebviewUrl,
};
use std::fs;

const APP_URL: &str = "https://evolvepreneuriq.app";
const SIDEBAR_WIDTH: f64 = 56.0;
const SIDEBAR_EXPANDED_WIDTH: f64 = 316.0;

// =====================================================================
//  TAURI COMMANDS (IPC between sidebar and content webviews)
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

/// Navigate the content webview to a URL
#[tauri::command]
async fn navigate_content(app: tauri::AppHandle, url: String) -> Result<(), String> {
    if let Some(content) = app.get_webview("content") {
        let js = format!("window.location.href = '{}'", url.replace('\'', "\\'"));
        content.eval(&js).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Get the current URL of the content webview
#[tauri::command]
async fn get_content_url(app: tauri::AppHandle) -> Result<String, String> {
    if let Some(content) = app.get_webview("content") {
        return content.url().map(|u| u.to_string()).map_err(|e| e.to_string());
    }
    Err("Content webview not found".into())
}

/// Go back in content webview
#[tauri::command]
async fn content_go_back(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(content) = app.get_webview("content") {
        let _ = content.eval("history.back()");
    }
    Ok(())
}

/// Reload content webview
#[tauri::command]
async fn content_reload(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(content) = app.get_webview("content") {
        let _ = content.eval("location.reload()");
    }
    Ok(())
}

/// Toggle sidebar config panel — resize both webviews
#[tauri::command]
async fn toggle_sidebar_config(app: tauri::AppHandle, open: bool) -> Result<(), String> {
    let window = app.get_webview_window("main").ok_or("Window not found")?;
    let size = window.inner_size().map_err(|e| e.to_string())?;
    let sidebar_width = if open { SIDEBAR_EXPANDED_WIDTH } else { SIDEBAR_WIDTH };

    if let Some(sidebar) = app.get_webview("sidebar") {
        let _ = sidebar.set_size(tauri::LogicalSize::new(sidebar_width, size.height as f64));
    }
    if let Some(content) = app.get_webview("content") {
        let _ = content.set_position(tauri::LogicalPosition::new(sidebar_width as i32, 0));
        let _ = content.set_size(tauri::LogicalSize::new(
            (size.width as f64 - sidebar_width).max(100.0),
            size.height as f64,
        ));
    }
    Ok(())
}

/// Save tabs to server by evaluating JS in the content webview (which has auth cookies)
#[tauri::command]
async fn save_tabs_via_content(app: tauri::AppHandle, tabs_json: String) -> Result<(), String> {
    if let Some(content) = app.get_webview("content") {
        let js = format!(
            "fetch('/api/v1/desktop/sidebar/user-overrides',{{method:'POST',headers:{{'Content-Type':'application/json'}},credentials:'include',body:JSON.stringify({{overrides:{}.map(function(t,i){{return{{tab_id:t.id,hidden:!t.enabled,sort_order:i}}}})}})}}).catch(function(){{}});",
            tabs_json
        );
        let _ = content.eval(&js);
    }
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
            let window = app.get_webview_window("main").expect("main window not found");

            let size = window.inner_size()?;
            let win_width = size.width as f64;
            let win_height = size.height as f64;

            // Create sidebar webview (loads local sidebar.html)
            let sidebar_builder = WebviewBuilder::new(
                "sidebar",
                WebviewUrl::App("sidebar.html".into()),
            );

            let _sidebar = window.add_child(
                sidebar_builder,
                tauri::LogicalPosition::new(0, 0),
                tauri::LogicalSize::new(SIDEBAR_WIDTH, win_height),
            )?;

            // Create content webview (loads the web app)
            let content_builder = WebviewBuilder::new(
                "content",
                WebviewUrl::External(APP_URL.parse().unwrap()),
            )
            .user_agent(&format!("EvolveApp/{} Tauri/2", env!("CARGO_PKG_VERSION")));

            let content = window.add_child(
                content_builder,
                tauri::LogicalPosition::new(SIDEBAR_WIDTH as i32, 0),
                tauri::LogicalSize::new(win_width - SIDEBAR_WIDTH, win_height),
            )?;

            // On content page load: emit navigation event + fetch badges/tabs
            let app_handle = app.handle().clone();
            content.on_page_load(move |webview, payload| {
                if payload.event() != PageLoadEvent::Finished {
                    return;
                }

                // Show window on first load (starts hidden to avoid white flash)
                if let Some(win) = webview.window() {
                    let _ = win.show();
                }

                // Emit current URL to sidebar for active tab highlighting
                if let Ok(url) = webview.url() {
                    let _ = app_handle.emit_to("sidebar", "content-navigated", url.to_string());
                }

                // Inject tiny script to fetch tabs and badges (content has auth cookies)
                let _ = webview.eval(r#"
                    (async function() {
                        try {
                            // Fetch merged tabs
                            var r = await fetch('/api/v1/desktop/sidebar/tabs', {credentials:'include'});
                            if (r.ok) {
                                var d = await r.json();
                                if (d.tabs) window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {event:'tabs-loaded', payload:JSON.stringify(d.tabs)});
                            }
                        } catch(e) {}
                        try {
                            // Fetch badge counts
                            var r2 = await fetch('/api/v1/desktop/check-notifications', {credentials:'include'});
                            if (r2.ok) {
                                var d2 = await r2.json();
                                window.__TAURI_INTERNALS__.invoke('plugin:event|emit', {event:'badge-update', payload:JSON.stringify(d2)});
                            }
                        } catch(e) {}
                    })();
                "#);
            });

            // Handle window resize — reposition both webviews
            let app_handle2 = app.handle().clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Resized(size) = event {
                    let h = size.height as f64;
                    let w = size.width as f64;
                    // Determine current sidebar width (may be expanded for config)
                    let sw = SIDEBAR_WIDTH; // TODO: track expanded state
                    if let Some(sidebar) = app_handle2.get_webview("sidebar") {
                        let _ = sidebar.set_size(tauri::LogicalSize::new(sw, h));
                    }
                    if let Some(content) = app_handle2.get_webview("content") {
                        let _ = content.set_position(tauri::LogicalPosition::new(sw as i32, 0));
                        let _ = content.set_size(tauri::LogicalSize::new((w - sw).max(100.0), h));
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
                        let url_str = url.as_str();
                        if let Some(path) = url_str.strip_prefix("evolveapp://") {
                            let path = if path.starts_with('/') {
                                path.to_string()
                            } else {
                                format!("/{}", path)
                            };
                            let full_url = format!("{}{}", APP_URL, path);
                            if let Some(content) = handle.get_webview("content") {
                                let _ = content.eval(&format!("window.location.href = '{}'", full_url));
                            }
                            if let Some(window) = handle.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                });
            }

            // --- System tray ---
            let email = MenuItem::with_id(app, "email", "Email", true, None::<&str>)?;
            let chat = MenuItem::with_id(app, "chat", "Team Chat", true, None::<&str>)?;
            let docs = MenuItem::with_id(app, "docs", "Evolve Docs", true, None::<&str>)?;
            let va = MenuItem::with_id(app, "va", "VA Assistant", true, None::<&str>)?;
            let sep1 = MenuItem::with_id(app, "sep1", "---", false, None::<&str>)?;
            let dashboard =
                MenuItem::with_id(app, "dashboard", "Dashboard", true, None::<&str>)?;
            let crm = MenuItem::with_id(app, "crm", "CRM Contacts", true, None::<&str>)?;
            let calendar =
                MenuItem::with_id(app, "calendar", "Calendar", true, None::<&str>)?;
            let books = MenuItem::with_id(app, "books", "Books", true, None::<&str>)?;
            let sep2 = MenuItem::with_id(app, "sep2", "---", false, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit EvolveApp", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &email, &chat, &docs, &va, &sep1, &dashboard, &crm, &calendar, &books,
                    &sep2, &quit,
                ],
            )?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("EvolveApp")
                .on_menu_event(|app, event| {
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
                    let url = format!("{}{}", APP_URL, path);
                    if let Some(content) = app.get_webview("content") {
                        let _ = content.eval(&format!("window.location.href = '{}'", url));
                    }
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running EvolveApp");
}
