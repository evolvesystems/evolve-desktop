//! The one number on the app icon (2.4.31): unread chat + unread mail. std-only on purpose: tested with plain `rustc --test src/tray_total.rs`.
//!
//! The icon itself is drawn by badge.rs (ported from Evolve Messenger): Windows taskbar overlay, macOS dock / Linux launcher badge count.
//! Counts only, never names or text (same privacy rule as the notifications).

/// A count that cannot be trusted below 0 or above this (a hostile page or a broken answer cannot show a billion).
pub const MAX_COUNT: i64 = 100_000;

pub fn clamp(count: i64) -> i64 {
    count.clamp(0, MAX_COUNT)
}

/// What the icon shows: chat plus mail, 0 = no badge. A missing source (-1 = not known yet) counts as nothing.
pub fn total(chat: i64, mail: i64) -> i64 {
    clamp(chat.max(0)) + clamp(mail.max(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_and_mail_are_added() {
        assert_eq!(total(3, 4), 7);
        assert_eq!(total(0, 5), 5);
        assert_eq!(total(5, 0), 5);
    }

    #[test]
    fn nothing_unread_clears_the_badge() {
        assert_eq!(total(0, 0), 0);
    }

    #[test]
    fn a_source_not_known_yet_counts_as_nothing_not_as_minus_one() {
        assert_eq!(total(-1, 4), 4);
        assert_eq!(total(2, -1), 2);
        assert_eq!(total(-1, -1), 0);
    }

    #[test]
    fn each_count_is_clamped_so_a_bad_answer_cannot_show_a_huge_number() {
        assert_eq!(total(i64::MAX, 0), MAX_COUNT);
        assert_eq!(total(i64::MAX, i64::MAX), 2 * MAX_COUNT);
        assert_eq!(clamp(-9), 0);
        assert_eq!(clamp(42), 42);
    }
}
