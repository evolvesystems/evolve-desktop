# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

🐕 **CLAUDE.md Confirmation Protocol**
When Claude is told to read CLAUDE.md, Claude will say "woof" to indicate that CLAUDE.md has been read before Claude starts.

## Project Overview

EvolveApp Desktop is a modular business management application built with:
- **Frontend**: Vue 3 (Composition API) + TypeScript + Vite 7
- **Desktop Runtime**: Tauri 2.0 + Rust
- **Backend API**: Symfony 7.3 (PHP) served via Docker on port 8547
- **Database**: PostgreSQL 17 (remote only via eiq-manager API)
- **Styling**: Tailwind CSS v3 + DaisyUI 5.2

**Critical Architecture Note**: This app uses **remote database only** - no local SQLite. All data operations go through the eiq-manager API.

## 🚨 CRITICAL DEBUGGING RULES - READ FIRST

### **NEVER BUILD IN WSL FOR WINDOWS RELEASES**

**🚨 CRITICAL**: Building in WSL creates Linux packages, NOT Windows installers.

```bash
# ❌ WRONG: Building in WSL terminal
john@DELL-LAPTOP-WORK:~$ npm run tauri:build
# Creates: .deb, .rpm, .AppImage (Linux only)

# ✅ CORRECT: Building in Windows PowerShell
PS C:\evolve-desktop> npm run tauri:build
# Creates: .msi, .exe (Windows installers)
```

**Why**: WSL is a Linux environment - `npm run tauri:build` always targets the host OS.

**Solution**: Use GitHub Actions for multi-platform builds (see CI/CD section below).

## Common Build Commands

### Development
```bash
npm run dev                  # Vue dev server only (http://localhost:5173)
npm run tauri:dev            # Full desktop app with hot reload
```

### Building
```bash
npm run build                # Frontend build only
npm run tauri:build          # Full desktop build (Windows/Linux/macOS)
npm run tauri:build:windows  # Windows-specific build
npm run tauri:build:linux    # Linux-specific build
```

**Important**: First build takes 10-15 minutes due to Rust compilation. Subsequent builds are faster.

### Cross-Platform Building
- **From WSL**: Builds Linux packages (.deb, .rpm, .AppImage)
- **From Windows PowerShell**: Builds Windows installers (.msi, .exe)
- **From macOS**: Builds macOS packages (.dmg, .app)

To access WSL files from Windows PowerShell:
```powershell
cd \\wsl$\Ubuntu\home\john\sources\evolveapp\evolve-desktop
```

**Note**: Windows cannot execute commands from UNC paths. Copy project to Windows drive first:
```powershell
xcopy \\wsl$\... C:\evolve-desktop /E /I /H
```

## GitHub Actions CI/CD

### **🚨 CRITICAL: Dual Remote Setup**

**This repository has TWO git remotes:**
- **origin** → GitLab (`git@gitlab.evolvepreneur.app:symfony-dev/evolveapp.git`)
- **github** → GitHub (`https://github.com/evolvesystems/evolve-desktop.git`)

**IMPORTANT**: GitHub Actions builds from the **github** remote, NOT origin!

**After making changes, you MUST push to BOTH remotes:**
```bash
git push origin master   # Push to GitLab (backup/primary repo)
git push github master   # Push to GitHub (for Actions builds)
```

**Common mistake**: If you only `git push`, it goes to `origin` (GitLab). GitHub Actions will build OLD code because it's not on GitHub yet!

**Verification**: Check GitHub has your latest changes:
```bash
# Check local version
grep '"version"' package.json

# Check GitHub version (should match)
curl -s https://raw.githubusercontent.com/evolvesystems/evolve-desktop/master/evolve-desktop/package.json | grep '"version"'
```

### Build Triggering

Builds are triggered via EIQ Manager admin panel:
- URL: `http://localhost:8547/admin/build-release/trigger`
- Enter version (e.g., 1.0.8) and click "Deploy to GitHub"
- Workflow runs at: `https://github.com/evolvesystems/evolve-desktop/actions`
- Takes ~5-7 minutes to build Windows + Linux
- macOS builds disabled to save GitHub Actions minutes (10x cost)

### Webhook Integration
- Webhook URL: `https://evolvepreneuriq.app/webhook/github/release`
- Webhook secret: Stored in `app_settings_system.github_webhook_secret`
- When GitHub releases are published, webhook auto-populates `app_releases` table
- **Localhost limitation**: Webhook reaches production, not `localhost:8547`

