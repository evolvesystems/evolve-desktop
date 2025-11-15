# 🚀 EvolveApp Desktop - Quick Start Guide

Complete setup guide for running the EvolveApp desktop application.

## 📋 Prerequisites

### Already Installed ✅
- Node.js v20.19.5
- npm 10.8.2
- Rust 1.91.1 / Cargo 1.91.1
- Vue 3 + TypeScript project set up
- Tauri project structure created

### Database ✅
- PostgreSQL 17 on DigitalOcean
- Already configured in eiq-manager
- Connection string in `/home/john/sources/eiq-manager/.env.local`

### Backend API ✅
- eiq-manager (Symfony 7.3) available
- Runs on Docker Compose
- Accessible at `http://localhost:8547`

## 🎯 Architecture Summary

```
Desktop App (Tauri + Vue)
    │
    │ HTTP/REST (axios)
    ▼
eiq-manager API (Symfony)
    │
    │ SQL
    ▼
PostgreSQL Database (DigitalOcean)
```

**No local database!** All data is remote via API.

## ⚡ Quick Start (3 Steps)

### Step 1: Start Backend API

```bash
cd /home/john/sources/eiq-manager

# Start Docker containers
docker compose up -d

# Verify API is running
curl http://localhost:8547
```

API should be accessible at: `http://localhost:8547`

### Step 2: Start Desktop App (Browser Mode)

```bash
cd /home/john/sources/evolveapp/evolve-desktop

# Start development server
npm run dev
```

Opens at: `http://localhost:5173`

**Features available:**
- ✅ Full Vue frontend
- ✅ API calls to eiq-manager
- ✅ Authentication
- ✅ All modules
- ✅ Vue DevTools
- ❌ Native desktop features (tray, notifications)

### Step 3 (Optional): Run as Native Desktop

**Requires system dependencies:**

```bash
# Install dependencies
sudo bash ./install-tauri-deps.sh

# Run desktop app
npm run tauri:dev
```

**Additional features:**
- ✅ Native window
- ✅ System tray
- ✅ Native notifications
- ✅ File dialogs
- ✅ No browser chrome

## 🔑 Authentication

### Default Credentials

Check your eiq-manager for user accounts. If you need to create one:

```bash
cd /home/john/sources/eiq-manager

# Create user via Symfony console
docker compose exec php bin/console app:create-user \
  --email=admin@evolveapp.com \
  --password=password123 \
  --role=ROLE_ADMIN
```

### Login Flow

1. Open desktop app
2. Navigate to `/login`
3. Enter credentials
4. JWT token stored in localStorage
5. All API requests include `Authorization: Bearer {token}`

## 📦 Project Structure

```
/home/john/sources/
├── eiq-manager/              # Backend API (Symfony)
│   ├── src/
│   │   ├── Controller/       # API endpoints
│   │   └── Entity/           # Database models
│   ├── migrations/           # Database migrations
│   ├── .env.local            # Database credentials
│   └── compose.yml           # Docker config
│
└── evolveapp/
    └── evolve-desktop/       # Desktop App (Tauri + Vue)
        ├── src/              # Vue frontend
        │   ├── views/        # Pages
        │   ├── components/   # UI components
        │   ├── stores/       # Pinia state
        │   └── services/     # API & Tauri services
        ├── src-tauri/        # Rust backend
        │   └── src/          # Tauri commands
        ├── .env              # API URL config
        └── package.json
```

## 🔧 Configuration

### Desktop App (.env)

```env
VITE_API_URL=http://localhost:8547
VITE_APP_NAME=EvolveApp
VITE_ENABLE_OFFLINE=false
```

### Backend API (.env.local)

```env
DATABASE_URL="postgresql://doadmin:***@...ondigitalocean.com:25061/eiq-manager-pool?sslmode=require"
APP_ENV=dev
CORS_ALLOW_ORIGIN='^https?://(localhost|127\.0\.0\.1)(:[0-9]+)?$'
```

## 🌐 API Endpoints

Base URL: `http://localhost:8547`

**Authentication:**
- `POST /api/v1/auth/login` - Login
- `POST /api/v1/auth/refresh` - Refresh token
- `GET /api/v1/auth/profile` - User profile

**Email:**
- `GET /api/v1/emails` - List emails
- `POST /api/v1/emails` - Send email
- `GET /api/v1/emails/{id}` - Get email
- `PATCH /api/v1/emails/{id}` - Update flags

