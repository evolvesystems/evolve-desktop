//! Page timing in evolveapp.log (task: "the desktop app is very slow to load the Lab"). Times only: no URLs, no ids, no personal data.
//! One line per EIQ page: launch to window open, page load time, the two injected sidebar requests' time, whether it is the first page and whether it is the first load after
//! the content view was parked (2.4.25). Also decides when the sidebar-tabs request is due again. std-only, tested with plain `rustc --test src/timing.rs`.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// The sidebar tabs are not asked for again if they were answered this recently.
pub const TABS_FRESH_SECS: u64 = 60;
/// The injected requests wait this long after the page is ready, so they do not compete with the page's own start-up requests.
pub const INJECT_DELAY_MS: u64 = 1500;
/// Durations reported by the page are clamped to this (a broken page cannot write absurd numbers).
pub const MAX_MS: i64 = 600_000;

pub struct State {
    launch_to_window_ms: AtomicU64,
    started_ms: AtomicU64,
    load_ms: AtomicU64,
    pages_seen: AtomicU64,
    after_park: AtomicBool,
    last_tabs_secs: AtomicU64,
}

impl State {
    pub const fn new() -> Self {
        State {
            launch_to_window_ms: AtomicU64::new(0),
            started_ms: AtomicU64::new(0),
            load_ms: AtomicU64::new(0),
            pages_seen: AtomicU64::new(0),
            after_park: AtomicBool::new(false),
            last_tabs_secs: AtomicU64::new(0),
        }
    }

    /// The window was created `ms` after launch.
    pub fn window_open(&self, ms: u64) {
        self.launch_to_window_ms.store(ms, Ordering::SeqCst);
    }

    /// The next page load is the first after the content view was parked.
    pub fn mark_after_park(&self) {
        self.after_park.store(true, Ordering::SeqCst);
    }

    /// A page started loading `now_ms` after launch.
    pub fn page_started(&self, now_ms: u64) {
        self.started_ms.store(now_ms, Ordering::SeqCst);
    }

    /// The page finished loading. The first page is measured from the window opening, later ones from their own start.
    pub fn page_ready(&self, now_ms: u64) {
        let first = self.pages_seen.load(Ordering::SeqCst) == 0;
        let from = if first { self.launch_to_window_ms.load(Ordering::SeqCst) } else { self.started_ms.load(Ordering::SeqCst) };
        self.load_ms.store(now_ms.saturating_sub(from), Ordering::SeqCst);
    }

    /// The injected requests finished after `requests_ms`. Returns the log line and resets the per-page flags.
    pub fn finish(&self, requests_ms: i64, lab: bool) -> String {
        let first = self.pages_seen.fetch_add(1, Ordering::SeqCst) == 0;
        let parked = self.after_park.swap(false, Ordering::SeqCst);
        line(self.launch_to_window_ms.load(Ordering::SeqCst), self.load_ms.load(Ordering::SeqCst), clamp(requests_ms), first, parked, lab)
    }

    /// True when the sidebar tabs have not been answered in the last TABS_FRESH_SECS.
    pub fn tabs_due(&self, now_secs: u64) -> bool {
        let last = self.last_tabs_secs.load(Ordering::SeqCst);
        last == 0 || now_secs.saturating_sub(last) >= TABS_FRESH_SECS
    }

    pub fn tabs_answered(&self, now_secs: u64) {
        self.last_tabs_secs.store(now_secs.max(1), Ordering::SeqCst);
    }
}

pub fn clamp(ms: i64) -> u64 {
    ms.clamp(0, MAX_MS) as u64
}

/// The one log line John sends. Example: "timing launch_to_window=410ms load=2310ms requests=640ms first=yes after_park=no lab=yes".
pub fn line(launch_to_window: u64, load: u64, requests: u64, first: bool, after_park: bool, lab: bool) -> String {
    let yn = |b: bool| if b { "yes" } else { "no" };
    format!("timing launch_to_window={}ms load={}ms requests={}ms first={} after_park={} lab={}", launch_to_window, load, requests, yn(first), yn(after_park), yn(lab))
}

/// Is this URL the Lab (the path only is looked at; nothing from the URL is logged).
pub fn is_lab_path(path: &str) -> bool {
    path == "/admin/lab" || path.starts_with("/admin/lab/") || path.starts_with("/admin/lab?")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_page_is_measured_from_the_window_opening_and_flagged_first() {
        let s = State::new();
        s.window_open(400);
        s.page_started(900);
        s.page_ready(2700);
        assert_eq!(s.finish(640, false), "timing launch_to_window=400ms load=2300ms requests=640ms first=yes after_park=no lab=no");
    }

    #[test]
    fn a_later_page_is_measured_from_its_own_start_and_is_not_first() {
        let s = State::new();
        s.window_open(400);
        s.page_started(900);
        s.page_ready(2700);
        s.finish(100, false);
        s.page_started(10_000);
        s.page_ready(11_250);
        assert_eq!(s.finish(300, true), "timing launch_to_window=400ms load=1250ms requests=300ms first=no after_park=no lab=yes");
    }

    #[test]
    fn the_park_flag_covers_only_the_next_page() {
        let s = State::new();
        s.window_open(1);
        s.page_started(1);
        s.page_ready(2);
        s.finish(1, false);
        s.mark_after_park();
        s.page_started(5);
        s.page_ready(9);
        assert!(s.finish(1, false).contains("after_park=yes"));
        s.page_started(10);
        s.page_ready(12);
        assert!(s.finish(1, false).contains("after_park=no"));
    }

    #[test]
    fn the_tabs_request_is_skipped_for_a_minute_after_it_was_answered() {
        let s = State::new();
        assert!(s.tabs_due(1000), "never asked: due");
        s.tabs_answered(1000);
        assert!(!s.tabs_due(1000 + TABS_FRESH_SECS - 1));
        assert!(s.tabs_due(1000 + TABS_FRESH_SECS));
    }

    #[test]
    fn page_values_are_clamped_and_a_clock_that_goes_backwards_cannot_underflow() {
        assert_eq!(clamp(-5), 0);
        assert_eq!(clamp(10_000_000), MAX_MS as u64);
        let s = State::new();
        s.window_open(5000);
        s.page_ready(100);
        assert!(s.finish(i64::MIN, false).contains("load=0ms requests=0ms"));
    }

    #[test]
    fn the_line_holds_times_and_yes_no_only() {
        let l = line(1, 2, 3, true, false, true);
        assert_eq!(l, "timing launch_to_window=1ms load=2ms requests=3ms first=yes after_park=no lab=yes");
        assert!(!l.contains("http") && !l.contains('/') && !l.contains('@'));
    }

    #[test]
    fn only_the_lab_path_counts_as_the_lab() {
        assert!(is_lab_path("/admin/lab"));
        assert!(is_lab_path("/admin/lab/anything"));
        assert!(!is_lab_path("/admin/laboratory"));
        assert!(!is_lab_path("/dashboard"));
    }
}
