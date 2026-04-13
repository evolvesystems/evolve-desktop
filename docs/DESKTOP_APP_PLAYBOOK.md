# EvolveApp Desktop — Developer Playbook

## Architecture

**Tauri 2.x** desktop wrapper for `evolvepreneuriq.app`. NOT a native app — it loads the web app in webviews with native OS integration.

### Webview Layout
```
┌──────┬────────────────────────────────────────┐
│      │ [Tab 1 ×] [Tab 2 ×] [Tab 3 ×] [+]    │  tabbar (30px)
│ Side ├────────────────────────────────────────┤
│ bar  │                                        │
│ 56px │  Active tab's webview content          │  tab_N webviews
│      │  (each tab = separate Tauri webview)   │
│      │                                        │
└──────┴────────────────────────────────────────┘
```

**Three webview types:**
- `sidebar` — local HTML (`evolve://localhost/sidebar.html`), 56px wide, full height
- `tabbar` — local HTML (`evolve://localhost/tabbar.html`), 30px height, right column
- `tab_N` — remote webviews loading `evolvepreneuriq.app`, one per browser tab

### Key Files
```
evolve-desktop/
├── src-tauri/
│   ├── src/main.rs          # All Rust code — commands, setup, handlers
│   ├── assets/
│   │   ├── sidebar.html     # Left sidebar (nav icons)
│   │   └── tabbar.html      # Browser tab bar
│   ├── capabilities/
│   │   └── default.json     # Permissions for webviews
│   ├── tauri.conf.json      # App config, deep links, updater
│   ├── Cargo.toml           # Rust dependencies
│   └── icons/               # App icons (all sizes)
├── dist/                    # Frontend dist (minimal, just splash)
└── build-frontend.js        # Pre-build script
```

## How It Works

### Tab Management
- **State**: `AppTabs` (Mutex<TabState>) stores `Vec<TabInfo>` with id, url, title, active flag
- **Create tab**: spawns a new Tauri webview, positions at (sidebar_width, tabbar_height)
- **Switch tab**: moves active webview on-screen, moves others to (-9999, -9999)
- **Close tab**: destroys webview via `.close()`, activates nearest neighbor
- **Persistence**: tabs saved to `browser_tabs.json` in app data dir on every state change

### Content Injection (on_page_load)
Every content webview gets JS injected on page load:
1. Fetch sidebar tabs config from `/api/v1/desktop/sidebar/tabs`
2. Fetch badge data from `/api/v1/desktop/check-notifications`
3. Extract page title → send to tabbar via `update_tab_title` command
4. Register keyboard shortcuts (Ctrl+T/W/Tab/N/1-9)

### Sidebar ↔ Content Communication
- Sidebar calls `navigate_or_switch_tab` command (smart routing)
- Content emits tab/badge data via `relay_tabs_to_sidebar` / `relay_badges_to_sidebar`
- Sidebar receives events: `content-navigated`, `content-loading`, `content-loaded`

### External Links
Handled in `base.html.twig` (server-side template):
- Detects Tauri via `window.__TAURI_INTERNALS__`
- External links → `plugin:shell|open` (opens system browser)
- Internal links → navigate in same webview

## Keyboard Shortcuts
| Shortcut | Action |
|----------|--------|
| Ctrl+T | New tab (dashboard) |
| Ctrl+W | Close current tab |
| Ctrl+Tab | Next tab |
| Ctrl+Shift+Tab | Previous tab |
| Ctrl+1-9 | Switch to tab N |
| Ctrl+N | Quick compose email |

## Plugins
| Plugin | Purpose |
|--------|---------|
| `tauri-plugin-shell` | Open external URLs in system browser |
| `tauri-plugin-notification` | Native OS notifications (new emails) |
| `tauri-plugin-http` | HTTP requests from webviews |
| `tauri-plugin-autostart` | Launch on login (macOS LaunchAgent) |
| `tauri-plugin-deep-link` | Handle `evolveapp://` and `mailto:` URLs |
| `tauri-plugin-updater` | Auto-update from endpoint |

