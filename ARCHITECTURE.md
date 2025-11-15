# EvolveApp Architecture - Remote Database

## 🏗️ Architecture Overview

EvolveApp Desktop uses a **remote-only database architecture** - all data is stored in the eiq-manager PostgreSQL database. The desktop app is a native wrapper around the Vue frontend that communicates with the backend API.

```
┌─────────────────────────────────────┐
│   EvolveApp Desktop (Tauri)         │
│                                     │
│  ┌──────────────────────────────┐  │
│  │  Vue 3 Frontend              │  │
│  │  - UI Components             │  │
│  │  - Pinia State               │  │
│  │  - Axios HTTP Client         │  │
│  └──────────┬───────────────────┘  │
│             │                       │
│  ┌──────────▼───────────────────┐  │
│  │  Tauri Backend (Rust)        │  │
│  │  - Native Notifications      │  │
│  │  - System Tray               │  │
│  │  - File Dialogs              │  │
│  │  - Window Management         │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
             │ HTTP/REST
             │ (axios)
             ▼
┌─────────────────────────────────────┐
│   eiq-manager API (Symfony)         │
│   http://localhost:8547             │
│                                     │
│  ┌──────────────────────────────┐  │
│  │  Symfony 7.3 Framework       │  │
│  │  - REST API Controllers      │  │
│  │  - JWT Authentication        │  │
│  │  - Business Logic            │  │
│  └──────────┬───────────────────┘  │
└─────────────┼───────────────────────┘
              │ SQL
              ▼
┌─────────────────────────────────────┐
│   PostgreSQL 17 Database            │
│   (DigitalOcean)                    │
│                                     │
│  - Email Manager Tables             │
│  - CRM Marketing Tables             │
│  - Calendar Tables                  │
│  - User Management                  │
│  - All Module Data                  │
└─────────────────────────────────────┘
```

## 💾 Database Strategy

### ❌ No Local Database
- **No SQLite** - Removed from architecture
- **No offline storage** - All data is server-side
- **No sync engine** - Direct API calls

### ✅ Remote Database Only
- **PostgreSQL 17** on DigitalOcean
- **Real-time data** - Always fresh from server
- **Centralized storage** - One source of truth
- **Multi-device ready** - Same data everywhere

## 🔄 Data Flow

### Read Operations
```
1. User clicks "View Emails"
2. Vue component calls Pinia store
3. Store calls axios.get('/api/v1/emails')
4. eiq-manager API queries PostgreSQL
5. Returns JSON response
6. Vue component renders data
```

### Write Operations
```
1. User creates new contact
2. Form data collected in Vue
3. axios.post('/api/v1/crm/contacts', data)
4. eiq-manager validates and saves to PostgreSQL
5. Returns created entity
6. Vue updates UI
```

## 🎨 Frontend (Vue 3)

### Key Technologies
- **Vue 3** - Composition API
- **TypeScript** - Type safety
- **Pinia** - State management
- **Axios** - HTTP client
- **Vue Router** - Navigation
- **Tailwind CSS + DaisyUI** - Styling

### API Integration

All API calls go through axios configured in `src/main.ts`:

```typescript
// Axios configuration
axios.defaults.baseURL = 'http://localhost:8547'
axios.defaults.headers.common['Authorization'] = `Bearer ${token}`

// Example API call
const response = await axios.get('/api/v1/emails')
const emails = response.data
```

### Store Pattern

```typescript
// stores/email.ts
export const useEmailStore = defineStore('email', () => {
  const emails = ref([])

  async function fetchEmails() {
    const response = await axios.get('/api/v1/emails')
    emails.value = response.data
  }

  return { emails, fetchEmails }
})
```

## 🦀 Backend (Rust/Tauri)

### Purpose
Tauri provides **native desktop features only**:
- ✅ System tray integration
- ✅ Native notifications
- ✅ File save/open dialogs
- ✅ Window management
- ✅ Auto-updates (future)
- ❌ **NOT for database operations**
- ❌ **NOT for data storage**

### Tauri Commands

```rust
// src-tauri/src/main.rs

#[tauri::command]
async fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
async fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
```

### Frontend Usage

```typescript
import { greet, showNotification } from '@/services/tauri'

// Call Rust command
const message = await greet('John')

// Show native notification
await showNotification('Email Received', 'New message from Alice')
```

## 🔐 Authentication

### JWT Flow

```
1. User enters email/password
2. POST /api/v1/auth/login
3. Server returns: { access_token, refresh_token, expires_in }
4. Frontend stores tokens in localStorage
5. Axios adds: Authorization: Bearer {token}
6. On 401 error, automatically refresh token
7. On refresh fail, redirect to login
```

### Implementation

