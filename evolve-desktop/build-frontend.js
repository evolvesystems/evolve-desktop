// Creates a minimal dist/index.html for the Tauri build.
// The app loads evolvepreneuriq.app directly via the window URL config,
// so this is just a fallback page.
const fs = require('fs');
fs.mkdirSync('dist', { recursive: true });
fs.writeFileSync('dist/index.html', `<!DOCTYPE html>
<html><head><title>EvolveApp</title></head>
<body style="margin:0;display:flex;align-items:center;justify-content:center;height:100vh;background:#667eea;color:white;font-family:sans-serif">
<div style="text-align:center"><h1>EvolveApp</h1><p>Loading...</p></div>
<script>window.location.href="https://evolvepreneuriq.app"</script>
</body></html>`);
console.log('Created dist/index.html');
