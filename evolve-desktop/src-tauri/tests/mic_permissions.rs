//! macOS microphone and speech recognition permissions (EVO-BUG-20036: the Lab's Talk button said "Voice input: not-allowed" in the desktop window).
//! Without the usage strings macOS refuses silently; without the audio-input entitlement a signed app gets no microphone. std-only, like tests/acl.rs.
use std::fs;

fn plist(name: &str) -> String {
    fs::read_to_string(name).unwrap()
}

/// The <string> that follows <key>NAME</key>, if the key exists.
fn string_after(doc: &str, key: &str) -> Option<String> {
    let k = format!("<key>{key}</key>");
    let rest = &doc[doc.find(&k)? + k.len()..];
    let open = rest.find("<string>")? + "<string>".len();
    let close = rest.find("</string>")?;
    // the value must belong to this key: nothing but whitespace between the key and its string
    if !rest[..open - "<string>".len()].trim().is_empty() {
        return None;
    }
    Some(rest[open..close].to_string())
}

fn is_true_after(doc: &str, key: &str) -> bool {
    let k = format!("<key>{key}</key>");
    match doc.find(&k) {
        Some(i) => doc[i + k.len()..].trim_start().starts_with("<true/>"),
        None => false,
    }
}

#[test]
fn info_plist_explains_the_microphone_in_plain_words() {
    let p = plist("Info.plist");
    let s = string_after(&p, "NSMicrophoneUsageDescription").expect("NSMicrophoneUsageDescription missing");
    assert_eq!(s, "EvolveApp uses the microphone when you press Talk in the Lab.");
}

#[test]
fn info_plist_explains_speech_recognition_in_plain_words() {
    let p = plist("Info.plist");
    let s = string_after(&p, "NSSpeechRecognitionUsageDescription").expect("NSSpeechRecognitionUsageDescription missing");
    assert_eq!(s, "EvolveApp uses the microphone when you press Talk in the Lab.");
}

#[test]
fn entitlements_allow_audio_input_and_keep_the_network_ones() {
    let e = plist("Entitlements.plist");
    assert!(is_true_after(&e, "com.apple.security.device.audio-input"), "audio-input entitlement missing or not true");
    assert!(is_true_after(&e, "com.apple.security.network.client"));
}

#[test]
fn both_files_are_wellformed_plists() {
    for f in ["Info.plist", "Entitlements.plist"] {
        let d = plist(f);
        assert!(d.contains("<plist version=\"1.0\">") && d.trim_end().ends_with("</plist>"), "{f}");
        assert_eq!(d.matches("<dict>").count(), 1, "{f}: one top-level dict");
        assert_eq!(d.matches("<key>").count(), d.matches("</key>").count(), "{f}");
    }
}

#[test]
fn the_bundle_config_still_points_at_the_entitlements_file() {
    let c = plist("tauri.conf.json");
    assert!(c.contains("\"entitlements\": \"Entitlements.plist\""));
}
