# EvolveApp Desktop

A modular desktop application built with Tauri, Vue 3, and TypeScript for managing business operations including Email, Calendar, CRM, and 37+ additional modules.

## 🎯 Project Status

**Phase 1: Frontend Foundation** - ✅ COMPLETED
- Vue 3 + TypeScript + Vite setup
- Tailwind CSS v4 + DaisyUI configuration
- Plugin system architecture
- Pinia state management
- Vue Router configuration
- Base UI components and layouts
- Authentication flow

**Phase 2: Tauri Integration** - ✅ COMPLETED (structure ready, needs system deps to run)
**Phase 3: Module Development** - 📋 READY TO START
**Phase 4: Backend Integration** - ✅ API CONFIGURED (eiq-manager on port 8547)

## 🏗️ Architecture

### Technology Stack

**Frontend:**
- **Framework:** Vue 3 (Composition API)
- **Language:** TypeScript
- **Build Tool:** Vite 7.2.2
- **Styling:** Tailwind CSS v4 + DaisyUI 5.2
- **State Management:** Pinia
- **Router:** Vue Router 4
- **HTTP Client:** Axios
- **Date Utils:** date-fns
- **Icons:** Heroicons

**Desktop Runtime:**
- **Framework:** Tauri 2.x (pending system dependencies)
- **Backend:** Rust with tokio async runtime

**Backend API:**
- **Framework:** Symfony 7.3 (PHP)
- **Database:** PostgreSQL 17 (DigitalOcean - **REMOTE ONLY**)
- **Auth:** JWT (LexikJWTAuthenticationBundle)
- **Port:** 8547 (via Docker Compose)

**Database Strategy:**
- ✅ **Remote Database Only** - All data stored in eiq-manager PostgreSQL
- ❌ **No Local SQLite** - Removed from architecture
- ❌ **No Offline Storage** - Requires internet connection
- ✅ **Direct API Calls** - Desktop app → eiq-manager API → PostgreSQL
- ✅ **Real-time Data** - Always fresh from server

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.

### Plugin System

The application uses a modular plugin architecture allowing dynamic loading of modules:

```typescript
interface ModulePlugin {
  metadata: ModuleMetadata
  install(): Promise<void>
  getMainView(): Component
  getRoutes(): RouteRecordRaw[]
  getSchema(): ModuleSchema
  search?(query: string): Promise<SearchResult[]>
}
```

**Core Components:**
- `ModuleRegistry` - Manages module registration and lifecycle
- `ModuleLoader` - Handles dynamic module loading
- `EventBus` - Cross-module communication
- `ServiceRegistry` - Dependency injection container

## 📁 Project Structure

```
evolve-desktop/
├── src/
│   ├── core/                    # Core systems
│   │   ├── plugin-system/       # Module loading & registry
│   │   ├── event-bus/           # Global event system
│   │   ├── service-registry/    # DI container
│   │   └── types/               # TypeScript types
│   ├── stores/                  # Pinia stores
│   │   ├── auth.ts              # Authentication
│   │   └── app.ts               # Global UI state
│   ├── router/                  # Vue Router
│   │   └── index.ts             # Route configuration
│   ├── layouts/                 # Layout components
│   │   └── MainLayout.vue       # Main app layout
│   ├── views/                   # Page components
│   │   ├── Login.vue
│   │   ├── Dashboard.vue
│   │   ├── Settings.vue
│   │   └── NotFound.vue
│   ├── components/              # Reusable components
│   │   ├── sidebar/
│   │   └── common/
│   ├── modules/                 # Module plugins (to be added)
│   ├── App.vue                  # Root component
│   ├── main.ts                  # App entry point
│   └── style.css                # Global styles
├── public/                      # Static assets
├── .env.example                 # Environment template
├── package.json
├── tsconfig.json
├── vite.config.ts
└── tailwind.config.js
```

## 🚀 Getting Started

### Prerequisites

**Installed:**
- ✅ Node.js v20.19.5
- ✅ npm 10.8.2
- ✅ Rust 1.91.1 / Cargo 1.91.1

**Required (for Tauri):**
- ❌ build-essential
- ❌ libssl-dev
- ❌ libgtk-3-dev
- ❌ libwebkit2gtk-4.1-dev
- ❌ libappindicator3-dev
- ❌ librsvg2-dev
- ❌ patchelf

### Installation

1. **Install System Dependencies** (requires sudo):
```bash
sudo apt-get update
sudo apt-get install -y build-essential curl wget libssl-dev \
  libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev \
  librsvg2-dev patchelf
```

2. **Install Tauri CLI:**
```bash
. "$HOME/.cargo/env"
cargo install tauri-cli --version "^2.0.0"
```

3. **Install Node Dependencies:**
```bash
cd /home/john/sources/evolveapp/evolve-desktop
npm install
```

4. **Configure Environment:**
```bash
cp .env.example .env
# Edit .env with your API URL
```

