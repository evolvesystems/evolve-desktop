# EvolveMailPro - Deployment & Distribution Guide

**Product**: EvolveMailPro Desktop Email Client
**Platforms**: Windows, macOS, Linux
**Distribution Model**: Native Installers + Auto-Update
**Date**: 2025-10-30

---

## 🎯 Overview

EvolveMailPro is distributed as **native desktop applications** for Windows, macOS, and Linux. Users download and install it like any other desktop application (think Microsoft Office, Slack, or Zoom).

### Distribution Methods

1. **Direct Download** (Primary)
   - Download installers from website
   - Native installers for each platform
   - Automatic updates built-in

2. **App Stores** (Optional - Phase 2)
   - Microsoft Store (Windows)
   - Mac App Store (macOS)
   - Snap Store / Flathub (Linux)

3. **Enterprise Distribution** (Optional - Phase 3)
   - MSI packages for Windows domain deployment
   - MDM/Intune deployment
   - Custom enterprise installers

---

## 📦 Platform-Specific Installers

### Windows

**Installer Type**: `.msi` (Windows Installer)

**Features**:
- Silent install support (`/quiet` flag)
- Add to Start Menu
- Desktop shortcut (optional)
- Add to Windows startup (optional)
- Automatic uninstaller in Control Panel
- Code signing (prevents "Unknown Publisher" warnings)

**Build Output**:
```
src-tauri/target/release/bundle/msi/
└── EvolveMailPro_1.0.0_x64_en-US.msi  (5-10 MB)
```

**Installation Path**:
```
C:\Program Files\EvolveMailPro\
├── EvolveMailPro.exe
├── resources\
└── uninstall.exe
```

**User Experience**:
```
1. User downloads EvolveMailPro_Setup.msi from website
2. Double-click installer
3. Windows shows "Do you want to allow this app?" → Click Yes
4. Installation wizard appears:
   - Welcome screen
   - License agreement
   - Installation location (default: Program Files)
   - Shortcut options (Start Menu, Desktop)
   - Install button
5. Installation takes 10-30 seconds
6. Launch EvolveMailPro checkbox
7. Done! App launches
```

**Command-Line Install** (Silent):
```bash
# Silent install
msiexec /i EvolveMailPro_1.0.0_x64_en-US.msi /quiet

# Silent install with desktop shortcut
msiexec /i EvolveMailPro_1.0.0_x64_en-US.msi /quiet ADDDESKTOPSHORTCUT=1
```

---

### macOS

**Installer Type**: `.dmg` (Disk Image) or `.pkg` (Package)

**Features**:
- Drag-and-drop installation
- Native macOS app bundle
- Code signing (prevents "Unidentified Developer" warnings)
- Notarization (required for macOS 10.15+)
- Automatic updates

**Build Output**:
```
src-tauri/target/release/bundle/dmg/
└── EvolveMailPro_1.0.0_x64.dmg  (5-10 MB)
```

**Installation Path**:
```
/Applications/EvolveMailPro.app/
└── Contents/
    ├── MacOS/
    │   └── EvolveMailPro
    ├── Resources/
    └── Info.plist
```

**User Experience (DMG)**:
```
1. User downloads EvolveMailPro.dmg from website
2. Double-click DMG file
3. Finder opens showing:
   - EvolveMailPro.app icon
   - Applications folder shortcut
   - "Drag to install" instruction
4. User drags EvolveMailPro.app to Applications
5. Eject DMG
6. Open Applications folder
7. Double-click EvolveMailPro.app
8. macOS shows "Opening for first time" confirmation → Click Open
9. Done! App launches
```

**User Experience (PKG)**:
```
1. User downloads EvolveMailPro.pkg from website
2. Double-click PKG file
3. macOS Installer opens:
   - Introduction
   - License
   - Installation location
   - Install button
4. User clicks Install
5. macOS asks for password (admin privileges)
6. Installation completes
7. Done! App available in Applications
```

---

### Linux

**Installer Types**:
- `.deb` (Debian/Ubuntu)
- `.rpm` (Fedora/Red Hat)
- `.AppImage` (Universal)
- `Snap` package (optional)
- `Flatpak` (optional)

**Build Output**:
```
src-tauri/target/release/bundle/
├── deb/
│   └── evolve-mail-pro_1.0.0_amd64.deb  (5-10 MB)
├── rpm/
│   └── evolve-mail-pro-1.0.0-1.x86_64.rpm
└── appimage/
    └── EvolveMailPro_1.0.0_amd64.AppImage
```

