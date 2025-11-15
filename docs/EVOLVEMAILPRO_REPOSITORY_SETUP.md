# EvolveMailPro - Repository Setup Guide

**Product**: EvolveMailPro (Professional Desktop Email Client)
**Architecture**: Separate Repository Strategy
**Date**: 2025-10-30

---

## 📁 Repository Structure

### Overview

EvolveMailPro follows a **two-repository architecture** for clean separation between backend API and desktop client:

```
Repository 1: eiq-manager (existing)
└── Backend platform with REST API

Repository 2: evolve-mail-pro (new)
└── Desktop client application
```

---

## 🗂️ Repository 1: eiq-manager (Backend Platform)

**Location**: `/home/john/sources/eiq-manager/`
**Purpose**: Symfony backend platform with REST API for EvolveMailPro

### Structure

```
eiq-manager/
├── src/
│   ├── Controller/
│   │   └── Api/                          # NEW: REST API controllers
│   │       ├── AuthApiController.php     # JWT authentication
│   │       ├── EmailAccountApiController.php
│   │       ├── EmailMessageApiController.php
│   │       ├── EmailFolderApiController.php
│   │       └── CalendarApiController.php
│   ├── Service/
│   │   └── Email/                        # Existing email services (reused)
│   │       ├── DbalEmailSyncService.php
│   │       ├── SmarterMailApiService.php
│   │       └── EmailProviderInterface.php
│   └── Entity/
│       └── Email/                        # Existing email entities
│           ├── EmailAccount.php
│           ├── EmailMessage.php
│           ├── EmailFolder.php
│           └── EmailAttachment.php
├── config/
│   ├── packages/
│   │   ├── lexik_jwt_authentication.yaml # NEW: JWT config
│   │   └── nelmio_cors.yaml              # NEW: CORS for desktop app
│   └── routes/
│       └── api.yaml                      # NEW: API routes
├── docs/
│   ├── DESKTOP_EMAIL_CLIENT_ARCHITECTURE.md    # Architecture document
│   └── EVOLVEMAILPRO_REPOSITORY_SETUP.md       # This file
└── composer.json
```

### Key Changes for Phase 1

**New Dependencies**:
```bash
composer require lexik/jwt-authentication-bundle
composer require nelmio/cors-bundle
composer require api-platform/core  # Optional - for auto API docs
```

**New Routes** (`config/routes/api.yaml`):
```yaml
api:
    resource: '../src/Controller/Api/'
    type: attribute
    prefix: /api/v1
```

**CORS Configuration** (`config/packages/nelmio_cors.yaml`):
```yaml
nelmio_cors:
    defaults:
        origin_regex: true
        allow_origin: ['*']
        allow_methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS']
        allow_headers: ['Content-Type', 'Authorization']
        max_age: 3600
    paths:
        '^/api/':
            allow_origin: ['*']
            allow_headers: ['*']
            allow_methods: ['POST', 'PUT', 'GET', 'DELETE', 'OPTIONS']
            max_age: 3600
```

**JWT Configuration** (`config/packages/lexik_jwt_authentication.yaml`):
```yaml
lexik_jwt_authentication:
    secret_key: '%env(resolve:JWT_SECRET_KEY)%'
    public_key: '%env(resolve:JWT_PUBLIC_KEY)%'
    pass_phrase: '%env(JWT_PASSPHRASE)%'
    token_ttl: 3600
```

---

## 🗂️ Repository 2: evolve-mail-pro (Desktop Client)

**Location**: `/home/john/sources/evolve-mail-pro/` (to be created)
**Purpose**: Cross-platform desktop email client (Tauri + Vue 3)

### Structure

