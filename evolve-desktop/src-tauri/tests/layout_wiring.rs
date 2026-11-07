//! The webviews must follow the window's real size (the Lab filled only the top half of John's Windows window). std-only, like tests/acl.rs.
use std::fs;

fn main_rs() -> String {
    fs::read_to_string("src/main.rs").unwrap()
}

#[test]
fn every_event_that_can_change_the_size_resyncs_the_views() {
    let src = main_rs();
    assert!(src.contains("tauri::WindowEvent::Resized(_) | tauri::WindowEvent::ScaleFactorChanged { .. } => sync_layout(&app_handle2)"));
    let focus = src.find("tauri::WindowEvent::Focused(true)").unwrap();
    assert!(src[focus..focus + 200].contains("sync_layout(&app_handle2)"));
    let fin = src.find("emit_sidebar(&app_handle, \"content-loaded\", true);").unwrap();
    assert!(src[fin..fin + 120].contains("sync_layout(&app_handle)"));
}

#[test]
fn the_sync_uses_the_window_size_logs_drift_and_skips_a_minimised_window() {
    let src = main_rs();
    let f = src.find("fn sync_layout").expect("sync_layout");
    let body = &src[f..f + 1700];
    assert!(body.contains("phys.width == 0 || phys.height == 0"), "a minimised window must not shrink the views to 100 px");
    assert!(body.contains("layout drift:"), "the log says when the content view had drifted");
    assert!(body.contains("ct.set_size(tauri::LogicalSize::new((w - sw).max(100.0), h))"));
    assert!(body.contains("phys.height as f64 / scale"));
}
