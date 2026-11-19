//! 2.4.31: the number on the app icon (chat + mail) is wired to both unread sources, behind its own switch in src/hosts.rs. std-only.
use std::fs;

fn main_rs() -> String {
    fs::read_to_string("src/main.rs").unwrap()
}

#[test]
fn both_unread_sources_feed_the_icon_number() {
    let src = main_rs();
    let badges = &src[src.find("async fn relay_badges_to_sidebar").unwrap()..src.find("static CHAT_UNREAD").unwrap()];
    assert!(badges.contains("MAIL_UNREAD.store(") && badges.contains("CHAT_UNREAD.store("), "the poll answer sets mail and chat");
    assert!(badges.contains("refresh_tray_total(&app);"), "and refreshes the icon");
    let chat = &src[src.find("async fn relay_chat_unread").unwrap()..];
    let chat = &chat[..chat.find("\n}\n").unwrap()];
    assert!(chat.contains("CHAT_UNREAD.store(") && chat.contains("refresh_tray_total(&app);"), "the page message sets chat and refreshes the icon");
}

#[test]
fn the_icon_shows_the_sum_through_the_ported_badge_code_and_only_while_its_switch_is_on() {
    let src = main_rs();
    let f = &src[src.find("fn refresh_tray_total").unwrap()..];
    let f = &f[..f.find("\n}\n").unwrap()];
    assert!(f.starts_with("fn refresh_tray_total(app: &tauri::AppHandle) {\n    if !hosts::TRAY_NUMBER {\n        return;\n    }"), "its own switch is the first thing it checks");
    assert!(f.contains("tray_total::total(") && f.contains("badge::badge_value(total)"));
    assert!(f.contains("set_overlay_icon") && f.contains("target_os = \"windows\""), "Windows overlay");
    assert!(src.contains("mod badge;") && src.contains("mod tray_total;"));
}

#[test]
fn the_icon_number_is_chat_plus_mail_in_one_place() {
    let t = fs::read_to_string("src/tray_total.rs").unwrap();
    assert!(t.contains("clamp(chat.max(0)) + clamp(mail.max(0))"));
}

#[test]
fn the_poll_stays_at_60_seconds() {
    assert!(fs::read_to_string("src/poll.rs").unwrap().contains("pub const BASE_SECS: u64 = 60;"));
}
