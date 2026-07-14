// EvolveApp Desktop — Dual Webview Architecture
//
// Two webviews in one window:
//   - "sidebar" (56px, left) — local sidebar.html, persists across all page loads
//   - "content" (rest of width) — loads evolvepreneuriq.app
//
// Uses Tauri 2.x multi-webview (unstable feature).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
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

// Emit an event directly to the standalone "sidebar" webview.
// emit_to("sidebar") converts to EventTarget::WebviewWindow which does NOT
// match a Webview added via window.add_child() — events are silently dropped.
// Webview::emit() targets the webview directly by its registered label.
fn emit_sidebar<S: serde::Serialize + Clone>(app: &tauri::AppHandle, event: &str, payload: S) {
    if let Some(wv) = app.get_webview("sidebar") {
        let _ = wv.emit(event, payload);
    }
}

fn nav_content(app: &tauri::AppHandle, url: &str) {
    // Use JSON encoding instead of manual escaping. serde_json::to_string
    // produces a double-quoted JS string literal with all special characters
    // properly escaped, including backslashes. Manual replace('\'', "\\'") is
    // vulnerable to backslash injection: a URL containing \' becomes \\' in
    // the JS literal, which JS parses as a backslash + end-of-string, letting
    // whatever follows execute as code. Backslashes are valid in URL fragments
    // and come from deep-link paths (user-controlled evolveapp:// scheme).
    if let Ok(json_url) = serde_json::to_string(url) {
        run_js(app, "content", &format!("window.location.href={}", json_url));
    }
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
    app.state::<SidebarExpanded>().0.store(open, std::sync::atomic::Ordering::SeqCst);
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

            // Show update-available modal in content webview.
            // version and body come from the update server — use serde_json to
            // produce safe JS string literals, then assign via textContent so
            // no server-supplied text is ever parsed as HTML (innerHTML XSS).
            let ver_json = serde_json::to_string(&version).unwrap_or_else(|_| "\"\"".to_string());
            let notes_json = serde_json::to_string(&body).unwrap_or_else(|_| "\"\"".to_string());
            let js = format!(
                r##"
(function() {{
  var _v={ver_json}; var _n={notes_json};
  if (document.getElementById('evolve-update-modal')) document.getElementById('evolve-update-modal').remove();
  var o = document.createElement('div');
  o.id = 'evolve-update-modal';
  o.style.cssText = 'position:fixed;inset:0;z-index:99999;display:flex;align-items:center;justify-content:center;background:rgba(0,0,0,0.5);font-family:system-ui,-apple-system,sans-serif;';
  o.innerHTML = '<div style="background:#1e1e2e;border:1px solid rgba(255,255,255,0.1);border-radius:16px;padding:32px;min-width:380px;max-width:460px;color:#fff;box-shadow:0 20px 60px rgba(0,0,0,0.5);position:relative;">'
    + '<div style="font-size:18px;font-weight:600;margin-bottom:6px;">Update Available</div>'
    + '<div id="evolve-update-ver-line" style="font-size:13px;color:rgba(255,255,255,0.5);margin-bottom:16px;"></div>'
    + '<div id="evolve-update-notes" style="background:rgba(255,255,255,0.05);border-radius:10px;padding:12px;font-size:12px;color:rgba(255,255,255,0.6);margin-bottom:16px;max-height:120px;overflow-y:auto;white-space:pre-wrap;"></div>'
    + '<div id="evolve-update-progress" style="display:none;margin-bottom:16px;">'
    + '<div style="font-size:12px;color:rgba(255,255,255,0.5);margin-bottom:6px;" id="evolve-update-status">Downloading...</div>'
    + '<div style="height:6px;background:rgba(255,255,255,0.1);border-radius:3px;overflow:hidden;"><div id="evolve-update-bar" style="height:100%;background:#3b82f6;border-radius:3px;width:0%;transition:width 0.3s;"></div></div>'
    + '</div>'
    + '<div style="display:flex;gap:10px;">'
    + '<button id="evolve-update-install" style="flex:1;padding:10px;border-radius:10px;border:none;background:#3b82f6;color:#fff;font-size:13px;font-weight:500;cursor:pointer;font-family:system-ui;">Install &amp; Restart</button>'
    + '<button id="evolve-update-later" style="flex:1;padding:10px;border-radius:10px;border:1px solid rgba(255,255,255,0.15);background:none;color:rgba(255,255,255,0.6);font-size:13px;cursor:pointer;font-family:system-ui;">Later</button>'
    + '</div></div>';
  document.body.appendChild(o);
  document.getElementById('evolve-update-ver-line').textContent = 'Version ' + _v + ' is ready to install';
  document.getElementById('evolve-update-notes').textContent = _n;
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
                ver_json = ver_json,
                notes_json = notes_json
            );
            run_js(&app, "content", &js);

            // Store the update in app state for install_update to use
            *app.state::<PendingUpdate>().0.lock().unwrap() = Some(update);
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
struct SidebarExpanded(std::sync::atomic::AtomicBool);

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

/// Open a URL in the system browser. Used for external links (target="_blank",
/// third-party domains) that should not navigate the content webview itself.
/// Only http / https / mailto are allowed — anything else is rejected.
#[tauri::command]
async fn open_external_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    use tauri_plugin_shell::ShellExt;
    if !url.starts_with("https://") && !url.starts_with("http://") && !url.starts_with("mailto:") {
        return Err("Only http/https/mailto URLs are supported".to_string());
    }
    app.shell().open(&url, None).map_err(|e| e.to_string())
}

/// Called from content webview JS to relay tabs data to sidebar
#[tauri::command]
async fn relay_tabs_to_sidebar(app: tauri::AppHandle, tabs_json: String) -> Result<(), String> {
    emit_sidebar(&app, "tabs-loaded", &tabs_json);
    Ok(())
}

/// Called from content webview JS to relay badge data to sidebar
#[tauri::command]
async fn relay_badges_to_sidebar(app: tauri::AppHandle, badges_json: String) -> Result<(), String> {
    emit_sidebar(&app, "badge-update", &badges_json);
    Ok(())
}

#[tauri::command]
async fn save_tabs_via_content(app: tauri::AppHandle, tabs_json: String) -> Result<(), String> {
    // Validate JSON before injecting into JS to avoid eval errors on corrupt data.
    let _: serde_json::Value = serde_json::from_str(&tabs_json)
        .map_err(|_| "Invalid tabs JSON".to_string())?;

    // Send both overrides (visibility + sort) and custom_tabs so user-created
    // tabs survive reinstall. The server accepts both keys in the same POST.
    let js = format!(r#"
(function() {{
    var t = {};
    var ov = t.map(function(x,i){{return{{tab_id:x.id,hidden:!x.enabled,sort_order:i}};}});
    var ct = t.filter(function(x){{return x.custom||x.tier==='user';}});
    fetch('/api/v1/desktop/sidebar/user-overrides',{{
        method:'POST',
        headers:{{'Content-Type':'application/json'}},
        credentials:'include',
        body:JSON.stringify({{overrides:ov,custom_tabs:ct}})
    }}).catch(function(){{}});
}})();
"#, tabs_json);
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
            let assets_canonical = assets_dir.canonicalize().unwrap_or_else(|_| assets_dir.clone());
            match file_path.canonicalize() {
                Ok(canonical) if canonical.starts_with(&assets_canonical) => {
                    match fs::read(&canonical) {
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
                }
                _ => {
                    tauri::http::Response::builder()
                        .status(403)
                        .body(b"Forbidden".to_vec())
                        .unwrap()
                }
            }
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
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
            open_external_url,
        ])
        .setup(|app| {
            // Register PendingUpdate state once; mutated in place by both update paths.
            // manage() panics on a second call for the same type, so NEVER call it again.
            app.manage(PendingUpdate(std::sync::Mutex::new(None)));
            app.manage(SidebarExpanded(std::sync::atomic::AtomicBool::new(false)));

            // Create a bare Window (not WebviewWindow) so we can add multiple webviews.
            // visible(true) so the splash screen is immediately shown — previously the
            // window was hidden for up to 5 s while the remote app loaded, making the
            // dock bounce with nothing appearing (looked like a crash).
            let window = tauri::window::WindowBuilder::new(app, "main")
                .title("EvolveApp")
                .inner_size(1400.0, 900.0)
                .min_inner_size(1024.0, 600.0)
                .center()
                .build()?;

            // Safety net: if we're still on the splash after 8 s (offline startup,
            // DNS failure, or eval error), force a navigation to the real app.
            // The window is already visible, so this only guards the infinite-splash
            // edge case — it is NOT the primary mechanism for leaving the splash.
            {
                let app_for_timeout = app.handle().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(8));
                    if let Some(wv) = app_for_timeout.get_webview("content") {
                        if let Ok(url) = wv.url() {
                            if url.to_string().contains("splash.html") {
                                if let Ok(j) = serde_json::to_string(APP_URL) {
                                    let _ = wv.eval(&format!("window.location.href={}", j));
                                }
                            }
                        }
                    }
                });
            }

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

            // Content webview — starts on splash.html (served via the evolve://
            // custom scheme). on_page_load fires when splash finishes rendering
            // and immediately navigates to APP_URL, so the user sees the splash
            // for only the brief moment it takes WKWebView to process the local HTML.
            // on_page_load must be set on the builder BEFORE add_child().
            let app_handle = app.handle().clone();
            let content_builder = tauri::webview::WebviewBuilder::new(
                "content",
                WebviewUrl::CustomProtocol("evolve://localhost/splash.html".parse().unwrap()),
            )
            .user_agent(&format!("EvolveApp/{} Tauri/2", env!("CARGO_PKG_VERSION")))
            .background_color(tauri::window::Color(15, 15, 25, 255))
            .on_page_load(move |webview, payload| {
                let url_str = payload.url().to_string();
                match payload.event() {
                    PageLoadEvent::Started => {
                        println!("[page-load] Started: {}", url_str);
                        emit_sidebar(&app_handle, "content-loading", true);
                        return;
                    }
                    PageLoadEvent::Finished => {
                        println!("[page-load] Finished: {}", url_str);
                        emit_sidebar(&app_handle, "content-loaded", true);
                    }
                    _ => return,
                }

                // Splash finished rendering — navigate to the real app.
                // Return early so the tab-fetch / interceptor injections below
                // don't run on the splash page (they'd hit wrong origins).
                if url_str.contains("splash.html") {
                    if let Ok(j) = serde_json::to_string(APP_URL) {
                        let _ = webview.eval(&format!("window.location.href={}", j));
                    }
                    return;
                }

                // Ensure the window is visible (harmless if already shown)
                if let Some(win) = app_handle.get_window("main") {
                    let _ = win.show();
                }

                // Emit URL to sidebar for active tab highlighting
                if let Ok(url) = webview.url() {
                    emit_sidebar(&app_handle, "content-navigated", url.to_string());
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

                // External-link interceptor: open target="_blank" and cross-origin
                // links in the system browser. Without this, WKWebView/WebView2
                // silently drops every target="_blank" click — the link just does nothing.
                let _ = webview.eval(r#"
                    (function() {
                        if (window.__evolve_ext_links_hooked__) return;
                        window.__evolve_ext_links_hooked__ = true;
                        document.addEventListener('click', function(e) {
                            var el = e.target;
                            while (el && el.tagName !== 'A') el = el.parentElement;
                            if (!el || !el.href) return;
                            var href = el.href;
                            try {
                                var u = new URL(href);
                                var isInternal = u.hostname === 'evolvepreneuriq.app'
                                    || u.hostname.endsWith('.evolvepreneuriq.app');
                                var isBlank = el.target === '_blank';
                                if (!isInternal || isBlank) {
                                    if (u.protocol === 'https:' || u.protocol === 'http:' || u.protocol === 'mailto:') {
                                        e.preventDefault();
                                        window.__TAURI_INTERNALS__.invoke('open_external_url', {url: href});
                                    }
                                }
                            } catch(_) {}
                        }, true);
                    })();
                "#);
            });

            let _content = window.add_child(
                content_builder,
                tauri::LogicalPosition::new(SIDEBAR_WIDTH, 0.0),
                tauri::LogicalSize::new(w - SIDEBAR_WIDTH, h),
            )?;

            // Window event handler — resize + close
            let app_handle2 = app.handle().clone();
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                match event {
                    // Close button (red X) hides the window; the process stays alive
                    // so the tray icon remains usable. Cmd+Q and tray > Quit call
                    // app.exit(0) which bypasses CloseRequested and actually quits.
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        if let Some(win) = app_handle2.get_window("main") {
                            let _ = win.hide();
                        }
                    }
                    tauri::WindowEvent::Resized(phys) => {
                        let scale = window_clone.scale_factor().unwrap_or(1.0);
                        let w = phys.width as f64 / scale;
                        let h = phys.height as f64 / scale;
                        let is_expanded = app_handle2.state::<SidebarExpanded>()
                            .0.load(std::sync::atomic::Ordering::SeqCst);
                        let sw = if is_expanded { SIDEBAR_EXPANDED } else { SIDEBAR_WIDTH };
                        if let Some(sb) = app_handle2.get_webview("sidebar") {
                            let _ = sb.set_size(tauri::LogicalSize::new(sw, h));
                        }
                        if let Some(ct) = app_handle2.get_webview("content") {
                            let _ = ct.set_position(tauri::LogicalPosition::new(sw, 0.0));
                            let _ = ct.set_size(tauri::LogicalSize::new((w - sw).max(100.0), h));
                        }
                    }
                    _ => {}
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

            // --- Auto-update check ---
            // First check 5s after startup, then every 6 hours while the app
            // runs. Without the periodic re-check, users who never quit the
            // app (= most users) would never see new versions, because Tauri's
            // updater.check() only runs when explicitly called. v2.4.7 made
            // that one-shot-at-startup pattern visible (john@evolvepreneur
            // ran v2.4.6 for weeks across multiple releases without seeing
            // a single modal — 2026-05-21).
            //
            // Per-version dedup: if we've already shown the modal for version
            // X this session, don't re-spam every 6 hours. Only re-pop when
            // the available version changes (new release) or the user has
            // explicitly dismissed-and-now-time-passed-and-newer-release.
            {
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    let mut last_shown_version: Option<String> = None;
                    let mut first_check = true;
                    loop {
                        let delay = if first_check {
                            std::time::Duration::from_secs(5)
                        } else {
                            std::time::Duration::from_secs(6 * 60 * 60) // 6 hours
                        };
                        first_check = false;
                        std::thread::sleep(delay);

                        let handle_inner = handle.clone();
                        let already_shown = last_shown_version.clone();
                        let result: Option<String> = tauri::async_runtime::block_on(async move {
                            let updater = match handle_inner.updater() {
                                Ok(u) => u,
                                Err(_) => return None,
                            };
                            match updater.check().await {
                                Ok(Some(update)) => {
                                    let version = update.version.clone();
                                    // Skip re-showing the same version's modal.
                                    if already_shown.as_deref() == Some(version.as_str()) {
                                        return None;
                                    }
                                    let body = update.body.clone().unwrap_or_default();
                                    let ver_json = serde_json::to_string(&version).unwrap_or_else(|_| "\"\"".to_string());
                                    let notes_json = serde_json::to_string(&body).unwrap_or_else(|_| "\"\"".to_string());

                                    // Sidebar version label: set innerHTML to only the static
                                    // SVG arrow, then prepend the version as a safe text node
                                    // so version string is never parsed as HTML markup.
                                    let sidebar_js = format!(
                                        "var _v={ver_json};var lbl=document.getElementById('version-label');lbl.innerHTML='<svg style=\"display:inline-block;vertical-align:middle;width:11px;height:11px;margin-left:3px;color:#ef4444\" xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"2.5\" stroke=\"currentColor\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m4.5 15.75 7.5-7.5 7.5 7.5\"/></svg>';lbl.insertBefore(document.createTextNode('v'+_v+' '),lbl.firstChild);document.getElementById('btn-info').title='Update available: v'+_v;",
                                        ver_json = ver_json
                                    );
                                    run_js(&handle_inner, "sidebar", &sidebar_js);

                                    // Show update modal in content webview.
                                    // _v and _n are JSON-encoded; textContent is used for both
                                    // after appendChild so no server text is parsed as HTML.
                                    let modal_js = format!(
                                        r##"
(function() {{
  var _v={ver_json}; var _n={notes_json};
  if (document.getElementById('evolve-update-modal')) document.getElementById('evolve-update-modal').remove();
  var o = document.createElement('div');
  o.id = 'evolve-update-modal';
  o.style.cssText = 'position:fixed;inset:0;z-index:99999;display:flex;align-items:center;justify-content:center;background:rgba(0,0,0,0.5);font-family:system-ui,-apple-system,sans-serif;';
  o.innerHTML = '<div style="background:#1e1e2e;border:1px solid rgba(255,255,255,0.1);border-radius:16px;padding:32px;min-width:380px;max-width:460px;color:#fff;box-shadow:0 20px 60px rgba(0,0,0,0.5);position:relative;">'
    + '<div style="font-size:18px;font-weight:600;margin-bottom:6px;">Update Available</div>'
    + '<div id="evolve-update-ver-line" style="font-size:13px;color:rgba(255,255,255,0.5);margin-bottom:16px;"></div>'
    + '<div id="evolve-update-notes" style="background:rgba(255,255,255,0.05);border-radius:10px;padding:12px;font-size:12px;color:rgba(255,255,255,0.6);margin-bottom:16px;max-height:120px;overflow-y:auto;white-space:pre-wrap;"></div>'
    + '<div id="evolve-update-progress" style="display:none;margin-bottom:16px;">'
    + '<div style="font-size:12px;color:rgba(255,255,255,0.5);margin-bottom:6px;" id="evolve-update-status">Downloading...</div>'
    + '<div style="height:6px;background:rgba(255,255,255,0.1);border-radius:3px;overflow:hidden;"><div id="evolve-update-bar" style="height:100%;background:#3b82f6;border-radius:3px;width:0%;transition:width 0.3s;"></div></div>'
    + '</div>'
    + '<div style="display:flex;gap:10px;">'
    + '<button id="evolve-update-install" style="flex:1;padding:10px;border-radius:10px;border:none;background:#3b82f6;color:#fff;font-size:13px;font-weight:500;cursor:pointer;font-family:system-ui;">Install &amp; Restart</button>'
    + '<button id="evolve-update-later" style="flex:1;padding:10px;border-radius:10px;border:1px solid rgba(255,255,255,0.15);background:none;color:rgba(255,255,255,0.6);font-size:13px;cursor:pointer;font-family:system-ui;">Later</button>'
    + '</div></div>';
  document.body.appendChild(o);
  document.getElementById('evolve-update-ver-line').textContent = 'Version ' + _v + ' is ready to install';
  document.getElementById('evolve-update-notes').textContent = _n;
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
                                        ver_json = ver_json,
                                        notes_json = notes_json
                                    );
                                    run_js(&handle_inner, "content", &modal_js);

                                    *handle_inner.state::<PendingUpdate>().0.lock().unwrap() = Some(update);
                                    Some(version)
                                }
                                _ => None,
                            }
                        });
                        if let Some(v) = result {
                            last_shown_version = Some(v);
                        }
                    }
                });
            }

            // --- Periodic badge polling ---
            // Badges update on every page navigation via the on_page_load injection,
            // but if the user stays on one page the counts go stale. A setInterval
            // injected into the content webview resets on every full navigation (new
            // window context), so multiple intervals stack up. Running the poll from
            // Rust gives a single stable timer regardless of navigation history.
            {
                let badge_handle = app.handle().clone();
                std::thread::spawn(move || {
                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(60));
                        run_js(&badge_handle, "content", r#"
(async function() {
    if (!window.__TAURI_INTERNALS__) return;
    try {
        var r = await fetch('/api/v1/desktop/check-notifications', {credentials:'include'});
        if (r.ok) {
            var d = await r.json();
            window.__TAURI_INTERNALS__.invoke('relay_badges_to_sidebar', {badgesJson: JSON.stringify(d)});
        }
    } catch(e) {}
})();
"#);
                    }
                });
            }

            // macOS: install native menu bar submenus.
            //
            // The first submenu MUST be the app name menu — macOS renders it in
            // the first slot (where "EvolveApp" should appear). Without it, macOS
            // puts "Edit" there, which is a HIG violation: Cmd+Q has no menu entry,
            // "Hide EvolveApp" (Cmd+H) is absent, and the menu bar looks wrong.
            //
            // The Edit submenu is required so WKWebView routes Cmd+C/V/X/Z to the
            // content webview; without it, macOS intercepts those keystrokes at the
            // OS level and they never fire DOM paste/copy events inside the webview.
            #[cfg(target_os = "macos")]
            {
                let h = app.handle().clone();
                let app_menu = Submenu::with_items(&h, "EvolveApp", true, &[
                    &PredefinedMenuItem::hide(&h, None)?,
                    &PredefinedMenuItem::hide_others(&h, None)?,
                    &PredefinedMenuItem::show_all(&h, None)?,
                    &PredefinedMenuItem::separator(&h)?,
                    &PredefinedMenuItem::quit(&h, None)?,
                ])?;
                let edit = Submenu::with_items(&h, "Edit", true, &[
                    &PredefinedMenuItem::undo(&h, None)?,
                    &PredefinedMenuItem::redo(&h, None)?,
                    &PredefinedMenuItem::separator(&h)?,
                    &PredefinedMenuItem::cut(&h, None)?,
                    &PredefinedMenuItem::copy(&h, None)?,
                    &PredefinedMenuItem::paste(&h, None)?,
                    &PredefinedMenuItem::separator(&h)?,
                    &PredefinedMenuItem::select_all(&h, None)?,
                ])?;
                app.set_menu(Menu::with_items(&h, &[&app_menu, &edit])?)?;
            }

            // --- System tray ---
            let menu = Menu::with_items(app, &[
                &MenuItem::with_id(app, "email", "Email", true, None::<&str>)?,
                &MenuItem::with_id(app, "chat", "Team Chat", true, None::<&str>)?,
                &MenuItem::with_id(app, "docs", "Evolve Docs", true, None::<&str>)?,
                &MenuItem::with_id(app, "va", "VA Assistant", true, None::<&str>)?,
                &PredefinedMenuItem::separator(app)?,
                &MenuItem::with_id(app, "dashboard", "Dashboard", true, None::<&str>)?,
                &MenuItem::with_id(app, "crm", "CRM Contacts", true, None::<&str>)?,
                &MenuItem::with_id(app, "calendar", "Calendar", true, None::<&str>)?,
                &MenuItem::with_id(app, "books", "Books", true, None::<&str>)?,
                &PredefinedMenuItem::separator(app)?,
                &MenuItem::with_id(app, "clear_session", "Clear Session & Login", true, None::<&str>)?,
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
                        "clear_session" => "/tenant/reset-session",
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