### Development

**Start Vue Development Server:**
```bash
npm run dev
```
- Opens at http://localhost:5173
- Hot module replacement enabled
- Vue DevTools compatible

**Start with Tauri** (after system dependencies installed):
```bash
npm run tauri dev
```

### Build

**Web Build:**
```bash
npm run build
```

**Desktop Build** (after Tauri setup):
```bash
npm run tauri build
```

## 🎨 Features Implemented

### ✅ Core Application
- [x] Vue 3 + TypeScript project setup
- [x] Tailwind CSS v4 + DaisyUI styling
- [x] Responsive layout with sidebar navigation
- [x] Dark/Light theme support
- [x] Global search interface
- [x] Notification system

### ✅ Plugin System
- [x] Module registry and loader
- [x] Dynamic route registration
- [x] Event bus for cross-module communication
- [x] Service registry for dependency injection
- [x] TypeScript interfaces and types

### ✅ Authentication
- [x] Login page with JWT support
- [x] Auth state management (Pinia)
- [x] Token refresh mechanism
- [x] Protected routes
- [x] Axios interceptors

### ✅ UI Components
- [x] Main layout with sidebar
- [x] Module navigation
- [x] Notification dropdown
- [x] Global search modal
- [x] User menu
- [x] Settings page

### 📋 Pending Implementation

**Phase 2: Tauri Integration**
- [ ] Initialize Tauri project
- [ ] Configure window settings
- [ ] System tray integration
- [ ] Native notifications
- [ ] Auto-updater

**Phase 3: Core Modules**
- [ ] Email module (IMAP/SMTP)
- [ ] Calendar module
- [ ] CRM module
- [ ] Module settings UI

**Phase 4: Backend Integration**
- [ ] Symfony API setup
- [ ] Database migrations
- [ ] API endpoints
- [ ] Sync engine

## 📚 Documentation

Comprehensive documentation is available in `/docs`:

- `PLUGIN_SYSTEM_ARCHITECTURE.md` - Plugin system design
- `MVP_IMPLEMENTATION_GUIDE.md` - 6-week implementation roadmap
- `SYMFONY_API_IMPLEMENTATION.md` - Backend API code
- `UI_DESIGN_SYSTEM.md` - UI/UX specifications
- `EIQ_DESKTOP_EXTENDED_ARCHITECTURE.md` - Complete system architecture
- `MODULE_PRIORITY_MATRIX.md` - Module prioritization
- `CRM_API_SPECIFICATION.md` - CRM API reference

## 🔧 Configuration

### API Configuration

Create `.env` from `.env.example`:
```env
VITE_API_URL=http://localhost:8000
```

### Theme Configuration

Themes are configured in `tailwind.config.js`. DaisyUI provides:
- Light theme (default)
- Dark theme
- Additional themes: cupcake, corporate

### Router Configuration

Routes are defined in `src/router/index.ts`:
- `/login` - Authentication
- `/dashboard` - Main dashboard
- `/settings` - Application settings
- Module routes added dynamically

## 🧪 Testing

```bash
# Run tests (when implemented)
npm run test

# Run E2E tests (when implemented)
npm run test:e2e
```

## 📝 API Integration

The application connects to a Symfony backend:

```typescript
// Axios is configured in src/main.ts
axios.defaults.baseURL = import.meta.env.VITE_API_URL

// Example API call
const response = await axios.post('/api/v1/auth/login', {
  email: 'user@example.com',
  password: 'password123'
})
```

## 🔐 Security

- JWT authentication with automatic token refresh
- Protected routes via navigation guards
- CORS configured on backend
- XSS protection via Vue's template escaping
- CSRF protection on API endpoints

## 🐛 Known Issues

1. **Tauri CLI Installation Failed** - Requires system dependencies (see Prerequisites)
2. **Vite Version Conflict** - Tailwind v4 requires Vite 5-6, using `--legacy-peer-deps`

## 📦 NPM Scripts

```json
{
  "dev": "vite",                    // Start dev server
  "build": "vue-tsc -b && vite build",  // Build for production
  "preview": "vite preview",        // Preview production build
  "tauri": "tauri"                  // Tauri commands
}
```

## 🤝 Contributing

This is a custom business application. For module development:

1. Create module in `src/modules/[module-name]/`
2. Implement `ModulePlugin` interface
3. Export as default from `index.ts`
4. Register in `src/main.ts`

## 📄 License

Proprietary - Internal use only

## 👥 Team

- Project: EvolveApp Extended Vision
- Created: November 2025
- Built with: Vue 3 + Tauri + Symfony

---

**Next Steps:**
1. Install system dependencies (requires sudo)
2. Complete Tauri integration
3. Implement Email module (Week 1-2)
4. Implement Calendar module (Week 3-4)
5. Implement CRM module (Week 5-6)

For detailed implementation guides, see `/docs/MVP_IMPLEMENTATION_GUIDE.md`
