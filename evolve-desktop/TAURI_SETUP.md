# Tauri Desktop Setup Guide

## 🎯 Current Status

✅ **Completed:**
- Tauri project structure created
- Rust backend with database and sync modules
- Tauri configuration (`tauri.conf.json`)
- Frontend API wrapper (`src/services/tauri.ts`)
- npm packages installed

⏸️ **Blocked - Requires Action:**
- System dependencies installation (needs sudo)
- Tauri CLI installation
- Testing desktop build

## 📋 Next Steps

### Step 1: Install System Dependencies

Run the installation script:

```bash
cd /home/john/sources/evolveapp/evolve-desktop
sudo bash ./install-tauri-deps.sh
```

Or manually:

```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf
```

### Step 2: Install Tauri CLI (via Cargo)

```bash
# Source Cargo environment
. "$HOME/.cargo/env"

# Install Tauri CLI
cargo install tauri-cli --version "^2.0.0"
```

This will take several minutes as it compiles the Tauri CLI from source.

### Step 3: Verify Installation

```bash
# Check Tauri CLI
tauri --version

# Should output: tauri-cli 2.x.x
```

### Step 4: Test Development Build

```bash
# Development mode (opens desktop window)
npm run tauri:dev
```

This will:
1. Start Vite dev server
2. Compile Rust backend
3. Launch desktop application
4. Enable hot-reload for both frontend and backend

### Step 5: Build Production App

```bash
# Production build
npm run tauri:build
```

Output locations:
- **Linux AppImage**: `src-tauri/target/release/bundle/appimage/`
- **Linux DEB**: `src-tauri/target/release/bundle/deb/`
- **Binaries**: `src-tauri/target/release/`

## 🏗️ Tauri Architecture

### Backend (Rust)

**Main Entry Point**: `src-tauri/src/main.rs`
- System tray integration
- Window management
- Command registration
- Plugin initialization

**Database Module**: `src-tauri/src/database.rs`
- SQLite initialization
- Query execution
- CRUD operations
- Located in `$HOME/.local/share/evolveapp/evolveapp.db`

**Sync Module**: `src-tauri/src/sync.rs`
- Module synchronization
- Conflict resolution
- Background sync

### Frontend (Vue)

**Tauri API Service**: `src/services/tauri.ts`
- Command invocation wrapper
- Database operations
- File dialogs
- Notifications
- Browser fallback support

### Configuration

**tauri.conf.json** - Main configuration:
- Window settings (1280x800, resizable)
- Bundle options (icons, metadata)
- Plugin permissions
- Security policies

**Cargo.toml** - Rust dependencies:
- Tauri 2.2.0
- Plugins: shell, store, notification, http, dialog, fs
- Database libraries (to be added)

## 🔧 Development Workflow

### Running in Browser (Development)

```bash
npm run dev
# Opens at http://localhost:5173
# Tauri commands will fallback to browser mode
```

### Running as Desktop App

```bash
npm run tauri:dev
# Opens native window
# Full Tauri API access
# Hot reload enabled
```

### Building for Distribution

```bash
npm run tauri:build
# Creates optimized binaries
# Generates installers
```

## 🎨 Features Implemented

### ✅ Tauri Features

- **System Tray**: Minimize to tray, show/hide window
- **Window Management**: Close to tray instead of exit
- **IPC Commands**:
  - `greet(name)` - Example command
  - `get_app_version()` - Get app version
  - `execute_sql(query)` - Run SQL queries
  - `sync_data(moduleId)` - Sync module data

### ✅ Plugins Configured

- **shell** - Open URLs and files
- **store** - Persistent key-value storage
- **notification** - Native notifications
- **http** - HTTP requests (CORS-free)
- **dialog** - File open/save dialogs
- **fs** - File system access

### 🔒 Security

**Capabilities** (`src-tauri/capabilities/default.json`):
- Window operations allowed
- File system scoped to app data directory
- HTTP scoped to API domains
- Dialog and notification permissions

**CSP**: Currently null (to be configured)

**Asset Protocol**: Enabled for app resources

## 📱 Using Tauri API in Vue

### Example: Call Rust Command

```typescript
import { greet } from '@/services/tauri'

// Call Rust command
const message = await greet('John')
console.log(message) // "Hello, John! Welcome to EvolveApp."
```

### Example: Show Native Notification

```typescript
import { showNotification } from '@/services/tauri'

await showNotification('New Email', 'You have 3 unread messages')
```

### Example: Database Query

```typescript
import { database } from '@/services/tauri'

// Query
const results = await database.query('SELECT * FROM emails LIMIT 10')

// Insert
await database.insert('emails', {
  subject: 'Hello',
  from: 'sender@example.com',
  body: 'Message content'
})
```

### Example: File Dialog

```typescript
import { openFileDialog } from '@/services/tauri'

const files = await openFileDialog({
  multiple: true,
  filters: [{
    name: 'Images',
    extensions: ['png', 'jpg', 'jpeg']
  }]
})

console.log('Selected files:', files)
```

## 🐛 Troubleshooting

### Issue: `cargo: command not found`

**Solution:**
```bash
. "$HOME/.cargo/env"
```

Add to `~/.bashrc` for persistence:
```bash
echo '. "$HOME/.cargo/env"' >> ~/.bashrc
```

### Issue: Missing system dependencies

**Solution:**
Run the installation script:
```bash
sudo bash ./install-tauri-deps.sh
```

### Issue: `tauri: command not found`

**Solution:**
Ensure Cargo bin is in PATH and Tauri CLI is installed:
```bash
. "$HOME/.cargo/env"
cargo install tauri-cli --version "^2.0.0"
```

### Issue: WebKit errors on Linux

**Solution:**
Install WebKit2GTK:
```bash
sudo apt-get install libwebkit2gtk-4.1-dev
```

### Issue: Window won't close

**Expected behavior** - Window hides to system tray instead of closing. Use tray menu "Quit" to exit.

## 📚 Next Steps

After Tauri is working:

1. **Add Icons**: Generate app icons using `tauri icon`
2. **Implement SQLite**: Add `rusqlite` or `sqlx` to Cargo.toml
3. **Create Modules**: Build Email, Calendar, CRM modules
4. **Backend Integration**: Connect to Symfony API
5. **Auto-Updater**: Configure Tauri updater plugin

## 🔗 Resources

- [Tauri Documentation](https://tauri.app/)
- [Tauri Plugins](https://tauri.app/plugin/)
- [Vue + Tauri Guide](https://tauri.app/start/frontend/vite/)
- [Rust Book](https://doc.rust-lang.org/book/)