**Installation Path**:
```
/usr/bin/evolve-mail-pro
/usr/share/applications/evolve-mail-pro.desktop
/usr/share/icons/hicolor/*/apps/evolve-mail-pro.png
```

**User Experience (.deb - Ubuntu/Debian)**:
```
1. User downloads evolve-mail-pro.deb from website
2. Double-click DEB file
3. Ubuntu Software Center opens
4. Click "Install" button
5. Ubuntu asks for password
6. Installation takes 5-10 seconds
7. Done! App appears in Applications menu
```

**Command-Line Install** (Debian/Ubuntu):
```bash
# Install
sudo dpkg -i evolve-mail-pro_1.0.0_amd64.deb

# Or using apt
sudo apt install ./evolve-mail-pro_1.0.0_amd64.deb
```

**Command-Line Install** (Fedora/Red Hat):
```bash
# Install
sudo rpm -i evolve-mail-pro-1.0.0-1.x86_64.rpm

# Or using dnf
sudo dnf install evolve-mail-pro-1.0.0-1.x86_64.rpm
```

**AppImage** (Universal - No Install):
```bash
# Make executable
chmod +x EvolveMailPro_1.0.0_amd64.AppImage

# Run directly (no installation needed!)
./EvolveMailPro_1.0.0_amd64.AppImage
```

---

## 🌐 Distribution Workflow

### 1. Build Process

```bash
# On build server (or developer machine)

# Build for Windows (on Windows or cross-compile)
npm run tauri build -- --target x86_64-pc-windows-msvc

# Build for macOS (on Mac)
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target aarch64-apple-darwin  # Apple Silicon

# Build for Linux (on Linux)
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

**Output**: Platform-specific installers in `src-tauri/target/release/bundle/`

### 2. Code Signing (Important!)

**Why?**
- Windows: Prevents "Unknown Publisher" warnings
- macOS: Required to bypass Gatekeeper
- Builds trust with users

**Windows Code Signing**:
```bash
# Using signtool (requires code signing certificate)
signtool sign /f certificate.pfx /p password /tr http://timestamp.digicert.com /td sha256 /fd sha256 EvolveMailPro.msi
```

**macOS Code Signing & Notarization**:
```bash
# Sign the app
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" EvolveMailPro.app

# Notarize with Apple
xcrun notarytool submit EvolveMailPro.dmg --apple-id your@email.com --password app-specific-password --team-id TEAM_ID

# Staple notarization
xcrun stapler staple EvolveMailPro.dmg
```

### 3. Upload to Distribution Server

**Option A: Direct Download (Website)**
```
https://evolvemailpro.com/download
├── EvolveMailPro-Setup-Windows.msi
├── EvolveMailPro-macOS.dmg
├── EvolveMailPro-Linux.deb
└── EvolveMailPro-Linux.AppImage
```

**Option B: CDN Distribution**
```
https://cdn.evolvemailpro.com/releases/
├── v1.0.0/
│   ├── EvolveMailPro_1.0.0_x64_en-US.msi
│   ├── EvolveMailPro_1.0.0_x64.dmg
│   └── evolve-mail-pro_1.0.0_amd64.deb
└── latest.json  # For auto-update checks
```

### 4. Download Page (Website)

```html
<!-- Example download page -->
<section class="download">
  <h1>Download EvolveMailPro</h1>

  <!-- Auto-detect OS and show appropriate button -->
  <button class="primary" id="download-button">
    Download for Windows
  </button>

  <div class="other-platforms">
    <a href="/downloads/windows">Windows (64-bit)</a>
    <a href="/downloads/mac">macOS (Intel & Apple Silicon)</a>
    <a href="/downloads/linux">Linux (.deb, .rpm, AppImage)</a>
  </div>

  <p class="version">Version 1.0.0 | Released: 2025-10-30</p>
</section>
```

**Smart OS Detection**:
```javascript
// Auto-detect user's OS and show appropriate download
const os = navigator.platform.toLowerCase();
if (os.includes('win')) {
  // Show Windows download
} else if (os.includes('mac')) {
  // Show macOS download
} else if (os.includes('linux')) {
  // Show Linux download
}
```

---

## 🔄 Automatic Updates

### How It Works

**Update Flow**:
```
1. App checks for updates on startup (or every 24 hours)
   ↓
2. Queries update server: https://cdn.evolvemailpro.com/latest.json
   ↓
3. Compares current version with latest version
   ↓
4. If newer version available:
   - Shows notification: "Update available: v1.1.0"
   - User clicks "Download & Install"
   ↓
