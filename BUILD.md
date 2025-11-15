# Building EvolveApp Desktop

## Prerequisites

- **Node.js** 18+ and npm
- **Rust** 1.70+
- Platform-specific requirements:
  - **Windows**: Visual Studio Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `build-essential`, `libgtk-3-dev`, `libwebkit2gtk-4.0-dev`

## Development

```bash
# Install dependencies
npm install

# Run in development mode (browser)
npm run dev

# Run in development mode (desktop app)
npm run tauri:dev
```

## Building for Production

### Build for Current Platform

```bash
# Build for your current operating system
npm run tauri:build
```

Outputs will be in `src-tauri/target/release/bundle/`:
- **Windows**: `.msi` installer in `msi/`
- **Linux**: `.deb` and `.AppImage` in `deb/` and `appimage/`
- **macOS**: `.dmg` and `.app` in `dmg/` and `macos/`

### Build for Specific Platforms

```bash
# Build for Windows
npm run tauri:build:windows

# Build for Linux
npm run tauri:build:linux

# Build for macOS
npm run tauri:build:mac
```

### Create a Release

```bash
# Increments patch version and builds
npm run release
```

This will:
1. Increment version (0.1.0 → 0.1.1)
2. Update `package.json` and `Cargo.toml`
3. Build the application

## Auto-Updater Setup

The app includes auto-update functionality. To enable it:

### 1. Generate Update Keys

```bash
cd src-tauri
cargo install tauri-cli
cargo tauri signer generate -w ~/.tauri/evolveapp.key
```

This creates:
- `~/.tauri/evolveapp.key` (private key - **KEEP SECRET**)
- `~/.tauri/evolveapp.key.pub` (public key)

### 2. Update Configuration

Copy the **public key** from `evolveapp.key.pub` and paste it into `src-tauri/tauri.conf.json`:

```json
{
  "plugins": {
    "updater": {
      "pubkey": "YOUR_PUBLIC_KEY_HERE"
    }
  }
}
```

### 3. Setup Update Server

Host a JSON file at `https://releases.evolveapp.com/{{target}}/{{current_version}}`:

**Example response** for Windows x86_64:
```json
{
  "version": "0.1.1",
  "notes": "Bug fixes and performance improvements",
  "pub_date": "2025-01-15T12:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "SIGNATURE_FROM_BUILD",
      "url": "https://releases.evolveapp.com/EvolveApp_0.1.1_x64.msi.zip"
    }
  }
}
```

The signature file is generated during build at `src-tauri/target/release/bundle/msi/EvolveApp_0.1.0_x64_en-US.msi.zip.sig`

### 4. Build & Sign Release

```bash
# Build with signing
npm run tauri:build

# The signature is automatically generated
# Upload both the installer and .sig file to your server
```

## Configuration

### API Endpoint

Update the production API URL in `src-tauri/tauri.conf.json`:

```json
{
  "plugins": {
    "http": {
      "scope": ["https://api.yourdomain.com/*"]
    }
  }
}
```

### App Metadata

Edit in `src-tauri/tauri.conf.json`:
- `productName`: Display name
- `version`: App version
- `identifier`: Unique app ID (com.company.app)

## Distribution

### Windows

1. Build creates `.msi` installer
2. Users double-click to install
3. App appears in Start Menu
4. Can be uninstalled via Windows Settings

### macOS

1. Build creates `.dmg` and `.app`
2. Users drag `.app` to Applications folder
3. May need to sign and notarize for distribution

### Linux

**Debian/Ubuntu (.deb)**:
```bash
sudo dpkg -i evolve-app_0.1.0_amd64.deb
```

**AppImage (Universal)**:
```bash
chmod +x EvolveApp_0.1.0_amd64.AppImage
./EvolveApp_0.1.0_amd64.AppImage
```

## File Sizes

Typical installer sizes:
- Windows `.msi`: ~15-20 MB
- Linux `.deb`: ~15-20 MB
- Linux `.AppImage`: ~20-25 MB
- macOS `.dmg`: ~15-20 MB

## Troubleshooting

### Build Fails on Linux

```bash
sudo apt-get install -y \
  build-essential \
  libwebkit2gtk-4.0-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### Build Fails on macOS

```bash
xcode-select --install
```

### Updater Not Working

1. Verify `pubkey` in `tauri.conf.json` matches your public key
2. Check update server is accessible
3. Ensure signature file matches the installer
4. Check browser console for errors

## CI/CD Integration

Example GitHub Actions workflow:

```yaml
name: Build Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        platform: [ubuntu-latest, windows-latest, macos-latest]

    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install dependencies
        run: npm install

      - name: Build
        run: npm run tauri:build

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: release-${{ matrix.platform }}
          path: src-tauri/target/release/bundle/
```
