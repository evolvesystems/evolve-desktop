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

/// JS injected on every page load:
/// 1. Immediately reserves 56px sidebar space + dark placeholder (no flash)
/// 2. Loads the full sidebar script which replaces the placeholder
const SIDEBAR_LOADER_JS: &str = r#"
(function() {
    // 1. Reserve sidebar space IMMEDIATELY — prevents layout shift
    if (!document.getElementById('desktop-sidebar-reserve')) {
        var style = document.createElement('style');
        style.id = 'desktop-sidebar-reserve';
        style.textContent = 'body{margin-left:56px !important}' +
            '#desktop-sidebar-placeholder{position:fixed;top:0;left:0;bottom:0;width:56px;' +
            'background:oklch(0.21 0.006 285.88);z-index:99998}' +
            '.navbar,.sticky,.fixed-top,[class*="sticky"]{left:56px !important;width:calc(100% - 56px) !important}';
        (document.head || document.documentElement).appendChild(style);

        var ph = document.createElement('div');
        ph.id = 'desktop-sidebar-placeholder';
        (document.body || document.documentElement).appendChild(ph);
    }

    // 2. Load full sidebar script (replaces placeholder with interactive sidebar)
    if (!document.getElementById('desktop-sidebar') && !document.getElementById('desktop-sidebar-loader')) {
        var s = document.createElement('script');
        s.id = 'desktop-sidebar-loader';
        s.src = 'https://evolvepreneuriq.app/js/desktop-sidebar.js?v=' + Date.now();
        document.head.appendChild(s);
    }
})();
"#;

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
        .on_page_load(|webview, payload| {
            // Load the unified sidebar on every page
            let _ = webview.eval(SIDEBAR_LOADER_JS);

        })
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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running EvolveApp");
}
