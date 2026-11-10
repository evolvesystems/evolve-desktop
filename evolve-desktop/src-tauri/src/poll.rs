//! The 60-second badge poll made safe: a 10 s timeout (in the page), never two polls at once, back-off after failures, and a log line when a poll is slow or fails.
//! std-only on purpose: tested with plain `rustc --test src/poll.rs` where the Tauri toolchain is missing. Task 5395 (the desktop app "freezes every so often").
//!
//! Why: the poll's fetch had no timeout, no overlap guard and no back-off. When the server took 30-50 s, polls piled up behind the page's own requests (a webview allows
//! about 6 connections per host), so the page stalled while the window frame still answered. The poll runs inside the page, so Rust cannot time it itself: the page
//! reports each result through the `log_poll` command and this module decides what to log and when the next poll is due.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

/// The page aborts a request after this long.
pub const TIMEOUT_MS: u64 = 10_000;
/// Normal gap between polls.
pub const BASE_SECS: u64 = 60;
/// Longest gap after repeated failures.
pub const MAX_SECS: u64 = 600;
/// A poll that was started but never reported (the page navigated away) counts as finished after this long.
pub const STALE_SECS: u64 = 20;
/// Slower than this is worth a line in the log even when it succeeded.
pub const SLOW_MS: u64 = 3_000;

pub const ENDPOINTS: [&str; 2] = ["check-notifications", "sidebar-tabs"];
pub const OUTCOMES: [&str; 6] = ["ok", "timeout", "http", "network", "parse", "unauthorized"];

/// Seconds to wait before the next poll after `failures` failures in a row: 60, 120, 240, 480, then 600.
pub fn next_delay_secs(failures: u32) -> u64 {
    let shift = failures.min(10);
    (BASE_SECS << shift).min(MAX_SECS)
}

/// Only known endpoint names and outcomes are accepted from the page (it is remote content), and they come back as the static strings.
pub fn sanitize_endpoint(s: &str) -> Option<&'static str> {
    ENDPOINTS.iter().copied().find(|e| *e == s)
}

pub fn sanitize_outcome(s: &str) -> Option<&'static str> {
    OUTCOMES.iter().copied().find(|o| *o == s)
}

/// Durations from the page are clamped: a hostile or broken page cannot write absurd numbers.
pub fn clamp_ms(ms: i64) -> u64 {
    ms.clamp(0, 600_000) as u64
}

pub fn is_loggable(ms: u64, outcome: &str) -> bool {
    outcome != "ok" || ms >= SLOW_MS
}

/// One log line: endpoint, duration, outcome and how many failures in a row (no URLs, no query strings, no response text).
pub fn log_line(endpoint: &str, ms: u64, outcome: &str, failures: u32) -> String {
    format!("poll {} {} ms {} (failures in a row: {})", endpoint, ms, outcome, failures)
}

/// Pages where a signed-out person is expected: the app must not push them to the sign-in page again (that would loop).
pub fn is_sign_in_path(path: &str) -> bool {
    ["/login", "/2fa", "/forgot", "/reset", "/register", "/logout"].iter().any(|p| path == *p || path.starts_with(&format!("{p}/")) || path.starts_with(&format!("{p}?")))
}

pub fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Shared state of the timer poll: how many failures in a row, when the next poll is allowed, and when the running one started (0 = none running).
pub struct State {
    failures: AtomicU32,
    next_at: AtomicU64,
    started_at: AtomicU64,
    /// The server said 401: the person is signed out. No timer poll runs until a request succeeds again (EVO-BUG-20077).
    signed_out: AtomicBool,
}

impl State {
    pub const fn new() -> Self {
        State { failures: AtomicU32::new(0), next_at: AtomicU64::new(0), started_at: AtomicU64::new(0), signed_out: AtomicBool::new(false) }
    }

