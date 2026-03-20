// Creates dist/index.html that redirects to the web app.
// Uses meta refresh (not JS) because Tauri's asset protocol CSP
// may block JS navigation from the local page.
import { mkdirSync, writeFileSync } from 'fs';
mkdirSync('dist', { recursive: true });
writeFileSync('dist/index.html', `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta http-equiv="refresh" content="0;url=https://evolvepreneuriq.app">
  <title>EvolveApp</title>
</head>
<body style="margin:0;display:flex;align-items:center;justify-content:center;height:100vh;background:#667eea;color:white;font-family:sans-serif">
<div style="text-align:center">
  <h1>EvolveApp</h1>
  <p>Loading...</p>
  <p><a href="https://evolvepreneuriq.app" style="color:white">Click here if not redirected</a></p>
</div>
</body>
</html>`);
console.log('Created dist/index.html');
