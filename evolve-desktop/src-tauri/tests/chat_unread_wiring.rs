//! 2.4.30: the chat:unread listener (part a) and the messenger-host app pages (part b) are wired in main.rs, each behind its own switch in src/hosts.rs.
//! std-only (runs with `rustc --test` where the Tauri toolchain is missing). The script logic itself is tested by tests/chat_unread_harness.mjs under node.
use std::fs;

fn main_rs() -> String {
    fs::read_to_string("src/main.rs").unwrap()
}

#[test]
fn app_pages_come_from_the_exact_host_check_not_a_prefix_match() {
    let src = main_rs();
    assert!(src.contains("let is_app_page = hosts::is_app_page(&url_str);"));
    assert!(!src.contains("url_str.starts_with(\"https://evolvepreneuriq.app\")"), "the old prefix match also accepted look-alike hosts");
    assert!(src.contains("mod hosts;"));
}

#[test]
fn the_listener_is_injected_on_app_pages_only_and_behind_its_own_switch() {
    let src = main_rs();
    assert!(src.contains("const CHAT_UNREAD_JS: &str = include_str!(\"chat_unread.js\");"));
    let at = src.find("if hosts::PAGE_UNREAD_LISTENER {").expect("listener switch");
    assert!(src[at..at + 120].contains("webview.eval(CHAT_UNREAD_JS)"));
    let app_page = src.find("if is_app_page {").expect("app page block");
    assert!(app_page < at, "the listener is inside the app-page block");
}

#[test]
fn the_chat_count_uses_its_own_event_so_email_and_lab_badges_are_not_zeroed() {
    let src = main_rs();
    assert!(src.contains("emit_sidebar(&app, \"chat-unread\", hosts::clamp_total(total));"));
    let cmd = &src[src.find("async fn relay_chat_unread").unwrap()..];
    let body = &cmd[..cmd.find("\n}\n").unwrap()];
    assert!(!body.contains("badge-update"), "badge-update overwrites email and Lab with 0 on a partial payload");
    let side = fs::read_to_string("assets/sidebar.html").unwrap();
    let handler = &side[side.find("tauriListen('chat-unread'").expect("sidebar listener")..];
    let handler = &handler[..handler.find("});").unwrap()];
    assert!(handler.contains("updateBadge('chat', n)"));
    assert!(!handler.contains("updateBadge('email'") && !handler.contains("updateLab("), "only the chat badge moves");
}

#[test]
fn the_new_command_is_registered_manifested_and_granted() {
    assert!(main_rs().contains("            relay_chat_unread,"));
    assert!(fs::read_to_string("build.rs").unwrap().contains("\"relay_chat_unread\""));
    assert!(fs::read_to_string("capabilities/default.json").unwrap().contains("\"allow-relay-chat-unread\""));
}

#[test]
fn the_poll_stays_at_60_seconds_as_the_backstop() {
    let poll = fs::read_to_string("src/poll.rs").unwrap();
    assert!(poll.contains("pub const BASE_SECS: u64 = 60;"));
}

#[test]
fn the_script_file_and_its_node_harness_ship_together() {
    let js = fs::read_to_string("src/chat_unread.js").unwrap();
    assert!(js.contains("e.origin !== location.origin"), "same origin only");
    assert!(js.contains("'relay_chat_unread'"));
    assert!(fs::metadata("tests/chat_unread_harness.mjs").is_ok());
}
