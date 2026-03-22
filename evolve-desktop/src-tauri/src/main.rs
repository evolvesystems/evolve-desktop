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

            // Inject sidebar on every page load (same design as EIQ sidebar)
            if let Some(window) = app.get_webview_window("main") {
                window.on_page_load(move |webview, payload| {
                    let url = payload.url().to_string();
                    // Skip injection on EIQ pages — they have their own dynamic sidebar
                    let is_eiq = url.contains("evolvepreneuriq.app")
                        && !url.contains("dashboard.evolvepreneuriq.app")
                        && !url.contains("/download");
                    if is_eiq { return; }

                    let _ = webview.eval(r#"
                    (function() {
                        if (document.getElementById('tauri-sidebar')) return;
                        var APP = 'https://evolvepreneuriq.app';
                        var tabs = [
                            {l:'Home',e:'\uD83C\uDFE0',u:APP+'/dashboard'},
                            {l:'Mail',e:'\uD83D\uDCE7',u:APP+'/email-manager'},
                            {l:'Chat',e:'\uD83D\uDCAC',u:APP+'/chat'},
                            {l:'CRM',e:'\uD83D\uDC65',u:APP+'/crm-marketing/contacts'},
                            {l:'Meet',e:'\uD83D\uDCC5',u:APP+'/evolvemeet'},
                            {l:'Docs',e:'\uD83D\uDCC4',u:APP+'/evolve-docs'}
                        ];
                        var sb = document.createElement('div');
                        sb.id = 'tauri-sidebar';
                        sb.style.cssText = 'position:fixed;top:0;left:0;bottom:0;width:56px;background:#1a1a2e;z-index:99999;display:flex;flex-direction:column;align-items:center;padding:8px 0;gap:2px;font-family:system-ui';

                        // Logo / Home
                        var logo = document.createElement('a');
                        logo.href = APP + '/dashboard';
                        logo.style.cssText = 'width:40px;height:40px;border-radius:50%;background:linear-gradient(135deg,#667eea,#764ba2);color:#fff;display:flex;align-items:center;justify-content:center;font-weight:700;font-size:15px;text-decoration:none;margin-bottom:8px';
                        logo.textContent = 'E';
                        sb.appendChild(logo);

                        tabs.forEach(function(t) {
                            var a = document.createElement('a');
                            a.href = t.u;
                            a.title = t.l;
                            a.style.cssText = 'width:44px;height:44px;display:flex;flex-direction:column;align-items:center;justify-content:center;border-radius:10px;color:rgba(255,255,255,0.5);text-decoration:none;font-size:9px;gap:2px;transition:all 0.15s';
                            a.innerHTML = '<span style="font-size:18px">'+t.e+'</span><span>'+t.l+'</span>';
                            a.onmouseenter = function(){this.style.background='rgba(255,255,255,0.1)';this.style.color='#fff'};
                            a.onmouseleave = function(){this.style.background='none';this.style.color='rgba(255,255,255,0.5)'};
                            sb.appendChild(a);
                        });

                        // Spacer
                        var sp = document.createElement('div');
                        sp.style.flex = '1';
                        sb.appendChild(sp);

                        // Back + Refresh
                        var back = document.createElement('button');
                        back.onclick = function(){history.back()};
                        back.title = 'Back';
                        back.style.cssText = 'width:36px;height:36px;border-radius:50%;background:rgba(255,255,255,0.08);color:rgba(255,255,255,0.4);border:none;font-size:14px;cursor:pointer;margin-bottom:4px';
                        back.textContent = '\u2190';
                        sb.appendChild(back);

                        var ref = document.createElement('button');
                        ref.onclick = function(){location.reload()};
                        ref.title = 'Refresh';
                        ref.style.cssText = 'width:36px;height:36px;border-radius:50%;background:rgba(255,255,255,0.08);color:rgba(255,255,255,0.4);border:none;font-size:14px;cursor:pointer;margin-bottom:8px';
                        ref.textContent = '\u21BB';
                        sb.appendChild(ref);

                        document.body.appendChild(sb);
                        document.body.style.marginLeft = '56px';
                        document.body.style.width = 'calc(100% - 56px)';
                    })();
                    "#);
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running EvolveApp");
}