5. App downloads new installer in background
   ↓
6. When ready, shows: "Restart to update"
   ↓
7. User clicks "Restart"
   ↓
8. App closes, installer runs, app reopens with new version
```

### Update Server Response

**Endpoint**: `https://cdn.evolvemailpro.com/latest.json`

```json
{
  "version": "1.1.0",
  "releaseDate": "2025-11-15",
  "platforms": {
    "windows": {
      "url": "https://cdn.evolvemailpro.com/releases/v1.1.0/EvolveMailPro_1.1.0_x64_en-US.msi",
      "signature": "sha256:abc123...",
      "size": 8456123
    },
    "darwin": {
      "url": "https://cdn.evolvemailpro.com/releases/v1.1.0/EvolveMailPro_1.1.0_x64.dmg",
      "signature": "sha256:def456...",
      "size": 9123456
    },
    "linux": {
      "url": "https://cdn.evolvemailpro.com/releases/v1.1.0/evolve-mail-pro_1.1.0_amd64.deb",
      "signature": "sha256:ghi789...",
      "size": 7654321
    }
  },
  "releaseNotes": "### What's New\n- Added dark mode\n- Improved sync performance\n- Fixed calendar bugs",
  "minimumVersion": "1.0.0",
  "critical": false
}
```

### Update Implementation (Tauri)

**Rust Backend** (`src-tauri/src/main.rs`):
```rust
use tauri::updater::builder;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let updater = handle.updater().check().await;

                match updater {
                    Ok(Some(update)) => {
                        println!("Update available: {}", update.version);

                        // Download and install update
                        update.download_and_install().await.unwrap();
                    }
                    Ok(None) => println!("App is up to date"),
                    Err(e) => println!("Failed to check for updates: {}", e),
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Vue Frontend** (`src/components/UpdateNotification.vue`):
```vue
<template>
  <div v-if="updateAvailable" class="alert alert-info">
    <div>
      <svg>...</svg>
      <span>Update available: v{{ updateInfo.version }}</span>
    </div>
    <div class="flex-none">
      <button class="btn btn-sm" @click="installUpdate">
        Update Now
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { checkForUpdates, installUpdate } from '@/services/updater';

const updateAvailable = ref(false);
const updateInfo = ref(null);

onMounted(async () => {
  const info = await checkForUpdates();
  if (info) {
    updateAvailable.value = true;
    updateInfo.value = info;
  }
});
</script>
```

---

## 📋 Release Process

### 1. Version Bump

```bash
# Update version in multiple files
# package.json
{
  "version": "1.1.0"
}

# src-tauri/Cargo.toml
[package]
version = "1.1.0"

# src-tauri/tauri.conf.json
{
  "package": {
    "version": "1.1.0"
  }
}
```

### 2. Build All Platforms

```bash
# Windows (on Windows machine or CI)
npm run tauri build -- --target x86_64-pc-windows-msvc

# macOS (on Mac machine or CI)
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target aarch64-apple-darwin

# Linux (on Linux machine or CI)
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### 3. Sign & Notarize

```bash
# Windows: Sign with code signing certificate
signtool sign /f cert.pfx /p password EvolveMailPro.msi

# macOS: Sign and notarize
codesign --sign "Developer ID" EvolveMailPro.app
xcrun notarytool submit EvolveMailPro.dmg ...
```

### 4. Upload to CDN

```bash
# Upload to S3/DigitalOcean Spaces/Cloudflare R2
aws s3 cp src-tauri/target/release/bundle/ \
  s3://evolvemailpro-releases/v1.1.0/ \
  --recursive

# Update latest.json
cat > latest.json <<EOF
{
  "version": "1.1.0",
  "releaseDate": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  ...
}
EOF

aws s3 cp latest.json s3://evolvemailpro-releases/latest.json
```

### 5. Create GitHub Release

```bash
# Tag release
git tag v1.1.0
git push origin v1.1.0

# Create GitHub release with installers attached
gh release create v1.1.0 \
  --title "EvolveMailPro v1.1.0" \
  --notes "Release notes here..." \
  src-tauri/target/release/bundle/**/*
```

### 6. Update Website

```bash
# Update download page with new version
# Update changelog
# Publish blog post (optional)
```

---

## 🏪 App Store Distribution (Optional)

### Microsoft Store (Windows)

**Pros**:
- Trusted distribution channel
- Automatic updates via Windows Store
- Increased discoverability

**Cons**:
- Submission process (1-3 days review)
- Annual developer fee ($19)
- Microsoft Store requirements

