//! Every command in generate_handler! must be granted to the app's windows, or a page that calls it
//! fails with "Command X not allowed by ACL" (bugs EVO-BUG-18921/18926: relay_badges_to_sidebar, 800+ hits).
//! std-only on purpose, so it also runs with `rustc --test tests/acl.rs` where the Tauri toolchain is missing.
use std::fs;

fn handler_commands() -> Vec<String> {
    let src = fs::read_to_string("src/main.rs").unwrap();
    let start = src.find("generate_handler![").expect("generate_handler! not found") + "generate_handler![".len();
    let body = &src[start..start + src[start..].find(']').unwrap()];
    body.split(',')
        .map(|s| s.trim().rsplit("::").next().unwrap().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn permission(cmd: &str) -> String {
    format!("allow-{}", cmd.replace('_', "-"))
}

#[test]
fn every_handler_command_is_listed_in_the_app_manifest() {
    let build = fs::read_to_string("build.rs").unwrap();
    assert!(build.contains("AppManifest"), "build.rs has no AppManifest");
    for c in handler_commands() {
        assert!(build.contains(&format!("\"{c}\"")), "{c} missing from build.rs AppManifest commands");
    }
}

#[test]
fn every_handler_command_is_granted_in_the_capability() {
    let cap = fs::read_to_string("capabilities/default.json").unwrap();
    for c in handler_commands() {
        assert!(cap.contains(&format!("\"{}\"", permission(&c))), "{c}: capability lacks {}", permission(&c));
    }
}

#[test]
fn the_badge_relay_in_particular_is_granted() {
    let cap = fs::read_to_string("capabilities/default.json").unwrap();
    assert!(cap.contains("\"allow-relay-badges-to-sidebar\""));
}
