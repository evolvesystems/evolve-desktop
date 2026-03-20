// Creates dist/index.html that loads the web app in a full-page iframe.
// This works because Tauri allows iframes to external URLs even when
// direct navigation from the asset protocol is blocked.
import { mkdirSync, writeFileSync } from 'fs';
mkdirSync('dist', { recursive: true });
writeFileSync('dist/index.html', `<!DOCTYPE html>
<html style="height:100%;margin:0;padding:0;overflow:hidden">
<head>
  <meta charset="UTF-8">
  <title>EvolveApp</title>
  <style>
    * { margin: 0; padding: 0; }
    html, body { height: 100%; overflow: hidden; }
    iframe { width: 100%; height: 100%; border: none; }
  </style>
</head>
<body>
  <iframe src="https://evolvepreneuriq.app" allow="camera;microphone;fullscreen;clipboard-write"></iframe>
</body>
</html>`);
console.log('Created dist/index.html');
