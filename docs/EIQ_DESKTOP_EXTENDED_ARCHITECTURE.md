# EIQ Desktop - Extended Multi-Module Architecture

**Product Name**: EIQ Desktop (formerly EvolveMailPro)
**Vision**: Complete business management desktop application
**Status**: Architecture Design Phase
**Date**: 2025-11-14

---

## 🎯 Executive Summary

EIQ Desktop is a **comprehensive cross-platform business management application** that brings the full power of the EIQ Manager platform to Windows, macOS, and Linux. Rather than just an email client, it's a complete business productivity suite similar to Microsoft Office or Salesforce Desktop.

### From Email Client to Business Suite

**Original Vision**: Email client with calendar
**Extended Vision**: Full business management suite with 37+ integrated modules

---

## 📊 Module Discovery Results

Based on exploration of `/home/john/sources/eiq-manager`, we identified **37 core modules**:

### Tier 1: High-Priority Desktop Modules
Essential modules that benefit most from offline-first desktop experience:

1. **Email Manager** ⭐⭐⭐⭐⭐
   - Full IMAP/SMTP email client
   - Calendar sync
   - Contact integration
   - **Desktop Priority**: CRITICAL - Core functionality

2. **CRM Marketing** ⭐⭐⭐⭐⭐
   - Contact management
   - Deal pipeline
   - Lead scoring
   - Email campaigns
   - **Desktop Priority**: CRITICAL - Sales teams need offline access

3. **Helpdesk/Ticketing** ⭐⭐⭐⭐
   - Support ticket management
   - Department routing
   - SLA tracking
   - **Desktop Priority**: HIGH - Support teams benefit from native app

4. **Knowledgebase** ⭐⭐⭐⭐
   - Article management
   - Full-text search
   - Offline reading
   - **Desktop Priority**: HIGH - Content creators need offline access

5. **Chatbot Management** ⭐⭐⭐
   - AI chatbot profiles
   - Conversation monitoring
   - Training data management
   - **Desktop Priority**: MEDIUM - Management interface

### Tier 2: Medium-Priority Modules
Modules that enhance productivity but work well in browser too:

6. **Agreements & Applications** ⭐⭐⭐⭐
   - Digital agreements
   - E-signatures
   - Proposal tracking
   - **Desktop Priority**: MEDIUM - Document signing benefits from desktop

7. **Video Meeting & Recording** ⭐⭐⭐
   - Meeting scheduling
   - Recording management
   - **Desktop Priority**: MEDIUM - Integration with email/calendar

8. **EvolveWriter** ⭐⭐⭐
   - Collaborative book writing
   - Real-time editing
   - Change tracking
   - **Desktop Priority**: MEDIUM - Writers benefit from native app

9. **File Manager** ⭐⭐⭐
   - File organization
   - DigitalOcean Spaces integration
   - **Desktop Priority**: MEDIUM - File sync capabilities

10. **Social Sharing** ⭐⭐⭐
    - Multi-platform posting
    - Post scheduling
    - **Desktop Priority**: MEDIUM - Social media managers

### Tier 3: Admin & Configuration Modules
Admin tools better suited for web interface:

11. **Ecommerce** ⭐⭐
    - Product catalog
    - Order management
    - Payment processing
    - **Desktop Priority**: LOW - Web-based shopping works well

12. **Email Hosting** ⭐⭐
    - Domain management
    - Email user provisioning
    - **Desktop Priority**: LOW - Admin-focused

13. **Hosting Management** ⭐⭐
    - Web hosting provisioning
    - Resource monitoring
    - **Desktop Priority**: LOW - Server management

14. **Page Builder** ⭐
    - Drag-and-drop website builder
    - **Desktop Priority**: LOW - Complex visual editor better in browser

15. **System Settings** ⭐
    - App configuration
    - Admin dashboard
    - **Desktop Priority**: LOW - Admin interface

### Tier 4: Integration & Background Services
API-based modules that work in background:

16. **Automation/Workflow** ⭐⭐⭐⭐
    - Workflow engine
    - Campaign automation
    - **Desktop Priority**: MEDIUM - Trigger management

