// Every command in generate_handler! (src/main.rs) must be listed here AND granted in
// capabilities/default.json as "allow-<command-with-dashes>". Without it, a page on evolvepreneuriq.app that
// calls the command fails with "Command X not allowed by ACL" (EVO-BUG-18921/18926).
// tests/acl.rs fails when the three drift apart.
const COMMANDS: &[&str] = &[
    "get_app_version",
    "save_cached_tabs",
    "load_cached_tabs",
    "navigate_content",
    "get_content_url",
    "content_go_back",
    "content_reload",
    "toggle_sidebar_config",
    "show_info_modal",
    "check_for_updates",
    "install_update",
    "save_tabs_via_content",
    "relay_tabs_to_sidebar",
    "relay_badges_to_sidebar",
    "open_external_url",
    "offline_cache_put",
    "offline_cache_get",
    "offline_cache_clear",
    "close_quick_send",
]; 

fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new().app_manifest(tauri_build::AppManifest::new().commands(COMMANDS)),
    )
    .expect("failed to run tauri-build");
}