### **ALWAYS CHECK PRODUCTION FIRST**

When debugging webhooks/releases:
1. ✅ Check if request hits localhost or production (webhook URL in DB)
2. ✅ If production domain → releases appear at `https://evolvepreneuriq.app/admin/app-releases`
3. ❌ Don't expect releases to auto-populate in `localhost:8547` (webhook can't reach localhost)

## Critical Tauri 2.0 Configuration

**IMPORTANT**: Tauri 2.0 plugins require empty configuration in `tauri.conf.json`:

```json
{
  "plugins": {}
}
```

**DO NOT** add plugin configurations like:
```json
{
  "plugins": {
    "store": { "path": "..." },      // ❌ WRONG - causes crash
    "notification": { "id": "..." }  // ❌ WRONG - causes crash
  }
}
```

Plugins are initialized in `src-tauri/src/main.rs` only:
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    // etc...
```

## Rust Backend Architecture

### Key Files
- `src-tauri/src/main.rs` - Entry point, Tauri commands, logging setup
- `src-tauri/src/database.rs` - Remote API configuration (no local DB)
- `src-tauri/src/diagnostics.rs` - System diagnostics & WebView2 detection
- `src-tauri/src/sync.rs` - Data sync module (placeholder)

### Logging System
Logs are written to platform-specific directories:
- **Windows**: `%APPDATA%\com.evolveapp.desktop\logs\evolveapp.log`
- **Linux**: `~/.local/share/com.evolveapp.desktop/logs/evolveapp.log`
- **macOS**: `~/Library/Application Support/com.evolveapp.desktop/logs/evolveapp.log`

Daily log rotation with `tracing-appender`. **Critical**: The non-blocking guard must be leaked with `std::mem::forget(guard)` to prevent premature shutdown.

### Diagnostics Module
On startup, the app collects:
- OS name and version
- Architecture (x86_64, aarch64)
- Total/available memory
- WebView2 availability (Windows only)
- App version
- Log file location

Diagnostics are logged locally and optionally sent to EIQ Manager API endpoint (when implemented): `/api/v1/app-diagnostics`

### Tauri Commands
Exposed to frontend via `invoke()`:
- `greet(name: String)` - Test command
- `get_app_version()` - Returns Cargo package version
- `execute_sql(query: String)` - Returns error (no local DB)
- `sync_data(module_id: String)` - Placeholder sync command

## Vue Frontend Architecture

### Plugin System
The app uses a modular plugin architecture allowing dynamic module loading.

**Core Plugin Interface** (`src/core/types/module.ts`):
```typescript
interface ModulePlugin {
  metadata: ModuleMetadata
  install(): Promise<void>
  getMainView(): Component
  getRoutes(): RouteRecordRaw[]
  getSchema(): ModuleSchema
  search?(query: string): Promise<SearchResult[]>
}
```

**Module Registration Flow**:
1. Module implements `ModulePlugin` interface
2. Export as default from `src/modules/[module-name]/index.ts`
3. Register in `src/main.ts`
4. ModuleRegistry adds routes dynamically
5. EventBus enables cross-module communication

**Core Systems**:
- `ModuleRegistry` (`src/core/plugin-system/module-registry.ts`) - Manages module lifecycle
- `ModuleLoader` (`src/core/plugin-system/module-loader.ts`) - Dynamic module loading
- `EventBus` (`src/core/event-bus/index.ts`) - Pub/sub for module communication
- `ServiceRegistry` (`src/core/service-registry/index.ts`) - Dependency injection

### State Management
Uses Pinia stores:
- `stores/auth.ts` - JWT authentication, token refresh, user session
- `stores/app.ts` - Global UI state (sidebar, theme, notifications)

### API Integration
Axios configured in `src/main.ts`:
```typescript
axios.defaults.baseURL = import.meta.env.VITE_API_URL  // http://localhost:8547
```

All API calls use JWT bearer tokens from auth store. Token refresh handled via axios interceptors.

## Known Issues & Troubleshooting

### Windows Build Issues

**Issue**: App crashes silently on Windows
**Diagnosis**: Check Windows Event Viewer (`eventvwr.msc`) → Windows Logs → Application

**Issue**: WebView2 not installed
**Fix**: Download from https://developer.microsoft.com/en-us/microsoft-edge/webview2/
The app checks for WebView2 at startup and logs to `evolveapp.log`

**Issue**: Missing Visual Studio Build Tools
**Fix**: Install "Build Tools for Visual Studio 2022" with "Desktop development with C++" workload

### Linux Build Issues

**Issue**: Missing system dependencies
**Fix**:
```bash
sudo apt-get install -y build-essential libssl-dev libgtk-3-dev \
  libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