```
evolve-mail-pro/
├── src/                              # Vue 3 frontend
│   ├── main.ts
│   ├── App.vue
│   ├── router/
│   │   └── index.ts                  # Vue Router
│   ├── stores/                       # Pinia stores
│   │   ├── auth.ts                   # Authentication state
│   │   ├── email.ts                  # Email state
│   │   ├── calendar.ts               # Calendar state
│   │   └── sync.ts                   # Sync status
│   ├── services/                     # API clients
│   │   ├── api.ts                    # Base API client (Axios)
│   │   ├── auth.service.ts           # Authentication API
│   │   ├── email.service.ts          # Email API
│   │   ├── calendar.service.ts       # Calendar API
│   │   └── sync.service.ts           # Sync operations
│   ├── components/                   # Vue components
│   │   ├── email/
│   │   │   ├── FolderList.vue
│   │   │   ├── MessageList.vue
│   │   │   ├── MessageReader.vue
│   │   │   └── Composer.vue
│   │   ├── calendar/
│   │   │   ├── MonthView.vue
│   │   │   ├── WeekView.vue
│   │   │   └── EventEditor.vue
│   │   └── common/
│   │       ├── Navbar.vue
│   │       ├── Sidebar.vue
│   │       └── StatusBar.vue
│   ├── views/                        # Page views
│   │   ├── LoginView.vue
│   │   ├── EmailView.vue
│   │   ├── CalendarView.vue
│   │   └── SettingsView.vue
│   ├── composables/                  # Vue composables
│   │   ├── useAuth.ts
│   │   ├── useEmail.ts
│   │   └── useSync.ts
│   └── assets/
│       └── styles/
│           └── main.css              # Tailwind CSS
├── src-tauri/                        # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs               # Tauri commands
│   │   ├── database/
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs         # SQLite connection
│   │   │   ├── migrations.rs         # Database migrations
│   │   │   └── models.rs             # Database models
│   │   ├── sync/
│   │   │   ├── mod.rs
│   │   │   ├── sync_service.rs       # Background sync
│   │   │   └── queue.rs              # Offline queue
│   │   ├── storage/
│   │   │   ├── mod.rs
│   │   │   └── keyring.rs            # Secure token storage
│   │   └── notifications/
│   │       ├── mod.rs
│   │       └── notifier.rs           # OS notifications
│   ├── Cargo.toml
│   ├── tauri.conf.json               # Tauri configuration
│   └── icons/                        # App icons
├── prisma/
│   └── schema.prisma                 # SQLite schema
├── tests/
│   ├── unit/
│   ├── integration/
│   └── e2e/
├── package.json
├── tsconfig.json
├── tailwind.config.js
├── vite.config.ts
└── README.md
```

### Key Files

#### `package.json`
```json
{
  "name": "evolve-mail-pro",
  "version": "1.0.0",
  "description": "Professional cross-platform email client",
  "scripts": {
    "dev": "tauri dev",
    "build": "tauri build",
    "test": "vitest"
  },
  "dependencies": {
    "vue": "^3.4.0",
    "vue-router": "^4.2.5",
    "pinia": "^2.1.7",
    "axios": "^1.6.2",
    "@tauri-apps/api": "^1.5.3",
    "prosemirror-state": "^1.4.3",
    "prosemirror-view": "^1.32.7",
    "prosemirror-schema-basic": "^1.2.2"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^1.5.9",
    "@vitejs/plugin-vue": "^5.0.0",
    "typescript": "^5.3.3",
    "vite": "^5.0.10",
    "tailwindcss": "^3.4.0",
    "daisyui": "^4.4.0",
    "vitest": "^1.1.0",
    "@prisma/client": "^5.8.0"
  }
}
```

#### `src-tauri/Cargo.toml`
```toml
[package]
name = "evolve-mail-pro"
version = "1.0.0"
description = "Professional cross-platform email client"
authors = ["EvolveIQ"]
edition = "2021"

[dependencies]
tauri = { version = "1.5", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.30", features = ["bundled"] }
reqwest = { version = "0.11", features = ["json"] }
chrono = { version = "0.4", features = ["serde"] }
tauri-plugin-store = "0.1"
tauri-plugin-notification = "0.1"
tauri-plugin-sql = { version = "0.1", features = ["sqlite"] }
keyring = "2.2"
```

