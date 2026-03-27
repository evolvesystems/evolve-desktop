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
use tauri_plugin_updater::UpdaterExt;
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
    run_js(&app, "content", &js);
    Ok(())
}

/// Check for updates using tauri-plugin-updater, show progress in content webview
#[tauri::command]
async fn check_for_updates(app: tauri::AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater.check().await.map_err(|e| e.to_string())?;

    match update {
        Some(update) => {
            let version = update.version.clone();
            let body = update.body.clone().unwrap_or_default();

            // Show update-available modal in content webview
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
                notes = body.replace('\\', "\\\\").replace('\'', "\\'").replace('\n', "\\n").replace('"', "&quot;")
            );
            run_js(&app, "content", &js);

            // Store the update in app state for install_update to use
            app.manage(PendingUpdate(std::sync::Mutex::new(Some(update))));
        }
        None => {
            // No update available — show brief toast
            run_js(&app, "content", r##"
(function() {
  var t = document.createElement('div');
  t.style.cssText = 'position:fixed;top:20px;right:20px;z-index:99999;background:#1e1e2e;border:1px solid rgba(255,255,255,0.1);border-radius:10px;padding:12px 20px;color:#fff;font-size:13px;font-family:system-ui;box-shadow:0 8px 24px rgba(0,0,0,0.4);';
  t.textContent = 'You are on the latest version!';
  document.body.appendChild(t);
  setTimeout(function() { t.remove(); }, 3000);
})();
"##);
        }
    }

    Ok(())
}

struct PendingUpdate(std::sync::Mutex<Option<tauri_plugin_updater::Update>>);

/// Called when user clicks "Install & Restart" — downloads and installs the update
#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    let update = {
        let state = app.try_state::<PendingUpdate>()
            .ok_or("No pending update")?;
        let mut guard = state.0.lock().map_err(|e| e.to_string())?;
        guard.take().ok_or("Update already consumed")?
    };

    let app2 = app.clone();
    let app3 = app.clone();
    let mut downloaded: u64 = 0;

    update.download_and_install(
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
                downloaded as f64 / 1_048_576.0, total_str, bar_width
            );
            run_js(&app2, "content", &js);
        },
        move || {
            run_js(&app3, "content",
                "document.getElementById('evolve-update-status').textContent='Installing... app will restart';document.getElementById('evolve-update-bar').style.width='100%';"
            );
        },
    ).await.map_err(|e| e.to_string())?;

    // The app should restart automatically after install.
    // If it doesn't (some platforms), force a relaunch:
    app.restart();
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
        .register_uri_scheme_protocol("evolve", |_ctx, request| {
            let path = request.uri().path().trim_start_matches('/');
            // In dev: read from src-tauri/assets/; in prod: same dir is bundled as resources
            let assets_dir = if cfg!(debug_assertions) {
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
            } else {
                // In production, assets are in the resource dir
                let exe = std::env::current_exe().unwrap_or_default();
                #[cfg(target_os = "macos")]
                let base = exe.parent().unwrap_or(&exe).parent().unwrap_or(&exe).join("Resources").join("assets");
                #[cfg(not(target_os = "macos"))]
                let base = exe.parent().unwrap_or(&exe).join("assets");
                base
            };
            let file_path = assets_dir.join(path);
            match fs::read(&file_path) {
                Ok(data) => {
                    let mime = if path.ends_with(".html") { "text/html" }
                        else if path.ends_with(".js") { "application/javascript" }
                        else if path.ends_with(".css") { "text/css" }
                        else { "application/octet-stream" };
                    tauri::http::Response::builder()
                        .header("Content-Type", mime)
                        .body(data)
                        .unwrap()
                }
                Err(_) => {
                    tauri::http::Response::builder()
                        .status(404)
                        .body(b"Not Found".to_vec())
                        .unwrap()
                }
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

            // Sidebar webview — served via custom 'evolve' protocol so IPC works
            // in both dev and production mode. (WebviewUrl::App resolves to devUrl
            // in dev mode which is a remote server, and file:// has no IPC.)
            let sidebar_builder = tauri::webview::WebviewBuilder::new(
                "sidebar",
                WebviewUrl::CustomProtocol("evolve://localhost/sidebar.html".parse().unwrap()),
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
            .background_color(tauri::window::Color(15, 15, 25, 255))
            .on_page_load(move |webview, payload| {
                let url_str = payload.url().to_string();
                match payload.event() {
                    PageLoadEvent::Started => {
                        println!("[page-load] Started: {}", url_str);
                        let _ = app_handle.emit_to("sidebar", "content-loading", true);
                        return;
                    }
                    PageLoadEvent::Finished => {
                        println!("[page-load] Finished: {}", url_str);
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

            // --- Auto-update check (5s after startup, silent) ---
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
                                let js = format!(
                                    "document.getElementById('version-label').textContent='v{} \u{2B06}\u{FE0F}';document.getElementById('btn-info').title='Update available: v{}';",
                                    version, version
                                );
                                run_js(&handle, "sidebar", &js);
                                handle.manage(PendingUpdate(std::sync::Mutex::new(Some(update))));
                            }
                            _ => {}
                        }
                    });
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