**Process**:
1. Create Microsoft Partner Center account
2. Prepare MSIX package
3. Submit for certification
4. Wait for approval
5. App appears in Microsoft Store

### Mac App Store

**Pros**:
- Trusted by macOS users
- Automatic updates
- Apple's marketing

**Cons**:
- Strict sandboxing requirements
- Annual developer fee ($99)
- 1-2 week review process

**Process**:
1. Apple Developer account required
2. App must be sandboxed
3. Submit via Xcode
4. Wait for review
5. App appears in Mac App Store

### Linux App Stores

**Snap Store**:
```bash
# Build snap package
snapcraft

# Upload to Snap Store
snapcraft upload evolve-mail-pro_1.0.0_amd64.snap
```

**Flathub**:
```bash
# Create Flatpak manifest
# Submit PR to Flathub repository
# Wait for approval
```

---

## 💼 Enterprise Deployment

### Windows Domain (GPO)

**MSI Deployment via Group Policy**:
```
1. Copy EvolveMailPro.msi to network share
2. Open Group Policy Management
3. Create new GPO
4. Computer Configuration > Software Installation
5. Add new package (point to MSI)
6. Deploy to target computers
7. App installs on next reboot/login
```

### Intune/MDM (Modern Management)

**Upload to Intune**:
```
1. Convert MSI to .intunewin format
2. Upload to Intune portal
3. Create assignment (users/groups)
4. Set install behavior (system/user context)
5. Deploy
```

### Silent Install Scripts

**Windows (PowerShell)**:
```powershell
# Download and install silently
$url = "https://cdn.evolvemailpro.com/releases/latest/EvolveMailPro.msi"
$output = "$env:TEMP\EvolveMailPro.msi"

Invoke-WebRequest -Uri $url -OutFile $output
Start-Process msiexec.exe -ArgumentList "/i $output /quiet" -Wait
Remove-Item $output
```

**macOS (Shell)**:
```bash
# Download and install
curl -L https://cdn.evolvemailpro.com/releases/latest/EvolveMailPro.dmg -o /tmp/EvolveMailPro.dmg
hdiutil attach /tmp/EvolveMailPro.dmg
cp -R /Volumes/EvolveMailPro/EvolveMailPro.app /Applications/
hdiutil detach /Volumes/EvolveMailPro
rm /tmp/EvolveMailPro.dmg
```

---

## 📊 Installation Analytics

### Track Installs

**Backend Endpoint**: Track first-launch
```
POST https://api.evolvemailpro.com/v1/analytics/install
{
  "version": "1.0.0",
  "platform": "windows",
  "architecture": "x64",
  "installDate": "2025-10-30T12:00:00Z"
}
```

**Metrics to Track**:
- Total installs per platform
- Active installations
- Update adoption rate
- Version distribution
- Geographic distribution

---

## ✅ User Installation Checklist

### What Users Need

**Requirements**:
- ✅ Internet connection (for download)
- ✅ 100 MB free disk space
- ✅ Admin/sudo rights (for installation)
- ✅ Platform credentials (eiq-manager account)

**Supported Systems**:
- Windows 10/11 (64-bit)
- macOS 10.15 Catalina or later
- Linux (Ubuntu 20.04+, Fedora 35+, etc.)

### First Launch

```
1. User installs app
2. App launches for first time
3. Shows welcome screen
4. User clicks "Sign In"
5. Enter platform credentials
6. Two-factor authentication (if enabled)
7. App connects to API
8. Initial sync begins (downloads emails)
9. Done! Ready to use
```

---

## 🎯 Summary

### For End Users:
1. **Download** installer from website
2. **Run** installer (double-click)
3. **Install** (click through wizard)
4. **Launch** EvolveMailPro
5. **Sign in** with credentials
6. **Done!** Start using

### For IT Admins:
1. **Download** MSI/PKG/DEB
2. **Deploy** via GPO/Intune/APT
3. **Configure** (optional policies)
4. **Done!** Users auto-receive

### For Developers:
1. **Build** for all platforms
2. **Sign** with certificates
3. **Upload** to CDN
4. **Update** latest.json
5. **Done!** Auto-updates work

---

**Distribution Model**: Native Desktop Application
**Update Mechanism**: Automatic (built-in)
**Installer Size**: 5-10 MB per platform
**Installation Time**: 10-30 seconds
**User Experience**: Just like Slack, Zoom, or Microsoft Teams

---

**Last Updated**: 2025-10-30
**Status**: Ready for Implementation