```typescript
// stores/auth.ts
async function login(email: string, password: string) {
  const response = await axios.post('/api/v1/auth/login', {
    email,
    password
  })

  tokens.value = {
    accessToken: response.data.access_token,
    refreshToken: response.data.refresh_token,
    expiresAt: Date.now() + response.data.expires_in * 1000
  }

  axios.defaults.headers.common['Authorization'] =
    `Bearer ${tokens.value.accessToken}`
}
```

## 📡 API Endpoints

### Base URL
```
Development: http://localhost:8547
Production: https://api.evolveapp.com
```

### Available Endpoints

**Authentication:**
- `POST /api/v1/auth/login` - Login with credentials
- `POST /api/v1/auth/refresh` - Refresh access token
- `POST /api/v1/auth/logout` - Logout
- `GET /api/v1/auth/profile` - Get user profile

**Email:**
- `GET /api/v1/emails` - List emails
- `GET /api/v1/emails/{id}` - Get email
- `POST /api/v1/emails` - Send email
- `PATCH /api/v1/emails/{id}` - Update flags
- `DELETE /api/v1/emails/{id}` - Delete email

**Calendar:**
- `GET /api/v1/calendar/events` - List events
- `POST /api/v1/calendar/events` - Create event
- `PATCH /api/v1/calendar/events/{id}` - Update event
- `DELETE /api/v1/calendar/events/{id}` - Delete event

**CRM:**
- `GET /api/v1/crm/contacts` - List contacts
- `POST /api/v1/crm/contacts` - Create contact
- `GET /api/v1/crm/deals` - List deals
- `POST /api/v1/crm/deals` - Create deal

## 🚀 Development Workflow

### Start Backend API
```bash
cd /home/john/sources/eiq-manager
docker compose up
# API available at http://localhost:8547
```

### Start Desktop App (Browser Mode)
```bash
cd /home/john/sources/evolveapp/evolve-desktop
npm run dev
# Opens at http://localhost:5173
# Full access to Vue DevTools
# Uses axios for API calls
```

### Start Desktop App (Native Mode)
```bash
npm run tauri:dev
# Opens native window
# System tray integration
# Native notifications
```

## 📦 Build & Distribution

### Development Build
```bash
npm run tauri:dev
```

### Production Build
```bash
npm run tauri:build
```

**Output:**
- `src-tauri/target/release/evolve-desktop` - Binary
- `src-tauri/target/release/bundle/deb/` - Debian package
- `src-tauri/target/release/bundle/appimage/` - AppImage

## 🔧 Configuration

### Desktop App (.env)
```env
VITE_API_URL=http://localhost:8547
VITE_APP_NAME=EvolveApp
VITE_ENABLE_OFFLINE=false
```

### Backend API (.env.local)
```env
DATABASE_URL=postgresql://user:pass@host:5432/database
APP_ENV=dev
CORS_ALLOW_ORIGIN='^https?://(localhost|127\.0\.0\.1)(:[0-9]+)?$'
```

## 🎯 Module System

Modules communicate **only through the API**:

```typescript
// Email module
import { moduleLoader } from '@/core/plugin-system'

const emailModule = {
  metadata: { id: 'email', name: 'Email' },
  async install() {
    // No database setup needed
    console.log('Email module installed')
  },
  getMainView: () => import('./views/EmailView.vue'),
  getRoutes: () => [
    { path: '/email', component: EmailView }
  ],
  async search(query) {
    // Search via API
    const response = await axios.get('/api/v1/emails/search', {
      params: { q: query }
    })
    return response.data
  }
}

await moduleLoader.registerModule(emailModule)
```

## 📊 Performance

### Advantages of Remote-Only
- ✅ **No sync complexity** - No conflict resolution
- ✅ **Always up-to-date** - Real-time data
- ✅ **Lower disk usage** - No local cache
- ✅ **Easier to maintain** - Single database schema
- ✅ **Multi-device** - Same data everywhere

### Considerations
- ⚠️ **Requires internet** - No offline mode
- ⚠️ **Network latency** - API call overhead
- ⚠️ **Server dependency** - Backend must be running

### Optimizations
- Cache frequently accessed data in Pinia stores
- Debounce search queries
- Lazy load large lists
- Use pagination for email/contacts
- Implement optimistic UI updates

## 🔒 Security

**Frontend:**
- XSS protection via Vue template escaping
- JWT tokens in localStorage (HttpOnly not available)
- CORS configured on backend
- No sensitive data in frontend code

**Backend:**
- JWT with expiration
- Password hashing (bcrypt)
- SQL injection protection (Doctrine ORM)
- CSRF tokens for form submissions
- Rate limiting on API endpoints

## 📝 Summary

**Architecture Type:** Client-Server with Native Wrapper

**Data Storage:** Remote PostgreSQL only

**Communication:** HTTP/REST via Axios

**Desktop Features:** Tauri for native OS integration

**Database:** DigitalOcean PostgreSQL 17

**API:** Symfony 7.3 on Docker (port 8547)

**Frontend:** Vue 3 + TypeScript + Tailwind

**State:** Pinia stores

**Auth:** JWT tokens
