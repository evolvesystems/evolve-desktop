# Multi-Platform Build & Release Guide

This document explains how to build and release EvolveApp for Windows, macOS, and Linux.

## Automated Builds (GitHub Actions)

The repository includes a GitHub Actions workflow that automatically builds for all platforms.

### How to Trigger a Build

**Option 1: Create a Git Tag (Recommended)**
```bash
git tag -a v1.0.5 -m "Release v1.0.5"
git push origin v1.0.5
```

**Option 2: Manual Workflow Dispatch**
1. Go to your GitHub repository
2. Click "Actions" tab
3. Select "Build Multi-Platform Release"
4. Click "Run workflow"
5. Enter the version number (e.g., `1.0.5`)
6. Click "Run workflow"

### What Gets Built

The workflow builds installers for:

**Windows (x64)**
- `.msi` installer
- `.exe` NSIS installer

**macOS**
- `.dmg` disk image (Intel)
- `.dmg` disk image (Apple Silicon)
- `.app` application bundle

**Linux (x64)**
- `.deb` package (Debian/Ubuntu)
- `.AppImage` universal installer
- `.rpm` package (RedHat/Fedora)

### Where to Find Builds

After the workflow completes:
1. Go to the "Releases" page on GitHub
2. Find your version tag (e.g., `v1.0.5`)
3. Download the installer for your platform

## Local Builds

### Linux Build (Current Platform)
```bash
npm run tauri:build
```

Output locations:
- `src-tauri/target/release/bundle/deb/*.deb`
- `src-tauri/target/release/bundle/appimage/*.AppImage`
- `src-tauri/target/release/bundle/rpm/*.rpm`

### Windows Build (Requires Windows)
```bash
npm run tauri:build:windows
```

### macOS Build (Requires macOS)
```bash
npm run tauri:build:mac
```

## Upload to CDN

After building locally on Linux, upload to CDN:

```bash
cd /home/john/sources/eiq-manager

# Copy installers to Docker-accessible location
mkdir -p var/releases
cp /path/to/*.deb var/releases/
cp /path/to/*.AppImage var/releases/

# Create release and upload
docker-compose exec -T php bin/console app:release:create "1.0.5" \
  --linux-deb="/app/var/releases/EvolveApp_0.1.0_amd64.deb" \
  --linux-appimage="/app/var/releases/EvolveApp_0.1.0_amd64.AppImage" \
  --notes="Release notes here" \
  --stable \
  --latest
```

## Prerequisites

### For Local Linux Builds
- Node.js 20+
- Rust (install via `rustup`)
- System dependencies:
  ```bash
  sudo apt-get install -y libwebkit2gtk-4.1-dev \
    libappindicator3-dev librsvg2-dev patchelf
  ```

### For Windows Builds
- Visual Studio with C++ build tools
- Node.js 20+
- Rust

### For macOS Builds
- Xcode Command Line Tools
- Node.js 20+
- Rust

## Code Signing (Optional)

To sign your builds, add these secrets to your GitHub repository:

1. Go to Settings → Secrets and variables → Actions
2. Add:
   - `TAURI_SIGNING_PRIVATE_KEY`
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

Generate signing keys:
```bash
npm run tauri signer generate
```

## Troubleshooting

### Build Fails on GitHub Actions
- Check the Actions logs for specific errors
- Ensure all dependencies are correctly specified in `package.json`
- Verify Tauri configuration in `src-tauri/tauri.conf.json`

### Icons Missing
Icons must exist at:
- `src-tauri/icons/32x32.png`
- `src-tauri/icons/128x128.png`
- `src-tauri/icons/icon.png`

### CDN Upload Fails
- Verify DigitalOcean Spaces credentials at http://localhost:8547/admin/app-settings
- Check that bucket permissions allow uploads
- Ensure files are in Docker-accessible location (`/app/var/releases/`)
