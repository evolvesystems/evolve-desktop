# EvolveMailPro

**Professional Cross-Platform Desktop Email Client**

> A modern, offline-first email client with calendar and task management for Windows, macOS, and Linux.

---

## 🎯 Overview

EvolveMailPro is a professional desktop email client that provides a complete Outlook-style experience with:

- ✅ **Offline-first email** - Read and compose emails without internet
- ✅ **Calendar & scheduling** - Full calendar with meeting invitations
- ✅ **Multiple accounts** - Manage multiple SmarterMail accounts
- ✅ **Native OS integration** - System notifications, tray icon, quick reply
- ✅ **Cross-platform** - Windows, macOS, and Linux from one codebase

---

## 🏗️ Technology Stack

### Frontend
- **Vue 3** + TypeScript
- **Tailwind CSS** + DaisyUI
- **ProseMirror** (rich text editor)
- **Pinia** (state management)
- **Vue Router**

### Backend (Desktop)
- **Tauri** (Rust)
- **SQLite** (local database)
- **Prisma ORM**
- **Tokio** (async runtime)

### Backend (Platform API)
- **Symfony 7.3** (PHP)
- **PostgreSQL 17**
- **JWT Authentication**
- **Mercure Hub** (real-time updates)

---

## 📁 Project Structure

```
evolvemailpro/
├── README.md                        # This file
├── docs/                            # Documentation
│   ├── DESKTOP_EMAIL_CLIENT_ARCHITECTURE.md
│   └── EVOLVEMAILPRO_REPOSITORY_SETUP.md
├── src/                             # Vue 3 frontend (to be created)
│   ├── components/
│   ├── views/
│   ├── stores/
│   └── services/
├── src-tauri/                       # Rust backend (to be created)
│   ├── src/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── prisma/                          # Database schema
│   └── schema.prisma
├── package.json
└── vite.config.ts
```

---

## 📚 Documentation

### Core Documentation
- **[Architecture](docs/DESKTOP_EMAIL_CLIENT_ARCHITECTURE.md)** - Complete system architecture, API specs, and implementation phases
- **[Repository Setup](docs/EVOLVEMAILPRO_REPOSITORY_SETUP.md)** - Two-repository strategy and development workflow

