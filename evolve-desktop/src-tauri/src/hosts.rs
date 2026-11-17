//! Which pages the shell treats as EIQ app pages (2.4.30). std-only on purpose: tested with plain `rustc --test src/hosts.rs`.
//!
//! An app page gets the tabs/badge helper, the external-link interceptor and (2.4.30) the chat:unread listener. Before 2.4.30 this was
//! `url.starts_with("https://evolvepreneuriq.app")`, which also matched look-alike hosts such as evolvepreneuriq.app.evil.test. Now the HOST must match exactly.

/// 2.4.30, part (b): pages on the messenger host (messager.evolvepreneuriq.app) count as app pages, so the badge helper and the link
/// interceptor work when the chat window is pointed there. Inert until the server redirects the chat window to that host. Set to false to back it out alone.
pub const MESSAGER_HOST_PAGES: bool = true;

/// 2.4.30, part (a): listen for the chat page's `chat:unread` message and move the chat badge from it. Inert until the page posts it. Set to false to back it out alone.
pub const PAGE_UNREAD_LISTENER: bool = true;

const MAIN_HOST: &str = "evolvepreneuriq.app";
const MESSAGER_HOST: &str = "messager.evolvepreneuriq.app";

/// The exact host of a http(s) URL, lower-cased; None for anything else, for credentials (`@`) and for an explicit port.
fn host_of(url: &str) -> Option<(bool, String)> {
    let lower = url.to_ascii_lowercase();
    let (https, rest) = if let Some(r) = lower.strip_prefix("https://") {
        (true, r.to_string())
    } else if let Some(r) = lower.strip_prefix("http://") {
        (false, r.to_string())
    } else {
        return None;
    };
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
    if authority.is_empty() || authority.contains('@') || authority.contains(':') {
        return None;
    }
    Some((https, authority.to_string()))
}

/// Is this page one the shell injects its helpers into?
pub fn is_app_page(url: &str) -> bool {
    is_app_page_with(url, MESSAGER_HOST_PAGES)
}

fn is_app_page_with(url: &str, messager_pages: bool) -> bool {
    match host_of(url) {
        // The main host keeps both schemes, as before (http redirects to https on the server).
        Some((_, h)) if h == MAIN_HOST => true,
        Some((true, h)) if h == MESSAGER_HOST => messager_pages,
        _ => false,
    }
}

/// The chat:unread total from a page message: a non-negative whole number, clamped so a hostile page cannot announce a billion.
pub fn clamp_total(raw: i64) -> i64 {
    raw.clamp(0, 100_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_main_host_is_an_app_page_with_or_without_a_path() {
        for u in ["https://evolvepreneuriq.app", "https://evolvepreneuriq.app/chat?embed=1", "http://evolvepreneuriq.app/login", "HTTPS://EvolvepreneurIQ.app/x", "https://evolvepreneuriq.app#a"] {
            assert!(is_app_page_with(u, true), "{u}");
            assert!(is_app_page_with(u, false), "{u} (messager switch off)");
        }
    }

    #[test]
    fn the_messager_host_is_an_app_page_only_over_https_and_only_while_its_switch_is_on() {
        assert!(is_app_page_with("https://messager.evolvepreneuriq.app/?screen=team-chat", true));
        assert!(!is_app_page_with("https://messager.evolvepreneuriq.app/", false), "switch off = 2.4.29 behaviour");
        assert!(!is_app_page_with("http://messager.evolvepreneuriq.app/", true), "http is never the messenger host");
    }

    #[test]
    fn look_alike_and_other_hosts_are_never_app_pages() {
        for u in [
            "https://evolvepreneuriq.app.evil.test/",
            "https://evolvepreneuriq.appx/",
            "https://evil.evolvepreneuriq.app/",
            "https://messager.evolvepreneuriq.app.evil.test/",
            "https://evil-messager.evolvepreneuriq.app/",
            "https://evolvepreneuriq.app@evil.test/",
            "https://evolvepreneuriq.app:8443/",
            "https://example.com/?u=https://evolvepreneuriq.app/",
            "file:///evolvepreneuriq.app/",
            "about:blank",
            "https://",
            "",
        ] {
            assert!(!is_app_page_with(u, true), "{u}");
        }
    }

    #[test]
    fn the_switches_ship_on() {
        assert!(MESSAGER_HOST_PAGES);
        assert!(PAGE_UNREAD_LISTENER);
        assert!(is_app_page("https://messager.evolvepreneuriq.app/"));
    }

    #[test]
    fn totals_are_clamped() {
        assert_eq!(clamp_total(-5), 0);
        assert_eq!(clamp_total(7), 7);
        assert_eq!(clamp_total(i64::MAX), 100_000);
    }
}
