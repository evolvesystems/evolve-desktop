//! Park the content view while the window is hidden in the tray (task 5469, "the desktop app is very memory hungry").
//! Closing the window only hides it, so the full EIQ app kept running in the hidden WebView2. After PARK_AFTER_SECS hidden, the content view is sent to a tiny
//! same-origin JSON page (the public version endpoint): the heavy page is released, the origin and session stay, so the 60 s badge poll still runs on it.
//! When the window is shown again the saved page is reloaded. std-only on purpose: tested with plain `rustc --test src/park.rs`.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// Hidden this long before the content view is parked.
pub const PARK_AFTER_SECS: u64 = 600;
/// The tiny page the content view rests on while parked: public, a few bytes of JSON, on the app origin (cookies and the Tauri capability still apply).
pub const PARK_URL: &str = "https://evolvepreneuriq.app/api/v1/desktop/version";

pub struct State {
    hidden_at: AtomicU64,
    saved: Mutex<Option<String>>,
}

impl State {
    pub const fn new() -> Self {
        State { hidden_at: AtomicU64::new(0), saved: Mutex::new(None) }
    }

    /// The window was hidden. Keeps the first time if it is called again.
    pub fn hidden(&self, now: u64) {
        let _ = self.hidden_at.compare_exchange(0, now.max(1), Ordering::SeqCst, Ordering::SeqCst);
    }

    /// The window is visible again.
    pub fn shown(&self) {
        self.hidden_at.store(0, Ordering::SeqCst);
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden_at.load(Ordering::SeqCst) != 0
    }

    /// True once the window has been hidden long enough and nothing is parked yet.
    pub fn due(&self, now: u64) -> bool {
        let at = self.hidden_at.load(Ordering::SeqCst);
        at != 0 && now.saturating_sub(at) >= PARK_AFTER_SECS && self.saved.lock().map(|s| s.is_none()).unwrap_or(false)
    }

    /// Remember the page to come back to.
    pub fn parked(&self, url: String) {
        if let Ok(mut s) = self.saved.lock() {
            *s = Some(url);
        }
    }

    /// The page to reload, once. None when nothing is parked.
    pub fn take_saved(&self) -> Option<String> {
        self.saved.lock().ok().and_then(|mut s| s.take())
    }

    /// Something else navigated the content view (a deep link, a tray item): do not reload the old page over it.
    pub fn forget(&self) {
        let _ = self.take_saved();
    }
}

pub fn is_park_url(url: &str) -> bool {
    url == PARK_URL || url.starts_with(&format!("{PARK_URL}?")) || url.starts_with(&format!("{PARK_URL}#"))
}

/// Only an ordinary page may be parked and later reloaded: https, no whitespace or control characters, not the park page, not a local/splash page.
pub fn is_restorable(url: &str) -> bool {
    url.len() <= 2048
        && url.starts_with("https://")
        && !is_park_url(url)
        && !url.contains("splash.html")
        && !url.chars().any(|c| c.is_whitespace() || c.is_control())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_is_due_while_the_window_is_visible() {
        let s = State::new();
        assert!(!s.due(1_000_000));
        assert!(!s.is_hidden());
    }

    #[test]
    fn it_is_due_only_after_the_window_has_been_hidden_long_enough() {
        let s = State::new();
        s.hidden(1000);
        assert!(s.is_hidden());
        assert!(!s.due(1000 + PARK_AFTER_SECS - 1));
        assert!(s.due(1000 + PARK_AFTER_SECS));
    }

    #[test]
    fn hiding_again_keeps_the_first_time_and_showing_resets_it() {
        let s = State::new();
        s.hidden(1000);
        s.hidden(5000);
        assert!(s.due(1000 + PARK_AFTER_SECS));
        s.shown();
        assert!(!s.is_hidden());
        assert!(!s.due(1_000_000));
    }

    #[test]
    fn once_parked_it_is_not_due_again_and_the_page_comes_back_once() {
        let s = State::new();
        s.hidden(1);
        s.parked("https://evolvepreneuriq.app/crm".to_string());
        assert!(!s.due(1 + PARK_AFTER_SECS + 100));
        assert_eq!(s.take_saved().as_deref(), Some("https://evolvepreneuriq.app/crm"));
        assert_eq!(s.take_saved(), None);
    }

    #[test]
    fn a_deep_link_forgets_the_parked_page() {
        let s = State::new();
        s.parked("https://evolvepreneuriq.app/old".to_string());
        s.forget();
        assert_eq!(s.take_saved(), None);
    }

    #[test]
    fn the_park_page_is_on_the_app_origin_and_never_restorable() {
        assert!(PARK_URL.starts_with("https://evolvepreneuriq.app/"));
        assert!(is_park_url(PARK_URL));
        assert!(is_park_url(&format!("{PARK_URL}?x=1")));
        assert!(!is_park_url("https://evolvepreneuriq.app/api/v1/desktop/versions"));
        assert!(!is_restorable(PARK_URL));
    }

    #[test]
    fn only_ordinary_https_pages_are_restorable() {
        assert!(is_restorable("https://evolvepreneuriq.app/email-manager?id=3#top"));
        assert!(is_restorable("https://other.example/page"));
        for bad in ["http://evolvepreneuriq.app/x", "evolve://localhost/splash.html", "https://evolvepreneuriq.app/a b", "https://x/\n", "javascript:alert(1)", "about:blank", ""] {
            assert!(!is_restorable(bad), "{bad:?}");
        }
        assert!(!is_restorable(&format!("https://evolvepreneuriq.app/{}", "a".repeat(2100))));
    }
}