#### `src-tauri/tauri.conf.json`
```json
{
  "package": {
    "productName": "EvolveMailPro",
    "version": "1.0.0"
  },
  "build": {
    "distDir": "../dist",
    "devPath": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "open": true
      },
      "notification": {
        "all": true
      },
      "fs": {
        "scope": ["$APPDATA/evolve-mail-pro/*"]
      }
    },
    "bundle": {
      "active": true,
      "identifier": "com.evolveiq.emailpro",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "windows": {
        "certificateThumbprint": null,
        "wix": {
          "language": "en-US"
        }
      }
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "title": "EvolveMailPro",
        "width": 1280,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "systemTray": {
      "iconPath": "icons/icon.png",
      "iconAsTemplate": true
    }
  }
}
```

---

## 🔗 How They Connect

```
EvolveMailPro (Desktop)
         ↓
    REST API calls
         ↓
https://manager.evolvepreneuriq.app/api/v1/
         ↓
    eiq-manager (Backend)
         ↓
    SmarterMail Server
```

### Connection Configuration

**Desktop App** (`src/services/api.ts`):
```typescript
import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'https://manager.evolvepreneuriq.app/api/v1';

export const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add JWT token to all requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('jwt_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});
```

**Environment Variables** (`.env.development`):
```bash
VITE_API_BASE_URL=http://localhost:8000/api/v1  # Local development
```

**Environment Variables** (`.env.production`):
```bash
VITE_API_BASE_URL=https://manager.evolvepreneuriq.app/api/v1  # Production
```

---

## 🚀 Initial Setup Commands

### 1. Backend (eiq-manager)

```bash
cd /home/john/sources/eiq-manager

# Install new dependencies
composer require lexik/jwt-authentication-bundle
composer require nelmio/cors-bundle

# Generate JWT keys
php bin/console lexik:jwt:generate-keypair

# Create API controllers
mkdir -p src/Controller/Api
php bin/console make:controller Api/AuthApiController

# Restart containers
docker compose restart php web
```

### 2. Desktop Client (evolve-mail-pro)

```bash
cd /home/john/sources

# Create new Tauri project
npm create tauri-app@latest evolve-mail-pro

# Choose:
# - Package manager: npm
# - UI template: Vue + TypeScript

cd evolve-mail-pro

# Install dependencies
npm install
npm install tailwindcss daisyui postcss autoprefixer
npm install @prisma/client axios pinia vue-router

# Initialize Tailwind
npx tailwindcss init -p

# Install Tauri plugins
cd src-tauri
cargo add tauri-plugin-store
cargo add tauri-plugin-notification
cargo add tauri-plugin-sql --features sqlite
cargo add keyring
cd ..

# Initialize Prisma
npx prisma init --datasource-provider sqlite

# Run in development
npm run tauri dev
```

---

## 📋 Development Workflow

### Daily Development

**Backend Changes** (API endpoints):
```bash
cd /home/john/sources/eiq-manager
# Make changes to API controllers
docker compose restart php web
```

**Desktop Changes** (UI/features):
```bash
cd /home/john/sources/evolve-mail-pro
# Make changes to Vue components
# Auto-reload via Vite HMR
```

### Testing API from Desktop

**Option 1: Local Backend**
```bash
# Terminal 1: Run backend
cd /home/john/sources/eiq-manager
docker compose up

# Terminal 2: Run desktop app (points to localhost)
cd /home/john/sources/evolve-mail-pro
npm run tauri dev
```

**Option 2: Production Backend**
```bash
# Desktop app points to production API
cd /home/john/sources/evolve-mail-pro
echo "VITE_API_BASE_URL=https://manager.evolvepreneuriq.app/api/v1" > .env
npm run tauri dev
```

---

## 🔐 Environment Variables

### Backend (.env.local)

```bash
# JWT Keys
JWT_SECRET_KEY=%kernel.project_dir%/config/jwt/private.pem
JWT_PUBLIC_KEY=%kernel.project_dir%/config/jwt/public.pem
JWT_PASSPHRASE=your_passphrase_here

# Database
DATABASE_URL="postgresql://postgres:password@db:5432/webapp?serverVersion=17&charset=utf8"

# CORS (allow desktop app)
CORS_ALLOW_ORIGIN='^https?://(localhost|127\.0\.0\.1)(:[0-9]+)?$'
```

### Desktop (.env)

