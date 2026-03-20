// EvolveApp Desktop — Thin WebView wrapper for evolvepreneuriq.app
//
// Option A: Loads the web app directly. All features come from the web.
// Desktop extras: system tray, native notifications, auto-update.
//
// No local database, no data sync, no custom UI — just the web app
// in a native window.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[tauri::command]
async fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![get_app_version])
        .setup(|app| {
            // Build system tray menu with quick links
            let email = MenuItem::with_id(app, "email", "📧 Email", true, None::<&str>)?;
            let chat = MenuItem::with_id(app, "chat", "💬 Team Chat", true, None::<&str>)?;
            let docs = MenuItem::with_id(app, "docs", "📄 Evolve Docs", true, None::<&str>)?;
            let va = MenuItem::with_id(app, "va", "🤖 VA Assistant", true, None::<&str>)?;
            let sep1 = MenuItem::with_id(app, "sep1", "─────────────", false, None::<&str>)?;
            let dashboard = MenuItem::with_id(app, "dashboard", "📊 Dashboard", true, None::<&str>)?;
            let crm = MenuItem::with_id(app, "crm", "👥 CRM Contacts", true, None::<&str>)?;
            let calendar = MenuItem::with_id(app, "calendar", "📅 Calendar", true, None::<&str>)?;
            let books = MenuItem::with_id(app, "books", "📚 Books", true, None::<&str>)?;
            let sep2 = MenuItem::with_id(app, "sep2", "─────────────", false, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "❌ Quit EvolveApp", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[&email, &chat, &docs, &va, &sep1, &dashboard, &crm, &calendar, &books, &sep2, &quit],
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

                    if let Some(window) = app.get_webview_window("main") {
                        let url = format!("https://evolvepreneuriq.app{}", path);
                        let _ = window.eval(&format!("window.location.href = '{}'", url));
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
