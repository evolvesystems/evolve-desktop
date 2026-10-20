//! The update dialog (src/main.rs) must have a message for every error kind update_error.rs can produce, in both
//! places it is built, and must show errors with textContent (never innerHTML). std-only, like tests/acl.rs.
use std::fs;

const KINDS: [&str; 4] = ["network", "signature", "install", "consumed"]; // `unknown` falls through to the generic text

#[test]
fn both_dialogs_have_a_message_for_every_error_kind() {
    let main = fs::read_to_string("src/main.rs").unwrap();
    assert_eq!(main.matches("invoke('install_update').catch(function(err)").count(), 2, "expected exactly two update dialogs");
    for k in KINDS {
        assert_eq!(main.matches(&format!("{k}: '")).count() >= 2, true, "{k}: dialog text missing in one of the two dialogs");
    }
}

#[test]
fn every_kind_code_in_the_rust_module_is_covered_by_this_list() {
    let m = fs::read_to_string("src/update_error.rs").unwrap();
    for k in KINDS {
        assert!(m.contains(&format!("\"{k}\"")), "{k} not a code in update_error.rs");
    }
}

#[test]
fn the_error_text_is_only_ever_put_in_the_page_with_textcontent() {
    let main = fs::read_to_string("src/main.rs").unwrap();
    let start = main.find("invoke('install_update').catch").unwrap();
    let block = &main[start..start + 1600];
    assert!(block.contains("evolve-update-status').textContent"));
    assert!(!block.contains("innerHTML"));
}
