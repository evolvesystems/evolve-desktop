//! Task 5469 wiring: the hidden content view is parked after PARK_AFTER_SECS and restored on show, the parked page never re-shows the window, a deep link cancels a pending restore,
//! the memory sampler runs once a minute, and no second badge poll exists. std-only, like tests/acl.rs.
use std::fs;

fn main_rs() -> String {
    fs::read_to_string("src/main.rs").unwrap()
}

#[test]
fn closing_the_window_marks_it_hidden_and_focusing_it_restores_the_page() {
    let src = main_rs();
    let close = src.find("tauri::WindowEvent::CloseRequested").expect("close handler");
    let body = &src[close..close + 400];
    assert!(body.contains("PARK.hidden(poll::now_secs())"), "hide is recorded");
    let focus = src.find("tauri::WindowEvent::Focused(true)").expect("focus handler");
    let fbody = &src[focus..focus + 300];
    assert!(fbody.contains("PARK.shown()") && fbody.contains("restore_parked(&app_handle2)"));
}

#[test]
fn parking_waits_for_the_timer_never_parks_a_non_page_and_skips_playing_media() {
    let src = main_rs();
    assert!(src.contains("if PARK.due(poll::now_secs()) {"));
    let f = src.find("fn park_content").expect("park_content");
    let body = &src[f..f + 1100];
    assert!(body.contains("park::is_restorable(&url)"));
    assert!(body.contains("querySelectorAll('audio,video')"), "a call or video keeps the page");
    assert!(body.contains("window.location.replace"));
    assert!(body.contains("serde_json::to_string(park::PARK_URL)"), "the target is a JSON-encoded literal");
}

#[test]
fn the_parked_page_does_not_show_the_window_or_touch_the_sidebar() {
    let src = main_rs();
    let park = src.find("if park::is_park_url(&url_str)").expect("page-load guard");
    let show = src.find("Ensure the window is visible").expect("show call");
    let nav = src.find("emit_sidebar(&app_handle, \"content-navigated\"").expect("sidebar nav event");
    assert!(park < show && park < nav, "guard comes first");
}

#[test]
fn a_deep_link_or_tray_navigation_cancels_a_pending_restore() {
    let src = main_rs();
    let f = src.find("fn nav_content").unwrap();
    assert!(src[f..f + 400].contains("PARK.forget()"));
}

#[test]
fn restore_only_happens_while_the_content_view_is_still_on_the_parked_page() {
    let src = main_rs();
    let f = src.find("fn restore_parked").unwrap();
    let body = &src[f..f + 700];
    assert!(body.contains("park::is_park_url(u.as_str())"));
    assert!(body.contains("if on_park {"));
}

#[test]
fn the_park_page_is_on_the_app_origin_so_the_badge_poll_still_runs_on_it() {
    let park = fs::read_to_string("src/park.rs").unwrap();
    assert!(park.contains("pub const PARK_URL: &str = \"https://evolvepreneuriq.app/api/v1/desktop/version\";"));
    let src = main_rs();
    assert!(src.contains("run_js(&badge_handle, \"content\", POLL_TICK_JS)"), "the one poll still runs on the content view");
}

#[test]
fn memory_is_sampled_once_a_minute_into_the_log_and_there_is_no_second_poll() {
    let src = main_rs();
    assert!(src.contains("Duration::from_secs(memlog::EVERY_SECS)"));
    assert!(src.contains("diag::log(&memlog::log_line("));
    let memlog = fs::read_to_string("src/memlog.rs").unwrap();
    assert!(memlog.contains("pub const EVERY_SECS: u64 = 60;"));
    // the live sidebar page has no poll of its own (src/sidebar.js is not loaded anywhere)
    let html = fs::read_to_string("assets/sidebar.html").unwrap();
    assert!(!html.contains("check-notifications'") && !html.contains("/api/v1/desktop/check-notifications"), "sidebar.html must not poll");
}
