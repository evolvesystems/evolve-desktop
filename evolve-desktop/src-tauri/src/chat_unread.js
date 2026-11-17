// EvolveApp 2.4.30: the chat page tells the window how many chats are unread (window.postMessage {type:'chat:unread', total, channels}) within seconds of a message.
// This moves ONLY the sidebar's chat badge (command relay_chat_unread -> sidebar event 'chat-unread'); email and Lab badges are left alone. Runs once per page load.
(function() {
    if (!window.__TAURI_INTERNALS__ || window.__evolve_chat_unread_listener__) return;
    window.__evolve_chat_unread_listener__ = true;
    window.addEventListener('message', function(e) {
        // Same origin only: a frame or window from another site cannot set the badge.
        if (e.origin !== location.origin) return;
        var d = e.data;
        if (!d || d.type !== 'chat:unread') return;
        var t = d.total;
        if (typeof t !== 'number' || !isFinite(t) || t < 0 || Math.floor(t) !== t) return;
        window.__TAURI_INTERNALS__.invoke('relay_chat_unread', {total: t}).catch(function() {});
    });
})();