### Logging Issues

**Issue**: App only logs first line then crashes
**Cause**: Non-blocking guard dropped too early
**Fix**: Ensure `std::mem::forget(guard)` in `init_logging()` function

### Plugin Configuration Issues

**Issue**: Error like "invalid type: map, expected unit" for plugins
**Cause**: Tauri 2.0 changed plugin config format
**Fix**: Remove all plugin configurations from `tauri.conf.json`, keep only `"plugins": {}`

## Version Management

Version is defined in three places (must match):
- `src-tauri/Cargo.toml` - Rust package version
- `src-tauri/tauri.conf.json` - Tauri app version
- `package.json` - npm package version

**Update all three when bumping version.**

## Database Strategy

**No local database** - all operations via remote API:
- Frontend uses axios to call Symfony API endpoints
- Rust backend does NOT use SQLite
- `database.rs` is a stub that returns API config
- `execute_sql()` Tauri command returns error

**Why no local DB**:
- Simplified architecture
- Real-time data from server
- No sync conflicts
- Requires internet connection (acceptable for business app)

## Module Development

To create a new module:

1. Create directory: `src/modules/[module-name]/`
2. Implement `ModulePlugin` interface
3. Export from `src/modules/[module-name]/index.ts`
4. Add to module registry in `src/main.ts`

**Module Structure**:
```
src/modules/email/
├── index.ts          # Plugin export
├── views/
│   └── EmailView.vue
├── components/
│   ├── EmailList.vue
│   └── EmailCompose.vue
├── store.ts          # Module-specific Pinia store
└── types.ts          # TypeScript types
```

**Module Categories**:
- `communication` - Email, Chat, Video
- `productivity` - Calendar, Tasks, Notes
- `business` - CRM, Invoicing, Projects
- `tools` - File Manager, Settings
- `other` - Custom modules

## EIQ Manager Integration

The desktop app connects to EIQ Manager (Symfony backend):
- **Local Dev**: `http://localhost:8547`
- **Production**: `https://evolvepreneuriq.app`

**Key Endpoints**:
- `POST /api/v1/auth/login` - JWT authentication
- `POST /api/v1/auth/refresh` - Token refresh
- `GET /api/v1/emails` - Email module
- `GET /api/v1/calendar/events` - Calendar module
- `POST /api/v1/app-diagnostics` - Desktop diagnostics (to be implemented)

**Admin Panel URLs**:
- `/admin/app-releases` - Manage desktop releases
- `/admin/build-release/trigger` - Trigger GitHub Actions builds
- `/admin/settings/system` - GitHub token configuration

## Security Notes

- JWT tokens stored in Pinia (sessionStorage)
- GitHub token stored in database: `app_settings_system.github_token`
- Webhook secret: `app_settings_system.github_webhook_secret`
- Never commit tokens to git
- Use `.env` for local configuration (gitignored)

## Testing

Currently no test framework configured. To add tests:
- Frontend: Vitest + Vue Test Utils
- Backend: Cargo test for Rust unit tests
- E2E: Playwright or Cypress

## Documentation

Additional docs in `/docs`:
- `PLUGIN_SYSTEM_ARCHITECTURE.md` - Plugin system design
- `MVP_IMPLEMENTATION_GUIDE.md` - 6-week roadmap
- `SYMFONY_API_IMPLEMENTATION.md` - Backend API code
- `UI_DESIGN_SYSTEM.md` - UI/UX specs

## Development Workflow

1. Make changes to Vue frontend or Rust backend
2. Test locally with `npm run tauri:dev`
3. Commit changes: `git add -A && git commit -m "..."`
4. Push to GitHub: `git push origin master`
5. Trigger build via admin panel for release
6. GitHub Actions builds Windows + Linux installers
7. Webhook auto-populates releases in production database
8. Download installers from GitHub releases

**Current Version**: 1.0.8 (working version with diagnostics)