```bash
# API Base URL
VITE_API_BASE_URL=https://manager.evolvepreneuriq.app/api/v1

# App Configuration
VITE_APP_NAME=EvolveMailPro
VITE_APP_VERSION=1.0.0

# Feature Flags
VITE_ENABLE_CALENDAR=true
VITE_ENABLE_TASKS=true
```

---

## 📦 Building for Production

### Backend Deployment

```bash
cd /home/john/sources/eiq-manager
git pull origin main
composer install --no-dev --optimize-autoloader
sudo -u www-data php bin/console cache:clear --env=prod
sudo -u www-data php bin/console cache:warmup --env=prod
sudo systemctl restart php8.3-fpm
```

### Desktop Builds

**Windows**:
```bash
npm run tauri build -- --target x86_64-pc-windows-msvc
# Output: src-tauri/target/release/bundle/msi/EvolveMailPro_1.0.0_x64_en-US.msi
```

**macOS**:
```bash
npm run tauri build -- --target x86_64-apple-darwin
# Output: src-tauri/target/release/bundle/dmg/EvolveMailPro_1.0.0_x64.dmg
```

**Linux**:
```bash
npm run tauri build -- --target x86_64-unknown-linux-gnu
# Output: src-tauri/target/release/bundle/deb/evolve-mail-pro_1.0.0_amd64.deb
```

---

## 🧪 Testing Strategy

### Backend API Tests

```bash
cd /home/john/sources/eiq-manager
./vendor/bin/phpunit tests/Api/
```

### Desktop Unit Tests

```bash
cd /home/john/sources/evolve-mail-pro
npm run test
```

### E2E Tests

```bash
cd /home/john/sources/evolve-mail-pro
npm run test:e2e
```

---

## 📚 Documentation Structure

```
eiq-manager/docs/
├── DESKTOP_EMAIL_CLIENT_ARCHITECTURE.md    # Full architecture
├── EVOLVEMAILPRO_REPOSITORY_SETUP.md      # This file
├── API_REFERENCE.md                        # API documentation (to be created)
└── DEPLOYMENT_GUIDE.md                     # Deployment guide (to be created)

evolve-mail-pro/
├── README.md                               # Desktop app README
├── docs/
│   ├── DEVELOPMENT.md                      # Development guide
│   ├── USER_GUIDE.md                       # User manual
│   └── TROUBLESHOOTING.md                  # Common issues
```

---

## ✅ Checklist for Phase 1

### Backend (eiq-manager)
- [ ] Install `lexik/jwt-authentication-bundle`
- [ ] Generate JWT keypair
- [ ] Create `AuthApiController`
- [ ] Create `EmailAccountApiController`
- [ ] Create `EmailMessageApiController`
- [ ] Configure CORS for desktop app
- [ ] Test API endpoints with Postman
- [ ] Write API documentation

### Desktop (evolve-mail-pro)
- [ ] Create Tauri project
- [ ] Set up Vue 3 + TypeScript
- [ ] Configure Tailwind CSS + DaisyUI
- [ ] Set up Prisma with SQLite
- [ ] Create authentication flow
- [ ] Build login screen
- [ ] Implement API client service
- [ ] Test connection to backend API

---

## 🤝 Collaboration Workflow

### Git Branches

**Backend (eiq-manager)**:
- `main` - Production
- `feature/api-v1` - REST API development
- `feature/api-auth` - Authentication endpoints
- `feature/api-email` - Email endpoints

**Desktop (evolve-mail-pro)**:
- `main` - Stable releases
- `develop` - Active development
- `feature/login` - Login screen
- `feature/email-client` - Email client UI
- `feature/calendar` - Calendar feature

### Commit Messages

Use conventional commits:
```bash
feat(api): add JWT authentication endpoint
fix(desktop): resolve sync queue issue
docs(readme): update installation guide
chore(deps): upgrade Vue to 3.4.0
```

---

## 🎯 Next Steps

1. ✅ **Review this setup guide**
2. **Create `evolve-mail-pro` repository**
3. **Start Phase 1** (Backend REST API)
4. **Set up desktop project structure**
5. **Begin MVP development**

---

**Document Version**: 1.0
**Last Updated**: 2025-10-30
**Author**: Claude (AI Assistant)
**Status**: Ready for Implementation
