//! The poll fix (task 5395) must stay wired: no request to the two badge endpoints goes out without the timeout helper, the timer asks poll::State first,
//! and the helper's abort time equals poll::TIMEOUT_MS. std-only, like tests/acl.rs.
use std::fs;

fn main_rs() -> String {
    fs::read_to_string("src/main.rs").unwrap()
}

#[test]
fn no_badge_or_tabs_request_bypasses_the_timeout_helper() {
    let src = main_rs();
    for raw in ["fetch('/api/v1/desktop/check-notifications'", "fetch('/api/v1/desktop/sidebar/tabs'"] {
        assert!(!src.contains(raw), "{raw} must go through window.__evolveTimedFetch");
    }
    assert!(src.contains("__evolveTimedFetch('check-notifications', '/api/v1/desktop/check-notifications')"));
    assert!(src.contains("__evolveTimedFetch('sidebar-tabs', '/api/v1/desktop/sidebar/tabs')"));
}

#[test]
fn the_helper_aborts_after_the_same_time_poll_rs_says_and_reports_every_result() {
    let src = main_rs();
    let poll = fs::read_to_string("src/poll.rs").unwrap();
    let rust_ms: u64 = poll.lines().find(|l| l.contains("pub const TIMEOUT_MS")).unwrap().split('=').nth(1).unwrap().trim().trim_end_matches(';').replace('_', "").parse().unwrap();
    assert!(src.contains("new AbortController()"));
    assert!(src.contains(&format!("ctl.abort(); }}, {rust_ms})")), "the JS timeout must be {rust_ms} ms");
    assert!(src.contains("invoke('log_poll'"), "every result is reported");
}

#[test]
fn the_timer_thread_asks_the_state_before_polling_and_uses_the_base_interval() {
    let src = main_rs();
    assert!(src.contains("POLL.try_start(poll::now_secs())"));
    assert!(src.contains("Duration::from_secs(poll::BASE_SECS)"));
    let tick = src.find("POLL_TICK_JS)").expect("the tick script is run");
    let guard = src.find("POLL.try_start").unwrap();
    assert!(guard < tick, "the guard comes before the poll");
}

#[test]
fn log_poll_refuses_other_origins_and_unknown_values_and_is_a_registered_command() {
    let src = main_rs();
    assert!(src.contains("offline_cache::from_app_origin(&webview)"));
    assert!(src.contains("poll::sanitize_endpoint(&endpoint)") && src.contains("poll::sanitize_outcome(&outcome)"));
    assert!(src.contains("            log_poll,"), "listed in generate_handler!");
}
