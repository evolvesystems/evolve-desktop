/**
 * EvolveApp Desktop Sidebar
 * Injected by Tauri on every page load.
 * Loads tab config from EIQ API when available, falls back to defaults.
 *
 * Architecture:
 *   renderSidebarTabs() - rebuilds only the tab buttons in the sidebar
 *   renderConfigPanel() - builds the full config panel (called once on init)
 *   updateConfigList()  - rebuilds the tab rows inside the config panel without destroying it
 */
(function() {
    if (document.getElementById('desktop-sidebar')) return;

    var APP = 'https://evolvepreneuriq.app';
    var DEFAULT_TABS = [
        {id:'dashboard', label:'Home', icon:'squares-2x2', url:APP+'/dashboard', enabled:true},
        {id:'email', label:'Mail', icon:'envelope', url:APP+'/email-manager', enabled:true},
        {id:'chat', label:'Chat', icon:'chat-bubble-left-right', url:APP+'/chat', enabled:true},
        {id:'crm', label:'CRM', icon:'users', url:APP+'/crm-marketing/contacts', enabled:true},
        {id:'calendar', label:'EvolveMeet', icon:'calendar', url:APP+'/evolvemeet', enabled:true},
        {id:'docs', label:'Docs', icon:'document-text', url:APP+'/evolve-docs', enabled:true},
        {id:'va', label:'VA', icon:'cpu-chip', url:APP+'/workspace', enabled:false},
        {id:'books', label:'Books', icon:'book-open', url:APP+'/books/my-books', enabled:false},
        {id:'epanel', label:'E-Panel', icon:'server-stack', url:APP+'/spanel-hub', enabled:true},
        {id:'updates', label:'Updates', icon:'custom', emoji:'\u2B07\uFE0F', url:APP+'/download', enabled:false}
    ];

    var ICON_SVG = {
        'squares-2x2': '<path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25A2.25 2.25 0 0 1 13.5 18v-2.25Z"/>',
        'envelope': '<path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75"/>',
        'chat-bubble-left-right': '<path stroke-linecap="round" stroke-linejoin="round" d="M20.25 8.511c.884.284 1.5 1.128 1.5 2.097v4.286c0 1.136-.847 2.1-1.98 2.193-.34.027-.68.052-1.02.072v3.091l-3-3c-1.354 0-2.694-.055-4.02-.163a2.115 2.115 0 0 1-.825-.242m9.345-8.334a2.126 2.126 0 0 0-.476-.095 48.64 48.64 0 0 0-8.048 0c-1.131.094-1.976 1.057-1.976 2.192v4.286c0 .837.46 1.58 1.155 1.951m9.345-8.334V6.637c0-1.621-1.152-3.026-2.76-3.235A48.455 48.455 0 0 0 11.25 3c-2.115 0-4.198.137-6.24.402-1.608.209-2.76 1.614-2.76 3.235v6.226c0 1.621 1.152 3.026 2.76 3.235.577.075 1.157.14 1.74.194V21l4.155-4.155"/>',
        'users': '<path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 0 0 2.625.372 9.337 9.337 0 0 0 4.121-.952 4.125 4.125 0 0 0-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 0 1 8.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0 1 11.964-3.07M12 6.375a3.375 3.375 0 1 1-6.75 0 3.375 3.375 0 0 1 6.75 0Zm8.25 2.25a2.625 2.625 0 1 1-5.25 0 2.625 2.625 0 0 1 5.25 0Z"/>',
        'calendar': '<path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5"/>',
        'document-text': '<path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z"/>',
        'cpu-chip': '<path stroke-linecap="round" stroke-linejoin="round" d="M8.25 3v1.5M4.5 8.25H3m18 0h-1.5M4.5 12H3m18 0h-1.5m-15 3.75H3m18 0h-1.5M8.25 19.5V21M12 3v1.5m0 15V21m3.75-18v1.5m0 15V21m-9-1.5h10.5a2.25 2.25 0 0 0 2.25-2.25V6.75a2.25 2.25 0 0 0-2.25-2.25H6.75A2.25 2.25 0 0 0 4.5 6.75v10.5a2.25 2.25 0 0 0 2.25 2.25Zm.75-12h9v9h-9v-9Z"/>',
        'book-open': '<path stroke-linecap="round" stroke-linejoin="round" d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25"/>',
        'server-stack': '<path stroke-linecap="round" stroke-linejoin="round" d="M5.25 14.25h13.5m-13.5 0a3 3 0 0 1-3-3m3 3a3 3 0 1 0 0 6h13.5a3 3 0 1 0 0-6m-16.5-3a3 3 0 0 1 3-3h13.5a3 3 0 0 1 3 3m-19.5 0a4.5 4.5 0 0 1 .9-2.7L5.737 5.1a3.375 3.375 0 0 1 2.7-1.35h7.126c1.062 0 2.062.5 2.7 1.35l2.587 3.45a4.5 4.5 0 0 1 .9 2.7m0 0a3 3 0 0 1-3 3m0 3h.008v.008h-.008v-.008Zm0-6h.008v.008h-.008v-.008Zm-3 6h.008v.008h-.008v-.008Zm0-6h.008v.008h-.008v-.008Z"/>'
    };

    var EMOJI_ICONS = [
        '\uD83C\uDFE0','\uD83D\uDCE7','\uD83D\uDCAC','\uD83D\uDC65','\uD83D\uDCC5',
        '\uD83D\uDCC4','\uD83E\uDD16','\uD83D\uDCDA','\uD83D\uDDA5\uFE0F','\u2699\uFE0F',
        '\uD83D\uDD17','\uD83D\uDCCA','\uD83C\uDFAF','\uD83D\uDCCB','\uD83D\uDDC2\uFE0F',
        '\uD83D\uDD0D','\uD83D\uDCB0','\uD83D\uDED2','\uD83C\uDF93','\uD83D\uDCE1',
        '\uD83D\uDD14','\uD83C\uDFE2','\u2709\uFE0F','\uD83C\uDFA4'
    ];

    // --- State ---
    // Use cached tabs from Tauri (injected before this script) or fall back to defaults
    var tabs = (window.__EVOLVEAPP_CACHED_TABS__ && Array.isArray(window.__EVOLVEAPP_CACHED_TABS__))
        ? window.__EVOLVEAPP_CACHED_TABS__ : DEFAULT_TABS;
    var tabsLoadedFromServer = !!window.__EVOLVEAPP_CACHED_TABS__;  // cached counts as loaded
    var configOpen = false;
    var isEiqDomain = window.location.hostname.indexOf('evolvepreneuriq.app') !== -1
        && window.location.hostname.indexOf('dashboard.') === -1;
    // For API calls: use absolute URL if on external domain, relative if on EIQ
    var onEiq = isEiqDomain;
    var apiBase = isEiqDomain ? '' : APP;

    // --- DOM references (set once during build) ---
    var sidebarEl = null;
    var tabContainer = null;       // container inside sidebar that holds tab buttons
    var configPanelEl = null;      // the config slide-out panel
    var configListEl = null;       // the scrollable list of tab rows inside config panel
    var settingsBtnEl = null;      // gear button in sidebar

    // --- Helpers ---
    function makeSvg(name, size) {
        var s = size || 20;
        return '<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" style="width:'+s+'px;height:'+s+'px">' + (ICON_SVG[name] || ICON_SVG['squares-2x2']) + '</svg>';
    }

    function getUserRole() {
        return document.body.dataset.userRole || 'user';
    }

    function getRoleLevel(role) {
        var levels = {all:0, user:1, admin:2, super_admin:3};
        return levels[role] || 0;
    }

    function getEnabledTabs() {
        var myLevel = getRoleLevel(getUserRole());
        return tabs.filter(function(t) {
            if (!t.enabled) return false;
            return myLevel >= getRoleLevel(t.role || 'all');
        });
    }

    /**
     * Detect if a fetch response is an HTML redirect to login page rather than JSON.
     * Symfony often returns a 302 redirect (which fetch follows) resulting in 200 with HTML.
     */
    function isHtmlLoginRedirect(resp) {
        var ct = resp.headers.get('content-type') || '';
        return ct.indexOf('text/html') !== -1;
    }

    // =========================================================================
    //  SIDEBAR RENDERING
    // =========================================================================

    /**
     * Build the sidebar shell once (avatar, spacer, bottom buttons, gear).
     * Tab buttons go into a dedicated container that renderSidebarTabs() manages.
     */
    function buildSidebar() {
        // Remove any existing sidebar or placeholder
        var existing = document.getElementById('desktop-sidebar');
        if (existing) existing.remove();
        var placeholder = document.getElementById('desktop-sidebar-placeholder');
        if (placeholder) placeholder.remove();

        sidebarEl = document.createElement('div');
        sidebarEl.id = 'desktop-sidebar';
        sidebarEl.style.cssText = 'position:fixed;top:0;left:0;bottom:0;width:56px;background:oklch(0.21 0.006 285.88);z-index:99998;display:flex;flex-direction:column;align-items:center;padding:8px 0;gap:2px;overflow-y:auto;font-family:system-ui';

        // Avatar / logo
        var logo = document.createElement('a');
        logo.href = APP + '/dashboard';
        logo.title = 'Dashboard';
        logo.style.cssText = 'width:40px;height:40px;display:flex;align-items:center;justify-content:center;margin-bottom:8px;border-radius:50%;overflow:hidden;text-decoration:none;flex-shrink:0';
        var navAvatar = document.querySelector('.navbar .avatar img');
        if (navAvatar) {
            var img = document.createElement('img');
            img.src = navAvatar.src;
            img.style.cssText = 'width:100%;height:100%;object-fit:cover;border-radius:50%';
            logo.appendChild(img);
        } else {
            logo.style.cssText += ';background:oklch(0.65 0.24 16.57);color:#fff;font-weight:700;font-size:16px';
            logo.textContent = 'E';
        }
        sidebarEl.appendChild(logo);

        // Tab button container
        tabContainer = document.createElement('div');
        tabContainer.id = 'sidebar-tab-container';
        tabContainer.style.cssText = 'display:flex;flex-direction:column;align-items:center;gap:2px;width:100%';
        sidebarEl.appendChild(tabContainer);

        // Spacer
        var spacer = document.createElement('div');
        spacer.style.flex = '1';
        sidebarEl.appendChild(spacer);

        // Back button
        sidebarEl.appendChild(makeBottomButton('\u2190', 'Back', function() { history.back(); }));

        // Refresh button
        sidebarEl.appendChild(makeBottomButton('\u21BB', 'Refresh', function() { location.reload(); }));

        // Copy URL button
        var copyBtn = document.createElement('button');
        copyBtn.title = 'Copy page URL';
        copyBtn.style.cssText = 'width:44px;height:44px;display:flex;flex-direction:column;align-items:center;justify-content:center;border-radius:10px;color:rgba(255,255,255,0.3);cursor:pointer;background:none;border:none;font-size:9px;gap:1px;transition:all 0.15s';
        copyBtn.innerHTML = '<span style="font-size:14px">\uD83D\uDD17</span><span style="font-size:8px;opacity:0.6">Copy</span>';
        copyBtn.onmouseenter = function() { this.style.color = 'rgba(255,255,255,0.8)'; };
        copyBtn.onmouseleave = function() { this.style.color = 'rgba(255,255,255,0.3)'; };
        copyBtn.onclick = function() {
            navigator.clipboard.writeText(window.location.href).then(function() {
                copyBtn.querySelector('span:last-child').textContent = 'Copied!';
                copyBtn.style.color = '#22c55e';
                setTimeout(function() {
                    copyBtn.querySelector('span:last-child').textContent = 'Copy';
                    copyBtn.style.color = 'rgba(255,255,255,0.3)';
                }, 2000);
            });
        };
        sidebarEl.appendChild(copyBtn);

        // Info button
        var infoBtn = document.createElement('button');
        infoBtn.id = 'sidebar-info-btn';
        infoBtn.title = 'App Info';
        infoBtn.style.cssText = 'width:44px;height:44px;display:flex;flex-direction:column;align-items:center;justify-content:center;border-radius:10px;color:rgba(255,255,255,0.3);cursor:pointer;background:none;border:none;font-size:9px;gap:1px;transition:all 0.15s';
        infoBtn.innerHTML = '<span style="font-size:16px">\u24D8</span><span id="sidebar-version-label" style="font-size:8px;opacity:0.6">...</span>';
        infoBtn.onmouseenter = function() { this.style.color = 'rgba(255,255,255,0.8)'; };
        infoBtn.onmouseleave = function() { this.style.color = 'rgba(255,255,255,0.3)'; };
        infoBtn.onclick = function() {
            var popup = document.getElementById('sidebar-info-popup');
            if (popup) { popup.style.display = popup.style.display === 'none' ? 'block' : 'none'; }
        };
        sidebarEl.appendChild(infoBtn);

        // Info popup (appended to body, not sidebar)
        buildInfoPopup();

        // Settings gear
        settingsBtnEl = document.createElement('button');
        settingsBtnEl.title = 'Configure Sidebar';
        settingsBtnEl.style.cssText = 'width:44px;height:44px;display:flex;align-items:center;justify-content:center;border-radius:10px;color:rgba(255,255,255,0.3);cursor:pointer;margin-bottom:8px;background:none;border:none;transition:all 0.15s';
        settingsBtnEl.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" style="width:20px;height:20px"><path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.431.992a7.723 7.723 0 0 1 0 .255c-.007.378.138.75.43.991l1.004.827c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z"/><circle cx="12" cy="12" r="3"/></svg>';
        settingsBtnEl.onmouseenter = function() { this.style.color = 'rgba(255,255,255,0.8)'; };
        settingsBtnEl.onmouseleave = function() { if (!configOpen) this.style.color = 'rgba(255,255,255,0.3)'; };
        settingsBtnEl.onclick = function() { toggleConfigPanel(); };
        sidebarEl.appendChild(settingsBtnEl);

        document.body.appendChild(sidebarEl);

        // Render initial tab buttons
        renderSidebarTabs();

        // Shift page content
        document.documentElement.style.setProperty('--sidebar-width', '56px');
        document.body.style.marginLeft = '56px';
        document.body.style.width = 'calc(100% - 56px)';
        document.body.style.overflowX = 'hidden';

        // Fix fixed/sticky elements
        var style = document.createElement('style');
        style.textContent = '.navbar,.sticky,.fixed-top,[class*="sticky"]{left:56px !important;width:calc(100% - 56px) !important}.ws-fullpage{left:var(--sidebar-width,0px) !important}';
        document.head.appendChild(style);
    }

    function makeBottomButton(text, title, onclick) {
        var btn = document.createElement('button');
        btn.title = title;
        btn.onclick = onclick;
        btn.style.cssText = 'width:44px;height:44px;display:flex;align-items:center;justify-content:center;border-radius:10px;color:rgba(255,255,255,0.3);cursor:pointer;background:none;border:none;font-size:18px;transition:all 0.15s';
        btn.textContent = text;
        btn.onmouseenter = function() { this.style.color = '#fff'; this.style.background = 'oklch(0.30 0.006 285.88)'; };
        btn.onmouseleave = function() { this.style.color = 'rgba(255,255,255,0.3)'; this.style.background = 'none'; };
        return btn;
    }

    function buildInfoPopup() {
        var existing = document.getElementById('sidebar-info-popup');
        if (existing) existing.remove();

        var infoPopup = document.createElement('div');
        infoPopup.id = 'sidebar-info-popup';
        infoPopup.style.cssText = 'position:fixed;bottom:60px;left:60px;width:220px;background:oklch(0.22 0.006 285.88);border:1px solid rgba(255,255,255,0.1);border-radius:12px;padding:14px;z-index:99999;box-shadow:0 8px 24px rgba(0,0,0,0.4);color:#fff;display:none';
        infoPopup.innerHTML = '<div style="font-size:13px;font-weight:600;margin-bottom:8px">EvolveApp Desktop</div>'
            + '<div style="font-size:11px;color:rgba(255,255,255,0.5);margin-bottom:4px">Version: <span id="info-current-ver">checking...</span></div>'
            + '<div style="font-size:11px;color:rgba(255,255,255,0.5);margin-bottom:10px">Latest: <span id="info-latest-ver">checking...</span></div>'
            + '<div id="info-update-available" style="display:none;margin-bottom:10px"><a href="'+APP+'/download" style="display:block;padding:8px;border-radius:8px;background:#3b82f6;color:#fff;text-decoration:none;font-size:11px;text-align:center;font-weight:500">Download Update</a></div>'
            + '<div style="border-top:1px solid rgba(255,255,255,0.08);padding-top:8px"><a href="'+APP+'/download" style="color:rgba(255,255,255,0.4);font-size:10px;text-decoration:none">Downloads</a></div>';
        document.body.appendChild(infoPopup);
    }

    /**
     * Rebuild ONLY the tab buttons inside the sidebar.
     * Does NOT touch the config panel.
     */
    function renderSidebarTabs() {
        if (!tabContainer) return;
        tabContainer.innerHTML = '';

        var currentPath = window.location.href;
        var enabledTabs = getEnabledTabs();

        enabledTabs.forEach(function(tab) {
            var btn = document.createElement('a');
            var url = tab.url;
            if (url && !url.startsWith('http')) url = APP + url;
            btn.href = url;
            btn.title = tab.label;
            var isActive = currentPath.startsWith(url);
            btn.style.cssText = 'width:44px;height:44px;display:flex;flex-direction:column;align-items:center;justify-content:center;border-radius:10px;color:' + (isActive ? '#fff' : 'rgba(255,255,255,0.5)') + ';background:' + (isActive ? 'oklch(0.35 0.006 285.88)' : 'transparent') + ';text-decoration:none;font-size:9px;gap:2px;transition:all 0.15s;cursor:pointer';
            btn.innerHTML = (tab.emoji ? '<span style="font-size:18px;line-height:1">' + tab.emoji + '</span>' : makeSvg(tab.icon)) + '<span>' + tab.label + '</span>';
            btn.onmouseenter = function() { if (!isActive) { this.style.background = 'oklch(0.30 0.006 285.88)'; this.style.color = 'rgba(255,255,255,0.85)'; }};
            btn.onmouseleave = function() { if (!isActive) { this.style.background = 'transparent'; this.style.color = 'rgba(255,255,255,0.5)'; }};

            // Badge for email/chat
            if (tab.id === 'email' || tab.id === 'chat') {
                var badge = document.createElement('span');
                badge.id = 'badge-' + tab.id;
                badge.style.cssText = 'display:none;position:absolute;top:2px;right:2px;background:#ef4444;color:#fff;font-size:9px;min-width:16px;height:16px;border-radius:8px;text-align:center;line-height:16px';
                btn.style.position = 'relative';
                btn.appendChild(badge);
            }
            tabContainer.appendChild(btn);
        });
    }

    // =========================================================================
    //  CONFIG PANEL
    // =========================================================================

    /**
     * Build the config panel once and append it to the body.
     * The panel contains a scrollable list (configListEl) that gets rebuilt
     * by updateConfigList() without destroying the panel itself.
     */
    function renderConfigPanel() {
        var existing = document.getElementById('sidebar-config');
        if (existing) existing.remove();

        configPanelEl = document.createElement('div');
        configPanelEl.id = 'sidebar-config';
        configPanelEl.style.cssText = 'position:fixed;top:0;left:56px;bottom:0;width:260px;background:oklch(0.24 0.006 285.88);z-index:99997;display:flex;flex-direction:column;transform:translateX(-316px);transition:transform 0.2s ease;font-family:system-ui;color:#fff;box-shadow:4px 0 20px rgba(0,0,0,0.3)';

        // Fixed header
        var header = document.createElement('div');
        header.style.cssText = 'padding:16px 16px 12px;flex-shrink:0';
        var configTitle = document.createElement('div');
        configTitle.style.cssText = 'font-size:13px;font-weight:600;color:rgba(255,255,255,0.7);text-transform:uppercase;letter-spacing:0.5px';
        configTitle.textContent = 'Sidebar Tabs';
        header.appendChild(configTitle);
        configPanelEl.appendChild(header);

        // Scrollable body
        var scrollBody = document.createElement('div');
        scrollBody.style.cssText = 'flex:1;overflow-y:auto;padding:0 16px 16px';

        // Tab list container
        configListEl = document.createElement('div');
        configListEl.style.cssText = 'display:flex;flex-direction:column;gap:4px';
        scrollBody.appendChild(configListEl);

        // Add Custom Tab section
        var addSection = document.createElement('div');
        addSection.style.cssText = 'margin-top:16px;padding-top:16px;border-top:1px solid rgba(255,255,255,0.1)';
        var addTitle = document.createElement('div');
        addTitle.style.cssText = 'font-size:11px;font-weight:600;color:rgba(255,255,255,0.5);text-transform:uppercase;letter-spacing:0.5px;margin-bottom:8px';
        addTitle.textContent = 'Add Custom Tab';
        addSection.appendChild(addTitle);

        var iStyle = 'width:100%;padding:5px 8px;border-radius:6px;border:1px solid rgba(255,255,255,0.12);background:oklch(0.18 0.006 285.88);color:#fff;font-size:11px;font-family:system-ui;margin-bottom:5px;outline:none';
        var nameInput = document.createElement('input');
        nameInput.type = 'text'; nameInput.placeholder = 'Label'; nameInput.style.cssText = iStyle;
        var urlInput = document.createElement('input');
        urlInput.type = 'text'; urlInput.placeholder = 'URL (https://...)'; urlInput.style.cssText = iStyle;

        var iconLabel = document.createElement('div');
        iconLabel.style.cssText = 'font-size:10px;color:rgba(255,255,255,0.4);margin-bottom:4px';
        iconLabel.textContent = 'Choose an icon:';

        var selectedEmoji = '';
        var iconGrid = document.createElement('div');
        iconGrid.style.cssText = 'display:grid;grid-template-columns:repeat(8,1fr);gap:3px;margin-bottom:8px';
        EMOJI_ICONS.forEach(function(ic) {
            var btn = document.createElement('button');
            btn.style.cssText = 'width:28px;height:28px;border-radius:6px;border:none;background:transparent;font-size:14px;cursor:pointer;transition:background 0.1s';
            btn.textContent = ic;
            btn.onmouseenter = function() { if (selectedEmoji !== ic) this.style.background = 'oklch(0.30 0.006 285.88)'; };
            btn.onmouseleave = function() { if (selectedEmoji !== ic) this.style.background = 'transparent'; };
            btn.onclick = function(e) {
                e.preventDefault();
                selectedEmoji = ic;
                iconGrid.querySelectorAll('button').forEach(function(b) { b.style.background = 'transparent'; });
                this.style.background = '#3b82f6';
            };
            iconGrid.appendChild(btn);
        });

        var addBtn = document.createElement('button');
        addBtn.style.cssText = 'width:100%;padding:7px;border-radius:6px;border:none;background:#3b82f6;color:#fff;font-size:11px;cursor:pointer;font-family:system-ui';
        addBtn.textContent = '+ Add Tab';
        addBtn.onclick = function() {
            var l = nameInput.value.trim(), u = urlInput.value.trim();
            if (!l || !u) return;
            tabs.push({id:'custom_'+Date.now(), label:l, icon:'custom', url:u, enabled:true, custom:true, emoji:selectedEmoji || l.charAt(0).toUpperCase()});
            nameInput.value = ''; urlInput.value = ''; selectedEmoji = '';
            iconGrid.querySelectorAll('button').forEach(function(b) { b.style.background = 'transparent'; });
            updateConfigList();
            saveTabs();
        };

        addSection.appendChild(nameInput);
        addSection.appendChild(urlInput);
        addSection.appendChild(iconLabel);
        addSection.appendChild(iconGrid);
        addSection.appendChild(addBtn);
        scrollBody.appendChild(addSection);

        // Reset button
        var resetBtn = document.createElement('button');
        resetBtn.style.cssText = 'margin-top:12px;width:100%;padding:7px;border-radius:6px;border:1px solid rgba(255,255,255,0.12);background:none;color:rgba(255,255,255,0.4);font-size:11px;cursor:pointer;font-family:system-ui';
        resetBtn.textContent = 'Reset to Defaults';
        resetBtn.onclick = function() {
            tabs = JSON.parse(JSON.stringify(DEFAULT_TABS));
            updateConfigList();
            saveTabs();
        };
        scrollBody.appendChild(resetBtn);

        // Set as tenant default (admin only)
        var userRole = getUserRole();
        var isAdmin = onEiq && (userRole === 'admin' || userRole === 'super_admin');
        if (isAdmin) {
            var tenantBtn = document.createElement('button');
            tenantBtn.style.cssText = 'margin-top:6px;width:100%;padding:7px;border-radius:6px;border:1px solid rgba(59,130,246,0.3);background:none;color:rgba(59,130,246,0.7);font-size:11px;cursor:pointer;font-family:system-ui';
            tenantBtn.textContent = 'Set as Default for All Users';
            tenantBtn.onmouseenter = function() { this.style.color = '#3b82f6'; this.style.borderColor = '#3b82f6'; };
            tenantBtn.onmouseleave = function() { this.style.color = 'rgba(59,130,246,0.7)'; this.style.borderColor = 'rgba(59,130,246,0.3)'; };
            tenantBtn.onclick = async function() {
                tenantBtn.textContent = 'Saving...';
                tenantBtn.disabled = true;
                try {
                    var resp = await fetch(apiBase + '/api/v1/desktop/settings/tenant-defaults/sidebar_tabs', {
                        method: 'POST', headers: {'Content-Type': 'application/json'},
                        credentials: 'include',
                        body: JSON.stringify({value: tabs})
                    });
                    if (isHtmlLoginRedirect(resp)) {
                        tenantBtn.textContent = 'Not logged in';
                        tenantBtn.disabled = false;
                        return;
                    }
                    if (resp.ok) {
                        tenantBtn.textContent = 'Saved as default!';
                        setTimeout(function() { tenantBtn.textContent = 'Set as Default for All Users'; tenantBtn.disabled = false; }, 2000);
                    } else {
                        tenantBtn.textContent = 'Failed (' + resp.status + ')';
                        setTimeout(function() { tenantBtn.textContent = 'Set as Default for All Users'; tenantBtn.disabled = false; }, 3000);
                    }
                } catch(e) {
                    tenantBtn.textContent = 'Failed -- try again';
                    tenantBtn.disabled = false;
                }
            };
            scrollBody.appendChild(tenantBtn);
        }

        configPanelEl.appendChild(scrollBody);
        document.body.appendChild(configPanelEl);

        // Fill the tab list
        updateConfigList();
    }

    /**
     * Rebuild the tab rows inside configListEl without destroying the config panel.
     * This preserves the Add Custom Tab section, Reset button, and tenant default button.
     */
    function updateConfigList() {
        if (!configListEl) return;
        configListEl.innerHTML = '';

        tabs.forEach(function(tab, idx) {
            var wrapper = document.createElement('div');
            wrapper.style.cssText = 'margin-bottom:2px';

            var row = document.createElement('div');
            row.style.cssText = 'display:flex;align-items:center;gap:6px;padding:7px 8px;border-radius:8px;background:' + (tab.enabled ? 'oklch(0.30 0.006 285.88)' : 'oklch(0.22 0.006 285.88)') + ';cursor:grab';
            row.draggable = true;
            row.dataset.idx = idx;

            var grip = document.createElement('span');
            grip.style.cssText = 'color:rgba(255,255,255,0.2);font-size:10px;cursor:grab;user-select:none';
            grip.textContent = '\u2807';

            var iconSpan = document.createElement('span');
            iconSpan.style.cssText = 'width:20px;text-align:center;font-size:14px';
            iconSpan.innerHTML = tab.emoji ? tab.emoji : makeSvg(tab.icon, 14);

            var labelWrap = document.createElement('span');
            labelWrap.style.cssText = 'flex:1;overflow:hidden';
            var label = document.createElement('span');
            label.style.cssText = 'display:block;font-size:11px;color:' + (tab.enabled ? '#fff' : 'rgba(255,255,255,0.4)');
            label.textContent = tab.label;
            labelWrap.appendChild(label);
            if (tab.role && tab.role !== 'all') {
                var roleTag = document.createElement('span');
                roleTag.style.cssText = 'display:block;font-size:8px;color:rgba(255,255,255,0.25);margin-top:1px';
                roleTag.textContent = tab.role === 'super_admin' ? 'Super Admin' : tab.role === 'admin' ? 'Admin+' : 'Users';
                labelWrap.appendChild(roleTag);
            }

            var toggle = document.createElement('button');
            toggle.style.cssText = 'width:32px;height:18px;border-radius:9px;border:none;cursor:pointer;position:relative;transition:background 0.2s;background:' + (tab.enabled ? '#3b82f6' : 'rgba(255,255,255,0.15)') + ';flex-shrink:0';
            var knob = document.createElement('span');
            knob.style.cssText = 'position:absolute;top:2px;width:14px;height:14px;border-radius:50%;background:#fff;transition:left 0.2s;left:' + (tab.enabled ? '16px' : '2px');
            toggle.appendChild(knob);
            toggle.onclick = function(e) {
                e.stopPropagation();
                tab.enabled = !tab.enabled;
                updateConfigList();
                saveTabs();
            };

            // Edit button
            var editBtn = document.createElement('button');
            editBtn.style.cssText = 'width:18px;height:18px;border-radius:4px;border:none;background:none;color:rgba(255,255,255,0.3);font-size:10px;cursor:pointer;flex-shrink:0';
            editBtn.textContent = '\u270F';
            editBtn.title = 'Edit';

            row.appendChild(grip);
            row.appendChild(iconSpan);
            row.appendChild(labelWrap);
            row.appendChild(editBtn);
            row.appendChild(toggle);

            if (tab.custom) {
                var delBtn = document.createElement('button');
                delBtn.style.cssText = 'width:18px;height:18px;border-radius:4px;border:none;background:none;color:rgba(239,68,68,0.6);font-size:11px;cursor:pointer;flex-shrink:0';
                delBtn.innerHTML = '&times;';
                delBtn.onclick = function(e) {
                    e.stopPropagation();
                    tabs.splice(idx, 1);
                    updateConfigList();
                    saveTabs();
                };
                row.appendChild(delBtn);
            }

            wrapper.appendChild(row);

            // Edit panel (separate element below the row, not inside it)
            var editPanel = buildEditPanel(tab, idx);
            wrapper.appendChild(editPanel);

            editBtn.onclick = function(e) {
                e.stopPropagation();
                var isVisible = editPanel.style.display !== 'none';
                editPanel.style.display = isVisible ? 'none' : 'block';
                if (!isVisible) {
                    // Scroll the edit panel into view within the config panel's scrollable area
                    setTimeout(function() {
                        editPanel.scrollIntoView({behavior:'smooth', block:'nearest'});
                    }, 50);
                }
            };

            // Drag & drop
            row.ondragstart = function(e) { e.dataTransfer.setData('text/plain', idx); this.style.opacity = '0.5'; };
            row.ondragend = function() { this.style.opacity = '1'; };
            row.ondragover = function(e) { e.preventDefault(); };
            row.ondrop = function(e) {
                e.preventDefault();
                var fromIdx = parseInt(e.dataTransfer.getData('text/plain'));
                if (fromIdx !== idx) {
                    var item = tabs.splice(fromIdx, 1)[0];
                    tabs.splice(idx, 0, item);
                    updateConfigList();
                    saveTabs();
                }
            };

            configListEl.appendChild(wrapper);
        });
    }

    /**
     * Build the edit sub-panel for a single tab.
     * Returns a detached DOM element (hidden by default).
     */
    function buildEditPanel(tab, idx) {
        var editPanel = document.createElement('div');
        editPanel.style.cssText = 'display:none;padding:8px;background:oklch(0.20 0.006 285.88);border-radius:0 0 8px 8px;margin-top:1px';
        var eStyle = 'width:100%;padding:4px 6px;border-radius:4px;border:1px solid rgba(255,255,255,0.1);background:oklch(0.18 0.006 285.88);color:#fff;font-size:10px;font-family:system-ui;margin-bottom:4px;outline:none;box-sizing:border-box';

        var eLabelInput = document.createElement('input');
        eLabelInput.type = 'text'; eLabelInput.value = tab.label; eLabelInput.placeholder = 'Label'; eLabelInput.style.cssText = eStyle;
        var eUrlInput = document.createElement('input');
        eUrlInput.type = 'text'; eUrlInput.value = tab.url; eUrlInput.placeholder = 'URL'; eUrlInput.style.cssText = eStyle;

        // Role selector
        var eRoleLabel = document.createElement('div');
        eRoleLabel.style.cssText = 'font-size:9px;color:rgba(255,255,255,0.4);margin-bottom:3px';
        eRoleLabel.textContent = 'Visible to:';
        var eRoleSelect = document.createElement('select');
        eRoleSelect.style.cssText = eStyle + ';margin-bottom:6px';
        [{v:'all',l:'Everyone'},{v:'user',l:'All logged-in users'},{v:'admin',l:'Admin & Super Admin'},{v:'super_admin',l:'Super Admin only'}].forEach(function(opt) {
            var o = document.createElement('option');
            o.value = opt.v; o.textContent = opt.l;
            if ((tab.role || 'all') === opt.v) o.selected = true;
            eRoleSelect.appendChild(o);
        });

        // Mini icon picker
        var eIconGrid = document.createElement('div');
        eIconGrid.style.cssText = 'display:grid;grid-template-columns:repeat(8,1fr);gap:2px;margin-bottom:6px';
        var eSelectedEmoji = tab.emoji || '';
        EMOJI_ICONS.forEach(function(ic) {
            var ib = document.createElement('button');
            ib.style.cssText = 'width:24px;height:24px;border-radius:4px;border:none;background:' + (eSelectedEmoji === ic ? '#3b82f6' : 'transparent') + ';font-size:12px;cursor:pointer';
            ib.textContent = ic;
            ib.onclick = function(e) {
                e.preventDefault(); e.stopPropagation();
                eSelectedEmoji = ic;
                eIconGrid.querySelectorAll('button').forEach(function(b) { b.style.background = 'transparent'; });
                this.style.background = '#3b82f6';
            };
            eIconGrid.appendChild(ib);
        });

        // Auto-save on field changes
        function autoSaveTab() {
            tab.label = eLabelInput.value.trim() || tab.label;
            tab.url = eUrlInput.value.trim() || tab.url;
            tab.role = eRoleSelect.value;
            if (eSelectedEmoji) tab.emoji = eSelectedEmoji;
            saveTabs();
        }
        eLabelInput.onblur = autoSaveTab;
        eUrlInput.onblur = autoSaveTab;
        eRoleSelect.onchange = autoSaveTab;

        var eDoneBtn = document.createElement('button');
        eDoneBtn.style.cssText = 'width:100%;padding:5px;border-radius:4px;border:none;background:oklch(0.30 0.006 285.88);color:rgba(255,255,255,0.6);font-size:10px;cursor:pointer;font-family:system-ui';
        eDoneBtn.textContent = 'Close';
        eDoneBtn.onclick = function(e) {
            e.stopPropagation();
            autoSaveTab();
            editPanel.style.display = 'none';
        };

        editPanel.appendChild(eLabelInput);
        editPanel.appendChild(eUrlInput);
        editPanel.appendChild(eRoleLabel);
        editPanel.appendChild(eRoleSelect);
        editPanel.appendChild(eIconGrid);
        editPanel.appendChild(eDoneBtn);

        return editPanel;
    }

    // =========================================================================
    //  CONFIG PANEL TOGGLE
    // =========================================================================

    function toggleConfigPanel() {
        configOpen = !configOpen;
        if (configPanelEl) {
            configPanelEl.style.transform = configOpen ? 'translateX(0)' : 'translateX(-316px)';
        }
        if (settingsBtnEl) {
            settingsBtnEl.style.background = configOpen ? 'oklch(0.35 0.006 285.88)' : 'none';
            settingsBtnEl.style.color = configOpen ? '#fff' : 'rgba(255,255,255,0.3)';
        }
    }

    function closeConfigPanel() {
        if (!configOpen) return;
        configOpen = false;
        if (configPanelEl) configPanelEl.style.transform = 'translateX(-316px)';
        if (settingsBtnEl) {
            settingsBtnEl.style.background = 'none';
            settingsBtnEl.style.color = 'rgba(255,255,255,0.3)';
        }
    }

    // Close config panel when clicking outside
    document.addEventListener('click', function(e) {
        if (configOpen && configPanelEl && settingsBtnEl
            && !configPanelEl.contains(e.target) && !settingsBtnEl.contains(e.target)) {
            closeConfigPanel();
        }
    });

    // =========================================================================
    //  SAVE
    // =========================================================================

    /**
     * Save tabs to server, then update ONLY the sidebar tab buttons.
     * Does NOT rebuild the config panel.
     */
    async function saveTabs() {
        // Always update sidebar tabs immediately for responsiveness
        renderSidebarTabs();

        // Don't save if we never loaded from server — prevents overwriting real data with defaults
        if (!tabsLoadedFromServer) {
            showSaveIndicator('Cannot save — tabs not loaded from server', '#ef4444');
            return;
        }

        // Show save indicator
        var indicator = document.getElementById('sidebar-save-indicator');
        if (!indicator) {
            indicator = document.createElement('div');
            indicator.id = 'sidebar-save-indicator';
            indicator.style.cssText = 'position:fixed;top:8px;left:64px;z-index:99999;padding:4px 12px;border-radius:6px;font-size:11px;font-family:system-ui;transition:opacity 0.3s';
            document.body.appendChild(indicator);
        }
        indicator.style.opacity = '1';
        indicator.style.background = '#3b82f6';
        indicator.style.color = '#fff';
        indicator.textContent = 'Saving...';

        try {
            var resp = await fetch(apiBase + '/api/v1/desktop/settings/sidebar_tabs', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                credentials: 'include',
                body: JSON.stringify({value: tabs})
            });

            // Detect HTML login redirect (Symfony redirects to login form instead of returning JSON 401)
            if (isHtmlLoginRedirect(resp)) {
                indicator.style.background = '#ef4444';
                indicator.textContent = 'Not logged in';
                setTimeout(function() { indicator.style.opacity = '0'; }, 3000);
                return;
            }

            if (resp.ok) {
                indicator.style.background = '#22c55e';
                indicator.textContent = 'Saved';
            } else {
                var errText = '';
                try { errText = (await resp.json()).error || ''; } catch(x) {}
                indicator.style.background = '#ef4444';
                indicator.textContent = 'Save failed: ' + resp.status + (errText ? ' ' + errText : '');
            }
        } catch(e) {
            indicator.style.background = '#ef4444';
            indicator.textContent = 'Save error';
        }
        setTimeout(function() { indicator.style.opacity = '0'; }, 2000);
    }

    // =========================================================================
    //  TAB LOADING
    // =========================================================================

    async function loadAndRender() {
        // On external domains, API calls will fail (no cookies for evolvepreneuriq.app).
        // If we have cached tabs from Tauri, trust them and skip the API call.
        var hasCachedTabs = window.__EVOLVEAPP_CACHED_TABS__ && Array.isArray(window.__EVOLVEAPP_CACHED_TABS__);

        if (onEiq) {
            // On EIQ domain: load merged tabs from new unified endpoint
            try {
                var resp = await fetch(apiBase + '/api/v1/desktop/sidebar/tabs', {credentials:'include'});
                if (resp.ok && !isHtmlLoginRedirect(resp)) {
                    var data = await resp.json();
                    if (data.tabs && Array.isArray(data.tabs)) { tabs = data.tabs; tabsLoadedFromServer = true; }
                } else if (resp.status === 404) {
                    // Fallback to old endpoints for servers that haven't upgraded
                    var tr = await fetch(apiBase + '/api/v1/desktop/settings/tenant-defaults/sidebar_tabs', {credentials:'include'});
                    if (tr.ok && !isHtmlLoginRedirect(tr)) {
                        var td = await tr.json();
                        if (td.value && Array.isArray(td.value)) { tabs = td.value; tabsLoadedFromServer = true; }
                    }
                    var ur = await fetch(apiBase + '/api/v1/desktop/settings/sidebar_tabs', {credentials:'include'});
                    if (ur.ok && !isHtmlLoginRedirect(ur)) {
                        var ud = await ur.json();
                        if (ud.value && Array.isArray(ud.value)) { tabs = ud.value; tabsLoadedFromServer = true; }
                    }
                }
            } catch(e) {}

            // Cache tabs locally via Tauri so external sites have the right tabs
            if (tabsLoadedFromServer && window.__TAURI_INTERNALS__) {
                try { window.__TAURI_INTERNALS__.invoke('save_cached_tabs', {tabsJson: JSON.stringify(tabs)}); } catch(e) {}
            }
        } else if (hasCachedTabs) {
            // On external domain: use cached tabs (already loaded at line 50), mark as loaded
            tabsLoadedFromServer = true;
        }

        // Build sidebar shell + tab buttons
        buildSidebar();

        // Build config panel (once)
        renderConfigPanel();

        // Start background tasks
        pollBadges();
        checkVersion();
    }

    // =========================================================================
    //  NOTIFICATION BADGES
    // =========================================================================

    async function pollBadges() {
        if (!onEiq) return;
        try {
            var r = await fetch(apiBase + '/api/v1/desktop/check-notifications', {credentials:'include'});
            if (!r.ok || isHtmlLoginRedirect(r)) return;
            var d = await r.json();
            var emailBadge = document.getElementById('badge-email');
            if (emailBadge) {
                if (d.unread_emails > 0) {
                    emailBadge.textContent = d.unread_emails > 99 ? '99+' : d.unread_emails;
                    emailBadge.style.display = 'block';
                } else {
                    emailBadge.style.display = 'none';
                }
            }
            var chatBadge = document.getElementById('badge-chat');
            if (chatBadge) {
                if (d.unread_chats > 0) {
                    chatBadge.textContent = d.unread_chats > 99 ? '99+' : d.unread_chats;
                    chatBadge.style.display = 'block';
                } else {
                    chatBadge.style.display = 'none';
                }
            }
        } catch(e) {}
    }
    setInterval(pollBadges, 60000);

    // =========================================================================
    //  VERSION CHECK
    // =========================================================================

    async function checkVersion() {
        var currentVersion = null;
        var uaMatch = navigator.userAgent.match(/EvolveApp\/([\d.]+)/);
        if (uaMatch) currentVersion = uaMatch[1];
        if (!currentVersion && window.__TAURI_INTERNALS__) {
            try { currentVersion = await window.__TAURI_INTERNALS__.invoke('plugin:app|version'); } catch(e) {}
        }

        var latest = null;
        var vData = null;
        try {
            var resp = await fetch((onEiq ? '' : APP) + '/api/v1/desktop/version');
            if (resp.ok) { vData = await resp.json(); latest = vData.version || null; }
        } catch(e) {}

        var curEl = document.getElementById('info-current-ver');
        var latEl = document.getElementById('info-latest-ver');
        var verLabel = document.getElementById('sidebar-version-label');
        if (curEl) curEl.textContent = currentVersion || 'unknown';
        if (latEl) latEl.textContent = latest || 'unknown';
        if (verLabel) verLabel.textContent = currentVersion ? 'v' + currentVersion : (latest ? 'v' + latest : '...');

        if (latest && currentVersion && latest !== currentVersion) {
            var infoB = document.getElementById('sidebar-info-btn');
            if (infoB) {
                infoB.style.position = 'relative';
                var dot = document.createElement('span');
                dot.style.cssText = 'position:absolute;top:4px;right:4px;width:10px;height:10px;border-radius:50%;background:#ef4444;border:2px solid oklch(0.21 0.006 285.88)';
                infoB.appendChild(dot);
                infoB.style.color = '#f59e0b';
                infoB.style.background = 'rgba(245,158,11,0.15)';
                if (verLabel) { verLabel.textContent = 'Update!'; verLabel.style.color = '#ef4444'; verLabel.style.fontWeight = '600'; }
            }
            var updateDiv = document.getElementById('info-update-available');
            if (updateDiv) updateDiv.style.display = 'block';

            // Auto-detect platform for download URL
            if (vData && vData.downloadUrls) {
                var plat = navigator.platform || '';
                var ua2 = navigator.userAgent || '';
                var platform = (plat.includes('Win') || ua2.includes('Windows')) ? 'windows'
                    : (plat.includes('Mac') || ua2.includes('Macintosh')) ? 'macos' : 'linux';
                var downloadUrl = null;
                if (platform === 'windows' && vData.downloadUrls.windows) {
                    downloadUrl = vData.downloadUrls.windows;
                } else if (platform === 'macos' && vData.downloadUrls.macos) {
                    downloadUrl = vData.downloadUrls.macos;
                } else if (platform === 'linux' && vData.downloadUrls.linux) {
                    var l = vData.downloadUrls.linux;
                    downloadUrl = l.flatpak || l.appimage || l.deb;
                }
                if (downloadUrl && updateDiv) {
                    updateDiv.innerHTML = '<a href="' + downloadUrl + '" style="display:block;padding:8px;border-radius:8px;background:#3b82f6;color:#fff;text-decoration:none;font-size:11px;text-align:center;font-weight:500">Download v' + latest + ' & Install</a>';
                }
            }
        }
    }

    // =========================================================================
    //  OFFLINE INDICATOR
    // =========================================================================

    function showOfflineBar(show) {
        var bar = document.getElementById('desktop-offline-bar');
        if (show && !bar) {
            bar = document.createElement('div');
            bar.id = 'desktop-offline-bar';
            bar.style.cssText = 'position:fixed;top:0;left:56px;right:0;z-index:99999;background:#ef4444;color:#fff;padding:6px 16px;font-size:13px;text-align:center;font-family:system-ui';
            bar.textContent = 'You are offline \u2014 check your internet connection';
            document.body.insertBefore(bar, document.body.firstChild);
        } else if (!show && bar) {
            bar.remove();
        }
    }
    window.addEventListener('offline', function() { showOfflineBar(true); });
    window.addEventListener('online', function() { showOfflineBar(false); });
    if (!navigator.onLine) showOfflineBar(true);

    // =========================================================================
    //  LINK INTERCEPTION
    // =========================================================================

    function isDownloadLink(href) {
        if (!href) return false;
        if (/\.(exe|msi|dmg|deb|rpm|appimage|flatpak|zip|tar\.gz|pdf|csv|xlsx|docx|pptx)(\?|$)/i.test(href)) return true;
        if (/github\.com\/.*\/releases\/download\//i.test(href)) return true;
        return false;
    }

    function openInBrowser(url) {
        if (window.__TAURI_INTERNALS__) {
            window.__TAURI_INTERNALS__.invoke('plugin:shell|open', { path: url })
                .catch(function() { window.open(url, '_blank'); });
        } else {
            window.open(url, '_blank');
        }
    }

    document.addEventListener('click', function(e) {
        var link = e.target.closest('a');
        if (!link || !link.href) return;

        // File downloads -> open in system browser
        if (isDownloadLink(link.href) || link.hasAttribute('download')) {
            e.preventDefault();
            openInBrowser(link.href);
            return;
        }

        // target="_blank" links -> navigate in same window
        if (link.target === '_blank') {
            e.preventDefault();
            window.location.href = link.href;
        }
    }, true);

    // Override window.open
    var _origOpen = window.open;
    window.open = function(url) {
        if (url && typeof url === 'string' && url.startsWith('http')) { window.location.href = url; return window; }
        return _origOpen.apply(this, arguments);
    };

    // =========================================================================
    //  START
    // =========================================================================

    loadAndRender();
})();