    /// May a poll start now? No while the previous one is still running (unless it is stale) or while backing off. Marks it started when it may.
    pub fn try_start(&self, now: u64) -> bool {
        if self.signed_out.load(Ordering::SeqCst) {
            return false;
        }
        let started = self.started_at.load(Ordering::SeqCst);
        if started != 0 && now.saturating_sub(started) < STALE_SECS {
            return false;
        }
        if now < self.next_at.load(Ordering::SeqCst) {
            return false;
        }
        self.started_at.store(now.max(1), Ordering::SeqCst);
        true
    }

    /// The page reported a result. Success resets the back-off; a failure doubles the wait. Returns the failures in a row.
    pub fn finish(&self, now: u64, ok: bool) -> u32 {
        self.started_at.store(0, Ordering::SeqCst);
        if ok {
            self.signed_out.store(false, Ordering::SeqCst);
            self.failures.store(0, Ordering::SeqCst);
            self.next_at.store(0, Ordering::SeqCst);
            0
        } else {
            let f = self.failures.fetch_add(1, Ordering::SeqCst).saturating_add(1);
            // The timer ticks every BASE_SECS, so waiting `next_delay_secs(f)` from now means skipping the ticks in between.
            self.next_at.store(now + next_delay_secs(f), Ordering::SeqCst);
            f
        }
    }

    /// The server answered 401. Stops the timer poll and clears the back-off. Returns true only the FIRST time (so the sign-in page is shown once, not on every report).
    pub fn sign_out(&self) -> bool {
        self.started_at.store(0, Ordering::SeqCst);
        self.failures.store(0, Ordering::SeqCst);
        self.next_at.store(0, Ordering::SeqCst);
        !self.signed_out.swap(true, Ordering::SeqCst)
    }

    pub fn is_signed_out(&self) -> bool {
        self.signed_out.load(Ordering::SeqCst)
    }