17. **Analytics** ⭐⭐⭐
    - Matomo integration
    - Email tracking
    - **Desktop Priority**: MEDIUM - Reporting dashboard

18. **Integrations Platform** ⭐⭐⭐
    - 17+ connectors
    - Webhook management
    - **Desktop Priority**: MEDIUM - Integration management

19. **SMS Management** ⭐⭐
    - SMS campaigns
    - Delivery tracking
    - **Desktop Priority**: LOW - Background service

20. **LinkedIn/Zoom Integration** ⭐⭐
    - Event sync
    - Lead capture
    - **Desktop Priority**: LOW - OAuth integrations

---

## 🏗️ Extended Desktop Architecture

### Modular Plugin System

```
┌─────────────────────────────────────────────────────────────┐
│                    EIQ Desktop Application                   │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Core Shell (Vue 3 + Tauri)                  │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • Window Management                                  │  │
│  │  • Authentication                                     │  │
│  │  • Module Launcher                                    │  │
│  │  • Settings & Preferences                             │  │
│  │  • Notification Center                                │  │
│  │  • Global Search                                      │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Module Plugin System                        │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │                                                        │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐             │  │
│  │  │  Email   │ │   CRM    │ │ Helpdesk │  [Core]     │  │
│  │  │  Module  │ │  Module  │ │  Module  │             │  │
│  │  └──────────┘ └──────────┘ └──────────┘             │  │
│  │                                                        │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐             │  │
│  │  │Knowledge │ │  Video   │ │ Chatbot  │  [Enhanced] │  │
│  │  │   Base   │ │ Meeting  │ │  Mgmt    │             │  │
│  │  └──────────┘ └──────────┘ └──────────┘             │  │
│  │                                                        │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐             │  │
│  │  │Analytics │ │Workflow  │ │File Mgr  │  [Optional] │  │
│  │  │Dashboard │ │ Engine   │ │          │             │  │
│  │  └──────────┘ └──────────┘ └──────────┘             │  │
│  │                                                        │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         Shared Services Layer (Rust)                  │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • SQLite Database (unified schema)                   │  │
│  │  • Background Sync Service                            │  │
│  │  • Queue Manager (offline operations)                 │  │
│  │  • OS Integration (notifications, tray)               │  │
│  │  • Secure Storage (tokens, keys)                      │  │
│  │  • Search Index (Tantivy full-text search)            │  │
│  │  • File Cache (attachments, documents)                │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         Platform API Client                           │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • JWT Authentication                                 │  │
│  │  • REST API Client (all modules)                      │  │
│  │  • Real-time Updates (SSE/WebSocket)                  │  │
│  │  • Module-specific endpoints                          │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           ↕
                  Internet Connection
                           ↕
┌─────────────────────────────────────────────────────────────┐
│          Symfony Platform (eiq-manager) REST API             │
│                                                               │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Email API  │  CRM API  │  Ticket API  │  KB API     │  │
│  │  Video API  │  Chat API │  File API    │  Analytics  │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 🧩 Module Plugin Architecture

### Plugin System Design

Each module is a **self-contained plugin** with:

```typescript
// Module Plugin Interface
interface ModulePlugin {
  // Metadata
  id: string;              // 'email', 'crm', 'helpdesk'
  name: string;            // 'Email Manager'
  icon: string;            // Icon identifier
  version: string;         // '1.0.0'
  category: ModuleCategory; // 'core', 'business', 'admin'

  // Lifecycle
  install(): Promise<void>;
  uninstall(): Promise<void>;
  activate(): Promise<void>;
  deactivate(): Promise<void>;

  // UI Components
  getMainView(): Component;       // Main module view
  getSidebarWidget?(): Component; // Sidebar integration
  getQuickActions?(): Action[];   // Quick action buttons

  // Database Schema
  getDatabaseSchema(): Schema;
  getMigrations(): Migration[];

  // API Integration
  getApiEndpoints(): ApiEndpoint[];

  // Settings
  getSettingsView?(): Component;
  getDefaultSettings(): Settings;

  // Search Integration
  provideSearchResults?(query: string): SearchResult[];

