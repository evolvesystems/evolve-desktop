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

#[test]
fn a_401_is_reported_as_unauthorized_not_as_a_generic_http_failure() {
    let src = main_rs();
    let h = src.find("window.__evolveTimedFetch = async function").expect("helper");
    let body = &src[h..h + 900];
    let unauth = body.find("r.status === 401").expect("401 is checked");
    let generic = body.find("outcome = 'http'").expect("generic http outcome");
    assert!(unauth < generic, "the 401 test comes before the generic non-ok test");
    assert!(body.contains("outcome = 'unauthorized'"));
}

#[test]
fn log_poll_stops_polling_on_unauthorized_and_shows_sign_in_once_never_from_a_sign_in_page() {
    let src = main_rs();
    let f = src.find("async fn log_poll").expect("log_poll");
    let body = &src[f..f + 1500];
    let un = body.find("if outcome == \"unauthorized\"").expect("unauthorized branch");
    let fin = body.find("POLL.finish(").expect("finish");
    assert!(un < fin, "the 401 branch runs before the back-off bookkeeping");
    assert!(body.contains("if POLL.sign_out() {"), "only the first 401 acts");
    assert!(body.contains("poll::is_sign_in_path(u.path())"), "no redirect loop from the sign-in page");
    let gate = body.find("if !on_sign_in {").expect("the navigation is gated on not being on a sign-in page");
    let nav = body.find("nav_content(&app").expect("navigation");
    assert!(gate < nav, "the gate comes before the navigation");
    assert!(body.contains("nav_content(&app, &format!(\"{}/login\", APP_URL))"));
    assert!(body.contains("return Ok(());"), "a 401 never counts as a failure in a row");
}
