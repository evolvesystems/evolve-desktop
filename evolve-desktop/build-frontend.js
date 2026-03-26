// Creates dist/ directory with:
//   - index.html (placeholder — required by Tauri but not used)
//   - sidebar.html (loaded by the sidebar webview, copied from src-tauri/assets/)
import { mkdirSync, writeFileSync, copyFileSync } from 'fs';

mkdirSync('dist', { recursive: true });

// Index.html is required by Tauri's frontendDist but not actually used
// (the content webview loads the remote URL directly)
writeFileSync('dist/index.html', `<!DOCTYPE html>
<html><head><meta charset="UTF-8"><title>EvolveApp</title></head>
<body><p>Loading...</p></body>
</html>`);

// Copy sidebar.html from source to dist
copyFileSync('src-tauri/assets/sidebar.html', 'dist/sidebar.html');

console.log('Build frontend complete: dist/index.html + dist/sidebar.html');
