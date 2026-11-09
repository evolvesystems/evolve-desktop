//! Page timing wiring (slow-Lab investigation). std-only, like tests/acl.rs.
use std::fs;

fn main_rs() -> String {
    fs::read_to_string("src/main.rs").unwrap()
}

#[test]
fn the_clock_starts_first_in_main_and_the_window_open_time_is_recorded() {
    let src = main_rs();
    let m = src.find("fn main() {").unwrap();
    assert!(src[m..m + 120].contains("since_launch_ms()"), "the launch clock starts before anything else");
    assert!(src.contains("TIMING.window_open(since_launch_ms())"));
    assert!(src.contains("TIMING.page_started(since_launch_ms())"));
    assert!(src.contains("TIMING.page_ready(since_launch_ms())"));
}

#[test]
fn the_requests_run_after_a_delay_and_the_tabs_call_is_skipped_when_fresh() {
    let src = main_rs();
    assert!(src.contains("TIMING.tabs_due(poll::now_secs())"));
    assert!(src.contains("if (__WANT_TABS__) {"));
    assert!(src.contains("}, __DELAY_MS__);"), "wrapped in a timeout");
    assert!(src.contains("replace(\"__DELAY_MS__\", &timing::INJECT_DELAY_MS.to_string())"));
    assert!(src.contains("TIMING.tabs_answered(poll::now_secs());"));
    // the old immediate injection is gone
    assert!(!src.contains("var d2 = await window.__evolveTimedFetch('check-notifications', '/api/v1/desktop/check-notifications');\n                            if (d2) window.__TAURI_INTERNALS__.invoke('relay_badges_to_sidebar', {badgesJson: JSON.stringify(d2)});\n                        })();"));
}

#[test]
fn log_page_timing_is_origin_guarded_logs_through_the_state_and_is_registered() {
    let src = main_rs();
    let f = src.find("async fn log_page_timing").expect("command");
    let body = &src[f..f + 420];
    assert!(body.contains("offline_cache::from_app_origin(&webview)"));
    assert!(body.contains("diag::log(&TIMING.finish(ms, lab))"));
    assert!(src.contains("            log_page_timing,"), "listed in generate_handler!");
}

#[test]
fn a_page_after_a_park_is_flagged() {
    let src = main_rs();
    let f = src.find("content restored (window shown)").unwrap();
    assert!(src[f..f + 120].contains("TIMING.mark_after_park()"));
}

#[test]
fn nothing_but_times_and_yes_no_is_logged() {
    let t = fs::read_to_string("src/timing.rs").unwrap();
    let l = t.find("pub fn line(").unwrap();
    let end = t[l..].find("\n}\n").unwrap();
    let body = &t[l..l + end];
    assert!(body.contains("timing launch_to_window={}ms load={}ms requests={}ms first={} after_park={} lab={}"));
    assert!(!body.contains("url") && !body.contains("path"));
}
