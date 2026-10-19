//! Turns a failed in-app update into a kind the dialog can explain and the log can record.
//! std-only on purpose: it is tested with plain `rustc --test src/update_error.rs` where the Tauri toolchain is missing.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Fetching the update file from GitHub.
    Download,
    /// Checking the signature and replacing the installed app.
    Install,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Network,
    Signature,
    /// The app could not replace itself (read-only volume, no permission, running from a disk image).
    Install,
    /// The update was already used by an earlier click (should no longer happen: it is put back on failure).
    Consumed,
    Unknown,
}

impl Kind {
    pub fn code(self) -> &'static str {
        match self {
            Kind::Network => "network",
            Kind::Signature => "signature",
            Kind::Install => "install",
            Kind::Consumed => "consumed",
            Kind::Unknown => "unknown",
        }
    }
}

pub fn classify(stage: Stage, message: &str) -> Kind {
    let m = message.to_lowercase();
    if m.contains("already consumed") || m.contains("no pending update") {
        return Kind::Consumed;
    }
    if m.contains("signature") || m.contains("minisign") || m.contains("public key") {
        return Kind::Signature;
    }
    if stage == Stage::Install {
        return Kind::Install;
    }
    let network = ["request", "network", "dns", "connect", "timed out", "timeout", "http", "tls", "ssl", "certificate", "proxy", "error sending"];
    if network.iter().any(|w| m.contains(w)) {
        return Kind::Network;
    }
    if stage == Stage::Download {
        return Kind::Network;
    }
    Kind::Unknown
}

/// What the JS catch receives: "<kind>|<detail>". The detail is trimmed to one short line; it is the error text of the
/// updater (no URLs with tokens: the update URL is a public GitHub release URL), never page content.
pub fn encode(kind: Kind, message: &str) -> String {
    let one_line: String = message.chars().map(|c| if c.is_control() { ' ' } else { c }).take(300).collect();
    format!("{}|{}", kind.code(), one_line.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failed_download_is_a_network_error_unless_it_says_signature() {
        assert_eq!(classify(Stage::Download, "error sending request for url (https://github.com/...)"), Kind::Network);
        assert_eq!(classify(Stage::Download, "something odd"), Kind::Network);
        assert_eq!(classify(Stage::Download, "the signature could not be verified"), Kind::Signature);
    }

    #[test]
    fn a_failed_install_is_an_install_error_unless_it_is_the_signature() {
        assert_eq!(classify(Stage::Install, "Read-only file system (os error 30)"), Kind::Install);
        assert_eq!(classify(Stage::Install, "Permission denied (os error 13)"), Kind::Install);
        assert_eq!(classify(Stage::Install, "minisign verification failed"), Kind::Signature);
    }

    #[test]
    fn the_consumed_cases_are_recognised_at_any_stage() {
        assert_eq!(classify(Stage::Download, "Update already consumed"), Kind::Consumed);
        assert_eq!(classify(Stage::Install, "No pending update"), Kind::Consumed);
    }

    #[test]
    fn the_encoded_error_is_kind_pipe_one_short_line() {
        let s = encode(Kind::Install, "line one\nline two\t\u{7}end");
        assert!(s.starts_with("install|"));
        assert!(!s.contains('\n') && !s.contains('\t'));
        assert!(encode(Kind::Network, &"x".repeat(1000)).len() <= "network|".len() + 300);
    }
}