**Calendar:**
- `GET /api/v1/calendar/events` - List events
- `POST /api/v1/calendar/events` - Create event

**CRM:**
- `GET /api/v1/crm/contacts` - List contacts
- `POST /api/v1/crm/contacts` - Create contact

## 🛠️ Development Workflow

### Hot Reload Development

**Terminal 1 - Backend:**
```bash
cd /home/john/sources/eiq-manager
docker compose up
```

**Terminal 2 - Frontend:**
```bash
cd /home/john/sources/evolveapp/evolve-desktop
npm run dev
```

Now you have:
- ✅ Backend API at http://localhost:8547
- ✅ Frontend at http://localhost:5173
- ✅ Hot reload on code changes
- ✅ Vue DevTools

### Making API Calls

```typescript
// In Vue component or store
import axios from 'axios'

// List emails
const response = await axios.get('/api/v1/emails')
const emails = response.data

// Create contact
const contact = await axios.post('/api/v1/crm/contacts', {
  first_name: 'John',
  last_name: 'Doe',
  email: 'john@example.com'
})
```

### Using Tauri Features

```typescript
import { showNotification, openFileDialog } from '@/services/tauri'

// Show notification
await showNotification('Email Received', 'New message from Alice')

// Open file dialog
const files = await openFileDialog({
  multiple: true,
  filters: [{ name: 'Images', extensions: ['png', 'jpg'] }]
})
```

## 🧪 Testing

### Test API Connection

```bash
curl http://localhost:8547/api/health
```

### Test Authentication

```bash
curl -X POST http://localhost:8547/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@evolveapp.com","password":"password123"}'
```

Should return:
```json
{
  "access_token": "eyJ0eXAiOiJKV1...",
  "refresh_token": "def50200...",
  "expires_in": 3600
}
```

## 📚 Available Modules

Modules are loaded from `src/modules/`:

1. **Email** - Email management (IMAP/SMTP)
2. **Calendar** - Event scheduling
3. **CRM** - Contact & deal management
4. **Helpdesk** - Ticket system (future)
5. **Knowledgebase** - Documentation (future)
6. **Chat** - Messaging (future)

## 🏗️ Building for Production

### Web Build

```bash
npm run build
```

Output: `dist/` directory - Deploy to any web server

### Desktop Build

```bash
# Requires system dependencies
npm run tauri:build
```

Output:
- `src-tauri/target/release/evolve-desktop` - Binary
- `src-tauri/target/release/bundle/deb/` - Debian package
- `src-tauri/target/release/bundle/appimage/` - AppImage

## 🐛 Troubleshooting

### Backend API not responding

```bash
cd /home/john/sources/eiq-manager
docker compose ps  # Check if containers are running
docker compose logs php  # Check PHP logs
```

### Frontend can't connect to API

1. Check `.env` has correct API_URL
2. Verify eiq-manager is running
3. Check CORS settings in eiq-manager
4. Try: `curl http://localhost:8547`

### Database connection error

1. Check `.env.local` in eiq-manager
2. Verify DigitalOcean database is accessible
3. Test connection: `docker compose exec php bin/console doctrine:database:connect`

### Tauri won't start

1. Install system dependencies: `sudo bash install-tauri-deps.sh`
2. Install Tauri CLI: `. "$HOME/.cargo/env" && cargo install tauri-cli`
3. Run: `npm run tauri:dev`

## 📖 Documentation

- **ARCHITECTURE.md** - Complete architecture documentation
- **TAURI_SETUP.md** - Tauri integration guide
- **README.md** - Project overview
- **docs/** - Module implementation guides

## 🎯 Next Steps

### 1. Start Backend
```bash
cd /home/john/sources/eiq-manager && docker compose up -d
```

### 2. Start Frontend
```bash
cd /home/john/sources/evolveapp/evolve-desktop && npm run dev
```

### 3. Access App
Open http://localhost:5173 in your browser

### 4. Login
Use your eiq-manager credentials

### 5. Start Building Modules
Begin with Email module implementation (see `docs/MVP_IMPLEMENTATION_GUIDE.md`)

## ✅ You're Ready!

Your development environment is fully configured:
- ✅ Backend API running (eiq-manager)
- ✅ Database connected (PostgreSQL)
- ✅ Frontend ready (Vue 3 + Tauri)
- ✅ Authentication configured (JWT)
- ✅ All dependencies installed

Just start the services and begin coding!

---

**Questions?** Check the docs or review the code architecture.
