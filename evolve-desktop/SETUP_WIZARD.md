# Setup Wizard Flow

## When the Wizard Appears

The setup wizard (`/welcome`) appears:

### ✅ First Launch
- App checks `localStorage.getItem('api_configured')`
- If **NOT SET** → Show wizard
- User chooses their setup option
- After completion → `api_configured` flag is saved

### ✅ Manual Reset
To show wizard again (for testing or changing server):
```javascript
// In browser console or settings page:
localStorage.removeItem('api_configured')
localStorage.removeItem('api_url')
// Reload app
```

## User Flow

```
┌─────────────────────────────────────────────────┐
│                                                 │
│  1. FIRST LAUNCH                                │
│     └─> Check: api_configured?                  │
│         ├─ NO  → Welcome Wizard                 │
│         └─ YES → Login Page                     │
│                                                 │
├─────────────────────────────────────────────────┤
│                                                 │
│  2. WELCOME WIZARD (/welcome)                   │
│     Choose setup:                               │
│     ○ EvolveApp Cloud (api.evolveapp.com)      │
│     ○ Self-Hosted Server (custom URL)          │
│     ○ Local Development (localhost:8547)       │
│                                                 │
│     [Test Connection] [Continue]                │
│                                                 │
├─────────────────────────────────────────────────┤
│                                                 │
│  3. AFTER WIZARD                                │
│     └─> Save to localStorage:                   │
│         - api_url: "http://localhost:8547"      │
│         - api_configured: "true"                │
│     └─> Redirect to: /login                     │
│                                                 │
├─────────────────────────────────────────────────┤
│                                                 │
│  4. LOGIN (/login)                              │
│     Email: demo@evolveapp.com                   │
│     Password: demo123                           │
│     └─> Redirect to: /dashboard                 │
│                                                 │
├─────────────────────────────────────────────────┤
│                                                 │
│  5. SUBSEQUENT LAUNCHES                         │
│     Check: api_configured = "true"              │
│     └─> Skip wizard → Go to /login              │
│                                                 │
└─────────────────────────────────────────────────┘
```

## Setup Options Explained

### 🌐 EvolveApp Cloud (SaaS)
- **URL**: `https://api.evolveapp.com` (or your hosted URL)
- **Best for**: End users, no technical setup
- **Pros**: Always updated, reliable, backed up
- **Cons**: Requires internet, hosted by you

### 🏢 Self-Hosted Server
- **URL**: Custom (e.g., `https://company.internal/api`)
- **Best for**: Enterprise customers, data sovereignty
- **Pros**: Full control, on-premise, compliant
- **Cons**: Requires IT setup, maintenance

### 💻 Local Development
- **URL**: `http://localhost:8547`
- **Best for**: Developers, testing
- **Pros**: Fast, offline, debugging
- **Cons**: Not for production use

## Files Modified

1. **src/views/Welcome.vue** - Setup wizard UI
2. **src/router/index.ts** - Routing logic with wizard check
3. **src/main.ts** - Reads saved API URL from localStorage

## Testing the Wizard

### To See Wizard Again:
```bash
# Open browser console (F12) and run:
localStorage.clear()
# Reload page
```

### To Skip Wizard (Direct to Login):
```bash
# Set these in browser console:
localStorage.setItem('api_configured', 'true')
localStorage.setItem('api_url', 'http://localhost:8547')
# Reload page
```

## Production Deployment

### SaaS Model (No Wizard Needed)
```env
# .env.production
VITE_API_URL=https://api.evolveapp.com
```

Then in `main.ts`, force API URL:
```typescript
// Skip wizard for SaaS deployments
localStorage.setItem('api_configured', 'true')
localStorage.setItem('api_url', 'https://api.evolveapp.com')
```

### Hybrid Model (Show Wizard)
Keep wizard enabled - users choose their setup on first launch.

## Current Demo Flow

1. Visit: http://localhost:5173/
2. See: **Welcome Wizard** (first time)
3. Choose: **Local Development** (localhost:8547)
4. Click: **Continue**
5. See: **Login Page**
6. Enter: demo@evolveapp.com / demo123
7. Access: **Dashboard & Email Module**

## Reset Instructions

**For Users (via Settings Page - TODO):**
```vue
<button @click="resetServer">
  Change Server Settings
</button>

<script>
function resetServer() {
  localStorage.removeItem('api_configured')
  localStorage.removeItem('api_url')
  router.push('/welcome')
}
</script>
```

**For Developers (Console):**
```javascript
localStorage.clear()
location.reload()
```
