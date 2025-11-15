# Application Icons

Place your application icons in this directory:

- `32x32.png` - 32x32 PNG icon
- `128x128.png` - 128x128 PNG icon
- `128x128@2x.png` - 256x256 PNG icon (2x retina)
- `icon.icns` - macOS icon bundle
- `icon.ico` - Windows icon file
- `icon.png` - Linux/system tray icon

You can generate these icons from a single source image using:
```bash
npm install -g @tauri-apps/cli
tauri icon path/to/your/icon.png
```

Or use an online icon generator that supports all required formats.

## Temporary Placeholder

Until icons are generated, Tauri will use default icons.
