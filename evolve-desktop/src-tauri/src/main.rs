// EvolveApp Desktop — WebView wrapper for evolvepreneuriq.app
//
// Loads the web app directly via the window "url" config in tauri.conf.json.
// System tray provides quick links to key modules.
// Supports: auto-start, deep linking (evolveapp://), auto-updater.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

const APP_URL: &str = "https://evolvepreneuriq.app";

#[tauri::command]
async fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Navigate the main webview window to a given path.
fn navigate_to(app: &tauri::AppHandle, path: &str) {
    if let Some(window) = app.get_webview_window("main") {
        let url = format!("{}{}", APP_URL, path);
        let _ = window.eval(&format!("window.location.href = '{}'", url));
        let _ = window.show();
        let _ = window.set_focus();
    }
}

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
        .invoke_handler(tauri::generate_handler![get_app_version])
        .setup(|app| {
            // --- Deep link handler ---
            // Register for deep link URLs (evolveapp://path)
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
                            navigate_to(&handle, &path);
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
                    navigate_to(app, path);
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

            // Inject persistent nav on every page load (works on external URLs too)
            if let Some(window) = app.get_webview_window("main") {
                window.on_page_load(move |webview, payload| {
                    let url = payload.url().to_string();
                    let is_eiq = url.contains("evolvepreneuriq.app") && !url.contains("dashboard.evolvepreneuriq.app");
                    // Only inject on non-EIQ pages (EIQ has its own sidebar)
                    if !is_eiq {
                        let _ = webview.eval(r#"
                            (function() {
                                if (document.getElementById('tauri-nav')) return;
                                var nav = document.createElement('div');
                                nav.id = 'tauri-nav';
                                nav.style.cssText = 'position:fixed;bottom:16px;left:16px;z-index:99999;display:flex;gap:6px;font-family:system-ui';
                                var btns = [
                                    {label:'\u2190', title:'Back', action:'history.back()'},
                                    {label:'\u21BB', title:'Refresh', action:'location.reload()'},
                                    {label:'\uD83C\uDFE0', title:'Home', action:'location.href="https://evolvepreneuriq.app/dashboard"'}
                                ];
                                btns.forEach(function(b) {
                                    var btn = document.createElement('button');
                                    btn.textContent = b.label;
                                    btn.title = b.title;
                                    btn.onclick = function() { eval(b.action); };
                                    btn.style.cssText = 'width:40px;height:40px;border-radius:50%;background:#333;color:#fff;border:none;font-size:16px;cursor:pointer;opacity:0.6;box-shadow:0 2px 8px rgba(0,0,0,0.3)';
                                    btn.onmouseenter = function() { this.style.opacity='1'; };
                                    btn.onmouseleave = function() { this.style.opacity='0.6'; };
                                    nav.appendChild(btn);
                                });
                                document.body.appendChild(nav);
                            })();
                        "#);
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running EvolveApp");
}