  // Notification Handling
  handleNotification?(notification: Notification): void;
}
```

### Core Modules (Always Installed)

1. **Email Module**
   - Email accounts
   - Folders & messages
   - Composer
   - Calendar integration

2. **Dashboard Module**
   - Overview widgets
   - Activity feed
   - Quick stats

3. **Settings Module**
   - Account settings
   - Module management
   - Preferences

### Optional Modules (User Installs)

Users can install additional modules based on their needs:

```
Available Modules Store:
┌─────────────────────────────────────┐
│ 📧 Email Manager          [Installed]│
│ 👥 CRM Marketing         [Install] │
│ 🎫 Helpdesk              [Install] │
│ 📚 Knowledgebase         [Install] │
│ 🤖 Chatbot Management    [Install] │
│ 📹 Video Meetings        [Install] │
│ ✍️  EvolveWriter          [Install] │
│ 📊 Analytics Dashboard   [Install] │
│ ⚙️  Workflow Automation   [Install] │
└─────────────────────────────────────┘
```

---

## 💾 Unified Database Schema

### Multi-Module SQLite Database

```sql
-- ============================================
-- CORE: Authentication & User
-- ============================================
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY,
    platform_user_id INTEGER NOT NULL,
    email TEXT NOT NULL,
    full_name TEXT,
    jwt_token TEXT,
    refresh_token TEXT,
    token_expires_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ============================================
-- MODULE: Email Manager
-- ============================================
CREATE TABLE email_accounts (
    id INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    email_address TEXT NOT NULL,
    provider TEXT DEFAULT 'smartermail',
    is_active BOOLEAN DEFAULT 1
);

CREATE TABLE email_messages (
    id INTEGER PRIMARY KEY,
    folder_id INTEGER REFERENCES email_folders(id),
    subject TEXT,
    body_html TEXT,
    from_address TEXT,
    received_date DATETIME,
    is_read BOOLEAN DEFAULT 0
);

CREATE TABLE calendar_events (
    id INTEGER PRIMARY KEY,
    calendar_account_id INTEGER,
    title TEXT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    attendees TEXT -- JSON
);

