//! The page the "Get the phone app" QR code opens (John, 2 Nov 2026). One constant, one validator.
//! std-only on purpose: tested with plain `rustc --test src/phone_url.rs` where the Tauri toolchain is missing.
//!
//! v1 encodes the shared Messager download page (EIQ mobile app 13: Google Play, web app, APK). A per-tenant page can follow later by changing
//! `PHONE_APP_URL` (or choosing it by tenant) - the validator below is the only rule it has to satisfy.

/// Where the QR code points. Change here only.
pub const PHONE_APP_URL: &str = "https://evolvepreneuriq.app/app/13/download";

/// Longest URL we will encode (keeps the code small and easy to scan).
pub const MAX_LEN: usize = 80;

/// https only, a platform host, and the path `/app/{digits}/download` - nothing else (no query, no fragment, no credentials, no whitespace).
pub fn is_valid(url: &str) -> bool {
    if url.len() > MAX_LEN || url.chars().any(|c| c.is_whitespace() || c.is_control() || !c.is_ascii()) {
        return false;
    }
    let Some(rest) = url.strip_prefix("https://") else { return false };
    let (host, path) = match rest.split_once('/') {
        Some((h, p)) => (h, p),
        None => return false,
    };
    if !(host == "evolvepreneuriq.app" || host.ends_with(".evolvepreneuriq.app")) || host.contains('@') || host.contains(':') {
        return false;
    }
    let mut parts = path.split('/');
    let (Some("app"), Some(id), Some("download"), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else { return false };
    !id.is_empty() && id.len() <= 9 && id.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_configured_url_is_valid_and_is_the_page_john_sent() {
        assert!(is_valid(PHONE_APP_URL));
        assert_eq!(PHONE_APP_URL, "https://evolvepreneuriq.app/app/13/download");
    }

    #[test]
    fn a_tenant_subdomain_is_accepted() {
        assert!(is_valid("https://messager.evolvepreneuriq.app/app/7/download"));
    }

    #[test]
    fn plain_http_other_hosts_and_lookalikes_are_refused() {
        for u in [
            "http://evolvepreneuriq.app/app/13/download",
            "https://evil.example/app/13/download",
            "https://evolvepreneuriq.app.evil.example/app/13/download",
            "https://notevolvepreneuriq.app/app/13/download",
            "https://user@evolvepreneuriq.app/app/13/download",
            "https://evolvepreneuriq.app:8443/app/13/download",
            "javascript:alert(1)",
            "",
        ] {
            assert!(!is_valid(u), "{u} must be refused");
        }
    }

    #[test]
    fn only_the_app_id_download_path_is_accepted() {
        for u in [
            "https://evolvepreneuriq.app/app//download",
            "https://evolvepreneuriq.app/app/abc/download",
            "https://evolvepreneuriq.app/app/13/download/extra",
            "https://evolvepreneuriq.app/app/13/download?x=1",
            "https://evolvepreneuriq.app/app/13/download#x",
            "https://evolvepreneuriq.app/app/13",
            "https://evolvepreneuriq.app/admin/lab",
            "https://evolvepreneuriq.app",
            "https://evolvepreneuriq.app/app/1234567890/download",
        ] {
            assert!(!is_valid(u), "{u} must be refused");
        }
    }

    #[test]
    fn whitespace_control_characters_non_ascii_and_long_urls_are_refused() {
        assert!(!is_valid("https://evolvepreneuriq.app/app/13/download "));
        assert!(!is_valid("https://evolvepreneuriq.app/app/13/down\nload"));
        assert!(!is_valid("https://evolvepreneuriq.app/app/１３/download"));
        let long = format!("https://evolvepreneuriq.app/app/13/download{}", "a".repeat(80));
        assert!(!is_valid(&long));
    }
}
