// node tests/chat_unread_harness.mjs src/chat_unread.js  - tests the logic of the window-message listener (no browser needed).
import fs from 'node:fs'; import assert from 'node:assert/strict';
const src = fs.readFileSync(process.argv[2], 'utf8');
function setup(origin = 'https://evolvepreneuriq.app', tauri = true) {
    const g = globalThis; g.window = g; delete g.__evolve_chat_unread_listener__;
    g.location = { origin };
    const handlers = []; g.addEventListener = (t, f) => { if (t === 'message') handlers.push(f); };
    const calls = [];
    g.__TAURI_INTERNALS__ = tauri ? { invoke: (c, a) => { calls.push([c, a]); return Promise.resolve(); } } : undefined;
    if (!tauri) delete g.__TAURI_INTERNALS__;
    (0, eval)(src);
    return { handlers, calls, send: (data, origin2) => handlers.forEach((h) => h({ origin: origin2 ?? origin, data })) };
}
let n = 0; const ok = (m) => { n++; console.log('ok', m); };
{ const s = setup(); s.send({ type: 'chat:unread', total: 7, channels: [] }); assert.deepEqual(s.calls, [['relay_chat_unread', { total: 7 }]]); ok('same-origin chat:unread sets the total'); }
{ const s = setup(); s.send({ type: 'chat:unread', total: 0 }); assert.deepEqual(s.calls, [['relay_chat_unread', { total: 0 }]]); ok('zero is a valid total (badge clears)'); }
{ const s = setup(); s.send({ type: 'chat:unread', total: 5 }, 'https://evil.test'); assert.equal(s.calls.length, 0); ok('another origin is ignored'); }
{ const s = setup('https://messager.evolvepreneuriq.app'); s.send({ type: 'chat:unread', total: 3 }); assert.equal(s.calls.length, 1); ok('works on the messenger host origin too'); }
for (const bad of [{ type: 'other', total: 1 }, { type: 'chat:unread' }, { type: 'chat:unread', total: -1 }, { type: 'chat:unread', total: 1.5 }, { type: 'chat:unread', total: '9' }, { type: 'chat:unread', total: NaN }, { type: 'chat:unread', total: Infinity }, null, 'chat:unread', 42]) {
    const s = setup(); s.send(bad); assert.equal(s.calls.length, 0, JSON.stringify(bad)); }
ok('wrong type and unusable totals are ignored');
{ const s = setup(); assert.equal(s.handlers.length, 1); (0, eval)(src); assert.equal(s.handlers.length, 1); ok('installed once per page'); }
{ const s = setup(undefined, false); assert.equal(s.handlers.length, 0); ok('outside the app (no Tauri bridge) nothing is installed'); }
console.log(n, 'passed');