    pub fn failures(&self) -> u32 {
        self.failures.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_delay_doubles_from_sixty_seconds_and_stops_at_ten_minutes() {
        let d: Vec<u64> = (0..7).map(next_delay_secs).collect();
        assert_eq!(d, vec![60, 120, 240, 480, 600, 600, 600]);
        assert_eq!(next_delay_secs(u32::MAX), 600, "no overflow");
    }

    #[test]
    fn only_known_endpoints_and_outcomes_get_through() {
        assert_eq!(sanitize_endpoint("check-notifications"), Some("check-notifications"));
        assert_eq!(sanitize_endpoint("sidebar-tabs"), Some("sidebar-tabs"));
        for bad in ["", "check-notifications?x=1", "https://evil.test", "CHECK-NOTIFICATIONS", "check-notifications\n"] {
            assert_eq!(sanitize_endpoint(bad), None, "{:?}", bad);
        }
        for ok in OUTCOMES {
            assert_eq!(sanitize_outcome(ok), Some(ok));
        }
        for bad in ["", "OK", "ok ", "success", "timeout\nFAKE LOG LINE"] {
            assert_eq!(sanitize_outcome(bad), None, "{:?}", bad);
        }
    }

    #[test]
    fn durations_are_clamped() {
        assert_eq!(clamp_ms(-5), 0);
        assert_eq!(clamp_ms(1234), 1234);
        assert_eq!(clamp_ms(i64::MAX), 600_000);
    }

    #[test]
    fn a_healthy_fast_poll_is_not_logged_but_slow_or_failed_ones_are() {
        assert!(!is_loggable(120, "ok"));
        assert!(!is_loggable(SLOW_MS - 1, "ok"));
        assert!(is_loggable(SLOW_MS, "ok"));
        for o in ["timeout", "http", "network", "parse"] {
            assert!(is_loggable(5, o), "{}", o);
        }
    }

    #[test]
    fn the_log_line_has_the_endpoint_duration_outcome_and_failures_and_nothing_else() {
        assert_eq!(log_line("check-notifications", 41_200, "timeout", 2), "poll check-notifications 41200 ms timeout (failures in a row: 2)");
    }

    #[test]
    fn a_second_poll_does_not_start_while_the_first_is_running() {
        let s = State::new();
        assert!(s.try_start(1000));
        assert!(!s.try_start(1010), "still running");
        assert!(!s.try_start(1019));
        assert!(s.try_start(1020), "stale after 20 s: the page never reported");
    }

    #[test]
    fn a_reported_success_lets_the_next_tick_run() {
        let s = State::new();
        assert!(s.try_start(1000));
        assert_eq!(s.finish(1002, true), 0);
        assert!(s.try_start(1060));
    }

    #[test]
    fn failures_skip_ticks_with_the_doubling_back_off() {
        let s = State::new();
        assert!(s.try_start(1000));
        assert_eq!(s.finish(1010, false), 1);
        assert!(!s.try_start(1070), "after one failure the tick 60 s later is skipped");
        assert!(s.try_start(1130), "120 s after the failure it runs again");
        assert_eq!(s.finish(1131, false), 2);
        assert!(!s.try_start(1131 + 239));
        assert!(s.try_start(1131 + 240));
    }

    #[test]
    fn the_wait_never_exceeds_ten_minutes_however_many_failures() {
        let s = State::new();
        let mut t = 1000;
        for _ in 0..30 {
            assert!(s.try_start(t));
            s.finish(t, false);
            t += 600;
        }
        assert_eq!(s.failures(), 30);
        assert!(!s.try_start(t - 1));
        assert!(s.try_start(t), "ten minutes after the last failure it tries again");
    }

    #[test]
    fn one_success_clears_the_back_off_completely() {
        let s = State::new();
        for i in 0..4u64 {
            assert!(s.try_start(10_000 * (i + 1)));
            s.finish(10_000 * (i + 1), false);
        }
        assert!(s.try_start(100_000));
        assert_eq!(s.finish(100_001, true), 0);
        assert_eq!(s.failures(), 0);
        assert!(s.try_start(100_002), "no remaining wait");
    }

    #[test]
    fn time_going_backwards_cannot_wedge_the_poll() {
        let s = State::new();
        assert!(s.try_start(5000));
        // clock stepped back: saturating subtraction keeps "still running" until it is stale by the clock
        assert!(!s.try_start(100));
        assert!(s.try_start(5020));
    }

    #[test]
    fn a_401_stops_the_timer_poll_until_a_request_succeeds_again() {
        let st = State::new();
        assert!(st.try_start(1000));
        assert!(st.sign_out(), "first 401: show sign-in once");
        assert!(st.is_signed_out());
        for t in [1060, 1120, 5000, 90_000] {
            assert!(!st.try_start(t), "no poll while signed out (t={t})");
        }
        st.finish(100_000, true);
        assert!(!st.is_signed_out(), "a successful request proves sign-in");
        assert!(st.try_start(100_001));
    }

    #[test]
    fn only_the_first_401_asks_for_the_sign_in_page() {
        let st = State::new();
        assert!(st.sign_out());
        assert!(!st.sign_out());
        assert!(!st.sign_out());
        st.finish(10, true);
        assert!(st.sign_out(), "after signing in and out again it is a new first");
    }

    #[test]
    fn a_401_is_not_backed_off_it_is_stopped_and_a_failure_in_a_row_is_not_counted() {
        let st = State::new();
        st.finish(0, false);
        st.finish(0, false);
        assert_eq!(st.failures(), 2);
        st.sign_out();
        assert_eq!(st.failures(), 0);
    }

    #[test]
    fn unauthorized_is_an_accepted_outcome_and_is_logged() {
        assert_eq!(sanitize_outcome("unauthorized"), Some("unauthorized"));
        assert!(is_loggable(10, "unauthorized"));
    }

    #[test]
    fn the_sign_in_pages_are_recognised_by_path_only() {
        for p in ["/login", "/login/", "/login?next=/crm", "/2fa", "/2fa/check", "/forgot", "/reset/abc", "/register", "/logout"] {
            assert!(is_sign_in_path(p), "{p}");
        }
        for p in ["/", "/crm", "/loginfo", "/admin/login-history", "/dashboard"] {
            assert!(!is_sign_in_path(p), "{p}");
        }
    }
}
