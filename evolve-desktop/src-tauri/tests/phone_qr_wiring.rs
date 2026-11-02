//! The "Get the phone app" QR (John, 2 Nov 2026) must stay offline, origin-guarded, validated and reachable from both the About window and the tray.
//! std-only, like tests/acl.rs.
use std::fs;

fn main_rs() -> String {
    fs::read_to_string("src/main.rs").unwrap()
}

#[test]
fn the_qr_is_made_in_rust_with_no_remote_service() {
    let src = main_rs();
    assert!(src.contains("qrcode::QrCode::with_error_correction_level(url.as_bytes(), qrcode::EcLevel::M)"));
    for bad in ["api.qrserver", "chart.googleapis", "qrcode.tec-it", "goqr.me", "quickchart.io"] {
        assert!(!src.contains(bad), "{bad}: no remote QR service");
    }
    let cargo = fs::read_to_string("Cargo.toml").unwrap();
    assert!(cargo.contains("qrcode = { version = \"0.14\", default-features = false, features = [\"svg\"] }"));
}

#[test]
fn the_code_is_black_on_white_with_a_quiet_zone_and_at_least_256_px() {
    let src = main_rs();
    assert!(src.contains(".min_dimensions(256, 256)"));
    assert!(src.contains(".quiet_zone(true)"));
    assert!(src.contains("Color(\"#000000\")") && src.contains("Color(\"#ffffff\")"));
    assert!(src.contains("svg.style.width = '256px'"));
}

#[test]
fn the_command_checks_the_origin_and_the_url_and_is_registered() {
    let src = main_rs();
    let at = src.find("async fn phone_app_qr").expect("command exists");
    let body = &src[at..at + 600];
    let origin = body.find("offline_cache::from_app_origin(&webview)").expect("origin guard");
    let valid = body.find("phone_url::is_valid(url)").expect("url validation");
    let render = body.find("QrCode::with_error_correction_level").expect("render");
    assert!(origin < valid && valid < render, "guard, then validate, then render");
    assert!(src.contains("            phone_app_qr,"), "listed in generate_handler!");
}

#[test]
fn the_url_comes_from_the_one_constant() {
    let src = main_rs();
    assert!(src.contains("phone_url::PHONE_APP_URL"));
    assert!(!src.contains("evolvepreneuriq.app/app/13/download"), "the URL lives only in phone_url.rs");
    let u = fs::read_to_string("src/phone_url.rs").unwrap();
    assert!(u.contains("pub const PHONE_APP_URL: &str = \"https://evolvepreneuriq.app/app/13/download\";"));
}

#[test]
fn both_the_about_window_and_the_tray_open_it() {
    let src = main_rs();
    assert!(src.contains("id=\"evolve-info-phone\""));
    assert!(src.contains("\"phone_qr\", \"Get the phone app (QR code)\""));
    assert!(src.contains("\"phone_qr\" =>") && src.contains("info_modal(app, true)"));
    assert!(src.contains("info_modal(&app, false)"));
}

#[test]
fn the_svg_is_inserted_as_the_rust_made_it_and_every_other_value_as_text() {
    let src = main_rs();
    assert!(src.contains("card.innerHTML = r.svg;"));
    assert!(src.contains("link.textContent = r.url;"));
    assert!(src.contains("copy.setAttribute('aria-label'"), "no bare icon button");
}