### Quick Links
- [Technology Stack](#-technology-stack)
- [Getting Started](#-getting-started)
- [Development Workflow](#-development-workflow)
- [API Integration](#-api-integration)
- [Build & Deploy](#-build--deploy)

---

## 🚀 Getting Started

### Prerequisites

**System Requirements**:
- Node.js 18+
- Rust 1.70+
- Platform-specific build tools (see [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

**Backend API**:
- Access to eiq-manager platform (REST API)
- Valid user credentials

### Installation

#### 1. Clone the repository
```bash
cd /home/john/sources/evolvemailpro
```

#### 2. Initialize Tauri project (First time only)
```bash
npm create tauri-app@latest .

# Choose:
# - Package manager: npm
# - UI template: Vue + TypeScript
# - UI flavor: vue-ts
```

#### 3. Install dependencies
```bash
# Install Node.js dependencies
npm install

# Install additional packages
npm install tailwindcss daisyui postcss autoprefixer
npm install axios pinia vue-router
npm install @prisma/client
npm install prosemirror-state prosemirror-view prosemirror-schema-basic

# Install Tauri plugins
cd src-tauri
cargo add tauri-plugin-store
cargo add tauri-plugin-notification
cargo add tauri-plugin-sql --features sqlite
cargo add keyring
cd ..
```

#### 4. Configure environment
```bash
# Create .env file
cat > .env <<EOF
VITE_API_BASE_URL=http://localhost:8000/api/v1
VITE_APP_NAME=EvolveMailPro
VITE_APP_VERSION=1.0.0
EOF
```

#### 5. Initialize Prisma
```bash
npx prisma init --datasource-provider sqlite
```

#### 6. Run development server
```bash
npm run tauri dev
```

---

## 💻 Development Workflow

### Daily Development

**Frontend changes** (Vue components, UI):
```bash
# Auto-reload via Vite HMR
npm run tauri dev
```

**Backend changes** (Rust, Tauri commands):
```bash
# Restart needed for Rust changes
# Stop with Ctrl+C, then:
npm run tauri dev
```

### Code Structure

**Frontend** (`src/`):
```
src/
├── main.ts                    # App entry point
├── App.vue                    # Root component
├── router/index.ts            # Vue Router
├── stores/                    # Pinia stores
│   ├── auth.ts               # Authentication
│   ├── email.ts              # Email state
│   └── sync.ts               # Sync status
├── services/                  # API clients
│   ├── api.ts                # Base API (Axios)
│   ├── auth.service.ts       # Auth endpoints
│   └── email.service.ts      # Email endpoints
├── components/                # Vue components
│   ├── email/
│   │   ├── FolderList.vue
│   │   ├── MessageList.vue
│   │   ├── MessageReader.vue
│   │   └── Composer.vue
│   └── common/
│       ├── Navbar.vue
│       └── Sidebar.vue
└── views/                     # Page views
    ├── LoginView.vue
    ├── EmailView.vue
    └── CalendarView.vue
```

**Backend** (`src-tauri/src/`):
```
src-tauri/src/
├── main.rs                    # Tauri entry point
├── commands.rs                # Tauri commands
├── database/                  # SQLite database
│   ├── connection.rs
│   └── models.rs
├── sync/                      # Background sync
│   ├── sync_service.rs
│   └── queue.rs
└── storage/
    └── keyring.rs             # Secure token storage
```

---

## 🔌 API Integration

### Backend API Endpoint

**Development**:
```
http://localhost:8000/api/v1
```

**Production**:
```
https://manager.evolvepreneuriq.app/api/v1
```

### Authentication Flow

```typescript
// Login
POST /api/v1/auth/token
{
  "email": "user@example.com",
  "password": "password"
}

// Response
{
  "token": "eyJhbGc...",
  "refresh_token": "def50200...",
  "expires_in": 3600
}
```

### Key Endpoints

**Email**:
- `GET /api/v1/email/accounts` - List accounts
- `GET /api/v1/email/folders/{id}/messages` - List messages
- `POST /api/v1/email/messages` - Send email

**Calendar**:
- `GET /api/v1/calendar/events` - List events
- `POST /api/v1/calendar/events` - Create event

See [full API specification](docs/DESKTOP_EMAIL_CLIENT_ARCHITECTURE.md#-platform-api-specification) for details.

---

## 🗄️ Local Database

### Schema (SQLite)

```sql
-- User accounts
accounts (id, email, jwt_token, refresh_token)

-- Email data
email_accounts (id, email_address, provider, server_host)
email_folders (id, account_id, name, path, type)
email_messages (id, folder_id, subject, body_html, from_address)
email_attachments (id, message_id, filename, local_path)

-- Calendar
calendar_accounts (id, email_account_id, name, color)
calendar_events (id, calendar_id, title, start_time, end_time)

-- Sync queue for offline operations
sync_queue (id, operation_type, entity_type, payload)
```

### Migrations

```bash
# Create migration
npx prisma migrate dev --name init

# Apply migrations
npx prisma migrate deploy
```

---

## 🎨 UI Components

### Layout

**Three-pane email layout**:
```
┌─────────────────────────────────────────────────┐
│  Navbar (Mail | Calendar | Tasks | Settings)    │
├───────┬─────────────┬───────────────────────────┤
│Folders│Message List │ Reading Pane              │
│       │             │                           │
│Inbox  │From: John   │Subject: Meeting tomorrow  │
│Sent   │Subject: ... │                           │
│Drafts │Date: ...    │Hi John,                  │
│Trash  │             │                           │
│       │From: Jane   │Just wanted to confirm... │
│       │Subject: ... │                           │
└───────┴─────────────┴───────────────────────────┘
```

### Components (DaisyUI)

- **Cards**: `<div class="card bg-base-100 shadow">`
- **Buttons**: `<button class="btn btn-primary">`
- **Inputs**: `<input class="input input-bordered">`
- **Modals**: `<dialog class="modal">`

See [DaisyUI docs](https://daisyui.com/components/) for all components.

---

## 🔐 Security

### Token Storage
- JWT tokens stored in OS-native keyring (secure)
- Never stored in plain text
- Auto-refresh before expiration

### Local Database
- SQLite database encrypted (SQLCipher)
- Master password or OS-derived key
- Secure deletion on account removal

### Transport
- All API calls over HTTPS
- Certificate pinning for production
- SSL certificate validation

---

## 🧪 Testing

### Unit Tests
```bash
npm run test
```

### E2E Tests
```bash
npm run test:e2e
```

### Manual Testing
```bash
# Login flow
# Send/receive email
# Offline mode
# Calendar events
```

---

## 📦 Build & Deploy

### Development Build
```bash
npm run tauri dev
```

### Production Builds

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

## 📋 Development Phases

### Phase 1: Platform REST API (Backend - Symfony)
**Duration**: 2-3 weeks
- JWT authentication
- Email API endpoints
- Calendar API endpoints
- CORS configuration

### Phase 2: Desktop MVP
**Duration**: 3-4 weeks
- Authentication flow
- Basic email client (read, send)
- Local database & sync
- Background sync service

### Phase 3: Full Email Features
**Duration**: 2-3 weeks
- Attachments
- Rich text editor
- Search
- Multiple accounts

### Phase 4: Calendar Integration
**Duration**: 2-3 weeks
- Calendar views
- Event management
- Meeting invitations
- Desktop notifications

### Phase 5: Polish & Native Integration
**Duration**: 2-3 weeks
- OS-specific features
- Performance optimization
- Security enhancements
- Auto-update system

**Total**: 12-16 weeks

---

## 🐛 Troubleshooting

### Common Issues

**Build errors on first run**:
```bash
# Clear cache and rebuild
rm -rf node_modules package-lock.json
npm install
npm run tauri dev
```

**API connection fails**:
```bash
# Check .env file
cat .env
# Should have: VITE_API_BASE_URL=http://localhost:8000/api/v1

# Test API directly
curl http://localhost:8000/api/v1/auth/token
```

**Rust build errors**:
```bash
# Update Rust
rustup update

# Clear Cargo cache
cd src-tauri
cargo clean
cd ..
npm run tauri dev
```

---

## 📞 Support & Contributing

### Getting Help
- Check [documentation](docs/)
- Review [architecture guide](docs/DESKTOP_EMAIL_CLIENT_ARCHITECTURE.md)
- Search existing issues

### Development Team
- Backend API: eiq-manager repository
- Desktop Client: This repository

---

## 📄 License

Copyright © 2025 EvolveIQ
All rights reserved.

---

## 🗺️ Roadmap

### v1.0 (MVP)
- [x] Architecture design
- [ ] Backend REST API
- [ ] Basic email client
- [ ] Local sync
- [ ] Login/authentication

### v1.1
- [ ] Attachments
- [ ] Rich text editor
- [ ] Search
- [ ] Multiple accounts

### v1.2
- [ ] Calendar integration
- [ ] Meeting invitations
- [ ] Desktop notifications

### v2.0
- [ ] Tasks & todos
- [ ] Contact management
- [ ] Email rules & filters
- [ ] Advanced search

---

**Status**: Planning Phase
**Version**: 1.0.0-alpha
**Last Updated**: 2025-10-30