## Deep Links
- `evolveapp://path` → navigates active tab to `evolvepreneuriq.app/path`
- `mailto:user@example.com` → opens email compose with recipient pre-filled

## Auto-Update
- Checks `https://evolvepreneuriq.app/api/v1/desktop/updates/{target}/{version}` on startup
- Shows modal in content webview with install/later options
- Downloads + installs + restarts automatically
- Update artifacts built by CI, signed with minisign key

## Building

### Prerequisites
- Rust toolchain (`rustup`)
- Node.js (for `build-frontend.js`)
- Xcode (macOS) or Visual Studio (Windows)

### Development
```bash
cd evolve-desktop
cargo tauri dev
```

### Production Build
```bash
cargo tauri build
```

Output: `.dmg` (macOS), `.msi` (Windows), `.AppImage` (Linux)

### CI/CD
- Push to GitHub `master` → GitLab CI builds + signs + publishes
- macOS: `.dmg` + `.app.tar.gz` (for updater)
- Windows: `.msi` + `.msi.zip` (for updater)
- Artifacts uploaded to release endpoint

## Custom Protocol
Local assets served via `evolve://localhost/` custom URI scheme:
```rust
.register_uri_scheme_protocol("evolve", |_ctx, request| {
    // Reads from src-tauri/assets/ (dev) or Resources/assets/ (prod)
})
```

## System Tray
- Left-click tray icon → show/focus window
- Right-click → menu: Email, Chat, Docs, VA, Dashboard, CRM, Calendar, Books, Quit

## Common Issues

### Links don't open in browser
Check `base.html.twig` — the Tauri link interception block must be enabled (not wrapped in `if(false)`). Uses `plugin:shell|open` for external URLs.

### Duplicate sidebars
The sidebar is loaded by Tauri's `on_page_load`. If `base.html.twig` also creates a sidebar (sections 3-5 of the desktop block), you get duplicates. Those sections must stay in `if(false)`.

### Build fails — capability errors
Add new webview labels to `capabilities/default.json` → `webviews` array. Tab webviews use wildcard `tab_*`.

### Multiple instances collide
Both instances share `app_data_dir()` for `sidebar_tabs.json` and `browser_tabs.json`. Multi-window (same process) is the better approach — not yet implemented.

## Repositories
- **Desktop app**: `github.com/evolvesystems/evolve-desktop` (Rust/Tauri)
- **Web app**: `gitlab.evolvepreneur.app/symfony-dev/eiq-manager` (PHP/Symfony)
- Web app serves the content; desktop app is just the shell

## Version History
- **2.3.x** — Dual webview (sidebar + content)
- **2.4.x** — Browser tabs, native notifications, mailto handler, keyboard shortcuts

## CI/CD Build Process

### Trigger a Build
- **Auto**: Push a version tag (`git tag v2.4.0 && git push --tags`)
- **Manual**: GitHub Actions → "Build Multi-Platform Release" → Run workflow

### Build Matrix
| Platform | Target | Output |
|----------|--------|--------|
| macOS Intel | x86_64-apple-darwin | `.dmg`, `.app.tar.gz` |
| macOS ARM | aarch64-apple-darwin | `.dmg`, `.app.tar.gz` |
| Windows | x86_64-pc-windows-msvc | `.msi`, `.msi.zip` |
| Linux | x86_64-unknown-linux-gnu | `.AppImage`, `.deb` |

### Version Bumping
Update version in TWO files:
```bash
# src-tauri/tauri.conf.json
"version": "2.4.0"

# src-tauri/Cargo.toml
version = "2.4.0"
```

Then tag and push:
```bash
git tag v2.4.0
git push origin master --tags
```

### Auto-Update
Built apps check `https://evolvepreneuriq.app/api/v1/desktop/updates/{target}/{version}` on startup. If a newer version exists, shows install modal. No need to reinstall — updates apply in-place.