-- ============================================
-- MODULE: CRM Marketing
-- ============================================
CREATE TABLE crm_contacts (
    id INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    server_contact_id INTEGER,
    first_name TEXT,
    last_name TEXT,
    email TEXT,
    phone TEXT,
    company TEXT,
    lead_score INTEGER DEFAULT 0,
    tags TEXT, -- JSON array
    custom_fields TEXT, -- JSON object
    last_contacted_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE crm_deals (
    id INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    server_deal_id INTEGER,
    contact_id INTEGER REFERENCES crm_contacts(id),
    title TEXT NOT NULL,
    value DECIMAL(10,2),
    stage TEXT, -- 'lead', 'qualified', 'proposal', 'won', 'lost'
    probability INTEGER, -- 0-100
    expected_close_date DATE,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE crm_companies (
    id INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    server_company_id INTEGER,
    name TEXT NOT NULL,
    website TEXT,
    industry TEXT,
    employee_count INTEGER,
    annual_revenue DECIMAL(15,2),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE crm_activities (
    id INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    contact_id INTEGER REFERENCES crm_contacts(id),
    deal_id INTEGER REFERENCES crm_deals(id),
    type TEXT, -- 'call', 'email', 'meeting', 'note'
    subject TEXT,
    description TEXT,
    completed BOOLEAN DEFAULT 0,
    due_date DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ============================================
-- MODULE: Helpdesk/Ticketing
-- ============================================
CREATE TABLE tickets (
    id INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    server_ticket_id INTEGER,
    ticket_number TEXT UNIQUE,
    subject TEXT NOT NULL,
    description TEXT,
    status TEXT, -- 'open', 'pending', 'resolved', 'closed'
    priority TEXT, -- 'low', 'normal', 'high', 'urgent'
    department_id INTEGER,
    assigned_to_id INTEGER,
    requester_email TEXT,
    requester_name TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    resolved_at DATETIME,
    closed_at DATETIME
);

CREATE TABLE ticket_messages (
    id INTEGER PRIMARY KEY,
    ticket_id INTEGER REFERENCES tickets(id),
    server_message_id INTEGER,
    message TEXT NOT NULL,
    is_internal BOOLEAN DEFAULT 0,
    from_email TEXT,
    from_name TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE ticket_departments (
    id INTEGER PRIMARY KEY,
    server_department_id INTEGER,
    name TEXT NOT NULL,
    email TEXT,
    description TEXT
);

-- ============================================
-- MODULE: Knowledgebase
-- ============================================
CREATE TABLE kb_categories (
    id INTEGER PRIMARY KEY,
    server_category_id INTEGER,
    name TEXT NOT NULL,
    description TEXT,
    parent_id INTEGER REFERENCES kb_categories(id),
    sort_order INTEGER DEFAULT 0
);

CREATE TABLE kb_articles (
    id INTEGER PRIMARY KEY,
    server_article_id INTEGER,
    category_id INTEGER REFERENCES kb_categories(id),
    title TEXT NOT NULL,
    content TEXT,
    author_id INTEGER,
    status TEXT, -- 'draft', 'published', 'archived'
    view_count INTEGER DEFAULT 0,
    helpful_count INTEGER DEFAULT 0,
    not_helpful_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    published_at DATETIME
);

-- ============================================
-- MODULE: Chatbot Management
-- ============================================
CREATE TABLE chatbot_profiles (
    id INTEGER PRIMARY KEY,
    server_profile_id INTEGER,
    name TEXT NOT NULL,
    provider TEXT, -- 'claude', 'openai'
    model TEXT, -- 'claude-3-opus', 'gpt-4'
    system_prompt TEXT,
    temperature REAL DEFAULT 0.7,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE chatbot_conversations (
    id INTEGER PRIMARY KEY,
    server_conversation_id INTEGER,
    profile_id INTEGER REFERENCES chatbot_profiles(id),
    user_email TEXT,
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    ended_at DATETIME,
    message_count INTEGER DEFAULT 0
);

-- ============================================
-- MODULE: Video Meetings
-- ============================================
CREATE TABLE video_meetings (
    id INTEGER PRIMARY KEY,
    server_meeting_id INTEGER,
    title TEXT NOT NULL,
    meeting_url TEXT,
    scheduled_start DATETIME,
    scheduled_end DATETIME,
    host_email TEXT,
    participants TEXT, -- JSON array
    recording_url TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ============================================
-- MODULE: File Manager
-- ============================================
CREATE TABLE files (
    id INTEGER PRIMARY KEY,
    server_file_id INTEGER,
    folder_id INTEGER REFERENCES file_folders(id),
    name TEXT NOT NULL,
    mime_type TEXT,
    size INTEGER,
    local_path TEXT,
    remote_url TEXT,
    is_downloaded BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME
);

CREATE TABLE file_folders (
    id INTEGER PRIMARY KEY,
    server_folder_id INTEGER,
    parent_id INTEGER REFERENCES file_folders(id),
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ============================================
-- SHARED: Sync Queue (All Modules)
-- ============================================
CREATE TABLE sync_queue (
    id INTEGER PRIMARY KEY,
    module TEXT NOT NULL, -- 'email', 'crm', 'tickets', etc.
    operation_type TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id INTEGER,
    payload TEXT, -- JSON
    retry_count INTEGER DEFAULT 0,
    last_error TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    processed_at DATETIME
);

-- ============================================
-- SHARED: Search Index (All Modules)
-- ============================================
CREATE VIRTUAL TABLE search_index USING fts5(
    module TEXT,
    entity_type TEXT,
    entity_id INTEGER,
    title TEXT,
    content TEXT,
    tags TEXT
);

-- ============================================
-- SHARED: Notifications (All Modules)
-- ============================================
CREATE TABLE notifications (
    id INTEGER PRIMARY KEY,
    module TEXT NOT NULL,
    type TEXT NOT NULL,
    title TEXT NOT NULL,
    message TEXT,
    data TEXT, -- JSON
    is_read BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ============================================
-- MODULE METADATA
-- ============================================
CREATE TABLE installed_modules (
    id INTEGER PRIMARY KEY,
    module_id TEXT UNIQUE NOT NULL,
    module_name TEXT NOT NULL,
    version TEXT NOT NULL,
    is_active BOOLEAN DEFAULT 1,
    installed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    settings TEXT -- JSON
);

CREATE TABLE module_sync_status (
    id INTEGER PRIMARY KEY,
    module_id TEXT NOT NULL,
    last_sync_at DATETIME,
    sync_status TEXT, -- 'idle', 'syncing', 'error'
    items_synced INTEGER DEFAULT 0,
    error_message TEXT
);
```

---

## 🔌 Extended Platform API Specification

### Base API Structure

```
/api/v1/
├── auth/
│   ├── token                  # POST - Login
│   ├── refresh                # POST - Refresh token
│   └── 2fa                    # POST - 2FA verification
├── user/
│   ├── profile                # GET, PUT
│   ├── preferences            # GET, PUT
│   └── modules                # GET - Available modules
│
├── email/                     # Email Module API
│   ├── accounts/              # Email accounts
│   ├── folders/               # Folders
│   ├── messages/              # Messages
│   └── calendar/              # Calendar events
│
├── crm/                       # CRM Module API
│   ├── contacts/              # Contact management
│   ├── companies/             # Company management
│   ├── deals/                 # Deal pipeline
│   ├── activities/            # Activities & tasks
│   └── segments/              # Contact segments
│
├── tickets/                   # Helpdesk Module API
│   ├── tickets/               # Ticket CRUD
│   ├── messages/              # Ticket messages
│   ├── departments/           # Departments
│   └── templates/             # Response templates
│
├── knowledgebase/             # KB Module API
│   ├── categories/            # Category management
│   ├── articles/              # Article CRUD
│   └── search/                # Article search
│
├── chatbot/                   # Chatbot Module API
│   ├── profiles/              # Chatbot profiles
│   ├── conversations/         # Conversation history
│   └── training/              # Training data
│
├── video/                     # Video Module API
│   ├── meetings/              # Meeting management
│   ├── recordings/            # Recording access
│   └── participants/          # Participant lists
│
├── files/                     # File Manager API
│   ├── folders/               # Folder navigation
│   ├── files/                 # File operations
│   └── upload/                # File upload
│
├── analytics/                 # Analytics API
│   ├── dashboard/             # Dashboard data
│   ├── reports/               # Report generation
│   └── tracking/              # Event tracking
│
└── sync/                      # Sync Management
    ├── status                 # GET - Overall sync status
    ├── trigger                # POST - Manual sync
    └── queue                  # GET - Queue status
```

---

## 📱 Desktop UI Design

### Main Window Layout

```
┌────────────────────────────────────────────────────────────────────┐
│  🏢 EIQ Desktop              [_] [□] [×]         john@example.com  │
├─────┬──────────────────────────────────────────────────────────────┤
│     │  ┌────────────────────────────────────────────────────┐     │
│ 📧  │  │                                                     │     │
│Email│  │         [Active Module Content Area]               │     │
│     │  │                                                     │     │
│ 👥  │  │  Shows the currently selected module:              │     │
│ CRM │  │  - Email: 3-pane layout                            │     │
│     │  │  - CRM: Contact list + details                     │     │
│ 🎫  │  │  - Tickets: Ticket list + conversation            │     │
│Desk │  │  - KB: Category tree + article viewer             │     │
│     │  │                                                     │     │
│ 📚  │  │                                                     │     │
│  KB │  │                                                     │     │
│     │  │                                                     │     │
│ 🤖  │  │                                                     │     │
│Chat │  │                                                     │     │
│     │  └────────────────────────────────────────────────────┘     │
│─────┤                                                              │
│ 📹  │  Status: ● Online | Last sync: 2 mins ago | 3 unread        │
│Video│                                                              │
│     │                                                              │
│ ⚙️  │                                                              │
│Set  │                                                              │
└─────┴──────────────────────────────────────────────────────────────┘
```

### Module Switcher

```
┌─────────────────────────────────┐
│  EIQ Desktop - Module Switcher  │
├─────────────────────────────────┤
│                                  │
│  Core Modules:                   │
│  📧 Email Manager        (12)    │
│  📅 Calendar             (3)     │
│  👥 CRM Marketing       (45)     │
│  🎫 Helpdesk            (8)      │
│                                  │
│  Productivity:                   │
│  📚 Knowledgebase               │
│  ✍️  EvolveWriter                │
│  📁 File Manager                 │
│                                  │
│  Communication:                  │
│  🤖 Chatbot Management          │
│  📹 Video Meetings              │
│                                  │
│  Analytics:                      │
│  📊 Dashboard & Reports         │
│  ⚙️  Workflow Automation         │
│                                  │
│  ─────────────────────────      │
│  + Add More Modules...           │
│                                  │
└─────────────────────────────────┘
```

---

## 🚀 Implementation Phases

### Phase 1: Foundation (Weeks 1-4)
**Platform API Development**

- ✅ JWT authentication
- ✅ Email API endpoints (existing)
- 🆕 CRM API endpoints
  - `/api/v1/crm/contacts` - CRUD
  - `/api/v1/crm/deals` - Pipeline
  - `/api/v1/crm/companies` - Company data
  - `/api/v1/crm/activities` - Tasks & notes
- 🆕 Helpdesk API endpoints
  - `/api/v1/tickets/tickets` - CRUD
  - `/api/v1/tickets/messages` - Replies
  - `/api/v1/tickets/departments` - Routing

**Desktop Core**
- Tauri project setup
- Vue 3 + TypeScript
- Module plugin system architecture
- SQLite database with multi-module schema
- Authentication flow

**Deliverable**: Desktop shell with Email + CRM modules

---

### Phase 2: Core Modules (Weeks 5-8)
**Email Module** (Enhanced from original plan)
- ✅ Basic email client
- ✅ Calendar integration
- 🆕 Contact sync with CRM
- 🆕 Email-to-ticket conversion

**CRM Module** (NEW)
- Contact list with search/filter
- Contact detail view
- Deal pipeline (Kanban board)
- Company management
- Activity tracking
- Offline contact creation

**Helpdesk Module** (NEW)
- Ticket list with filters
- Ticket detail + conversation view
- Reply to tickets
- Internal notes
- Department routing
- SLA indicators

**Deliverable**: Working Email, CRM, and Helpdesk modules

---

### Phase 3: Knowledge & Content (Weeks 9-11)
**Knowledgebase Module**
- Category tree navigation
- Article viewer with rich content
- Full-text search (offline)
- Article ratings
- Offline reading mode

**EvolveWriter Module**
- Book/document list
- Section editor
- Comment system
- Change tracking
- Collaborative editing indicators

**File Manager Module**
- Folder navigation
- File preview
- Upload/download
- File caching for offline access

**Deliverable**: Content management modules operational

---

### Phase 4: Communication & AI (Weeks 12-14)
**Chatbot Management Module**
- Chatbot profile management
- Conversation monitoring
- Training data viewer
- Analytics dashboard

**Video Meeting Module**
- Meeting scheduler
- Recording list & playback
- Calendar integration
- Participant management

**Social Sharing Module**
- Post composer
- Multi-platform publishing
- Scheduling queue
- Analytics

**Deliverable**: Communication tools integrated

---

### Phase 5: Analytics & Automation (Weeks 15-17)
**Analytics Dashboard Module**
- Custom dashboard builder
- Real-time metrics
- Report generation
- Chart visualizations

**Workflow Automation Module**
- Workflow list
- Trigger management
- Action configuration
- Execution logs

**Integration Management**
- Connected services list
- OAuth management
- Webhook configuration

**Deliverable**: Power user features complete

---

### Phase 6: Polish & Distribution (Weeks 18-20)
**Performance Optimization**
- Lazy module loading
- Database indexing
- Memory management
- Startup time optimization

**Native Integration**
- OS-specific features
- System tray enhancements
- Keyboard shortcuts
- Theme support

**Auto-Update System**
- Update distribution
- Module updates
- Rollback capability

**Deliverable**: Production-ready release

---

## 💡 Key Technical Decisions

### 1. **Modular Plugin System**
**Why**: Allows users to install only modules they need
**How**: Each module is a separate Vue 3 plugin with its own routes, components, and data

### 2. **Unified SQLite Database**
**Why**: Simpler than multiple databases, enables cross-module queries
**How**: Organized by module namespaces, shared tables for common features

### 3. **Offline-First Everything**
**Why**: Makes desktop app valuable vs web interface
**How**: All CRUD operations local-first, background sync

### 4. **Cross-Module Integration**
**Examples**:
- Email → CRM: Auto-create contacts from emails
- Email → Tickets: Convert emails to support tickets
- Calendar → CRM: Link meetings to deals
- Chatbot → KB: AI answers from knowledge base

### 5. **Progressive Module Installation**
**Why**: Reduces initial download size, user choice
**How**: Core modules included, optional modules downloadable

---

## 📊 Comparison: Desktop vs Web

| Feature | Web Platform | Desktop App | Winner |
|---------|-------------|-------------|--------|
| **Offline Access** | ❌ No | ✅ Full offline | 🏆 Desktop |
| **Performance** | ⚠️ Network dependent | ✅ Native speed | 🏆 Desktop |
| **System Integration** | ❌ Limited | ✅ Full OS integration | 🏆 Desktop |
| **Notifications** | ⚠️ Browser only | ✅ Native system | 🏆 Desktop |
| **File Access** | ❌ Upload only | ✅ Direct file system | 🏆 Desktop |
| **Updates** | ✅ Instant | ⚠️ Download required | 🏆 Web |
| **Accessibility** | ✅ Any device | ⚠️ Install required | 🏆 Web |
| **Admin Tasks** | ✅ Full access | ⚠️ Limited | 🏆 Web |

**Recommendation**: Desktop for daily work, Web for admin/config

---

## 🎯 Success Metrics

### MVP Success (Phase 2)
- [ ] 3 core modules working (Email, CRM, Helpdesk)
- [ ] Offline mode fully functional
- [ ] Sync works reliably
- [ ] < 5 second app launch
- [ ] Cross-platform builds

### Full Launch Success (Phase 6)
- [ ] 10+ modules available
- [ ] Module marketplace functional
- [ ] 10,000+ records synced smoothly
- [ ] < 3 second module switching
- [ ] Auto-updates working
- [ ] 95% crash-free sessions
- [ ] Positive beta user feedback

---

## 📦 Distribution Model

### Editions

**Free Edition**
- Email module
- CRM module (100 contacts limit)
- Helpdesk module (10 active tickets)

**Professional Edition** ($19/month)
- All modules unlocked
- Unlimited data
- Priority sync
- Advanced features

**Enterprise Edition** ($49/month)
- Everything in Professional
- Custom modules
- API access
- Advanced security
- Priority support

---

## 🔄 Migration from Web

**Smooth Transition Plan**:
1. User installs desktop app
2. Signs in with existing credentials
3. Initial sync downloads last 90 days of data
4. User can continue using web platform
5. Both sync to same backend
6. No data loss, seamless experience

---

## 📝 Summary

This extended architecture transforms EvolveMailPro from a simple email client into **EIQ Desktop** - a comprehensive business management suite that brings the full power of 37+ EIQ Manager modules to a native desktop experience.

### Key Benefits

1. **Offline-First**: Work anywhere, sync when online
2. **Modular**: Install only modules you need
3. **Fast**: Native performance, no browser overhead
4. **Integrated**: Modules work together seamlessly
5. **Secure**: OS-native security, encrypted local storage
6. **Cross-Platform**: Windows, macOS, Linux from one codebase

### Next Steps

1. **Review & Approve** this extended architecture
2. **Prioritize Modules** - Which modules first?
3. **API Development** - Extend Symfony backend
4. **Desktop Development** - Build plugin system
5. **Beta Testing** - Internal team first

---

**Document Version**: 1.0
**Last Updated**: 2025-11-14
**Author**: Claude (AI Assistant)
**Status**: Architecture Design - Awaiting Approval

---

**Estimated Timeline**: 20 weeks (5 months) for full release
**Team Size**: 2-3 developers full-time
**Technology**: Tauri + Vue 3 + Symfony 7.3
**Target Platforms**: Windows 10/11, macOS 10.15+, Linux (Ubuntu/Fedora)
