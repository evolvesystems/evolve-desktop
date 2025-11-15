# EvolveMailPro - Professional Desktop Email Client

**Product Name**: EvolveMailPro
**Status**: Planning Phase
**Target Platforms**: Windows, macOS, Linux (Cross-platform)
**Integration Model**: Platform-only (requires Symfony backend)
**Date**: 2025-10-30

---

## 🎯 Vision

Build **EvolveMailPro**, a professional cross-platform desktop email client that provides a full Outlook-style experience with offline-first email, calendar, and task management, syncing through the eiq-manager Symfony platform backend.

### Key Requirements
- ✅ Full Outlook-style replacement (email + calendar + tasks)
- ✅ Cross-platform (Windows, macOS, Linux)
- ✅ Platform-only integration (uses Symfony backend as API layer)
- ✅ Offline email access with local caching
- ✅ Calendar & scheduling with meeting invitations
- ✅ Multiple SmarterMail account support
- ✅ Native OS integration (notifications, tray, quick reply)

---

## 🏗️ Architecture Overview

### Technology Stack (Recommended: **Tauri**)

**Why Tauri over Electron?**
- 🚀 **Smaller binaries** (3-5 MB vs 100+ MB for Electron)
- ⚡ **Better performance** (native Rust backend)
- 🔒 **Enhanced security** (sandboxed by default)
- 💰 **Lower memory usage** (uses system WebView)
- 🎨 **Native OS integration** (system tray, notifications, menus)

**Frontend Stack**:
- **Vue 3** + TypeScript (matches existing Stimulus patterns)
- **Tailwind CSS** + **DaisyUI** (consistent with web platform)
- **ProseMirror** (same rich text editor as web platform)
- **Pinia** (state management)
- **Axios** (HTTP client for API calls)

**Backend Stack** (Desktop App):
- **Rust** (Tauri backend)
- **SQLite** with **Prisma ORM** (local database)
- **Tokio** (async runtime for background tasks)
- **Keyring** (secure credential storage)

**Platform Stack** (Symfony):
- **LexikJWTAuthenticationBundle** (JWT tokens)
- **API Platform** or custom REST controllers
- **Mercure Hub** (real-time updates via SSE)

---

## 📦 System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    EvolveMailPro                             │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Frontend (Vue 3 + Tailwind)                 │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • Email Client (inbox, folders, composer)            │  │
│  │  • Calendar View (month/week/day)                     │  │
│  │  • Tasks & Todos                                      │  │
│  │  • Contact Management                                 │  │
│  │  • Settings & Account Management                      │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Tauri Backend (Rust)                        │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • SQLite Database (local cache)                      │  │
│  │  • Background Sync Service                            │  │
│  │  • Queue Manager (offline operations)                 │  │
│  │  • OS Integration (notifications, tray)               │  │
│  │  • Secure Storage (encryption keys, tokens)           │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         Platform API Client                           │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • JWT Authentication                                 │  │
│  │  • REST API Client (email, calendar, contacts)        │  │
│  │  • Real-time Updates (SSE/WebSocket)                  │  │
│  │  • Retry Logic & Error Handling                       │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           ↕
                  Internet Connection
                           ↕
┌─────────────────────────────────────────────────────────────┐
│              Symfony Platform (eiq-manager)                  │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           REST API Layer (New)                        │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • Authentication API (JWT tokens)                    │  │
│  │  • Email API (accounts, messages, folders)            │  │
│  │  • Calendar API (events, tasks, invitations)          │  │
│  │  • Contact API (address book)                         │  │
│  │  • Sync Status API (progress, logs)                   │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │     Existing Services (Reusable)                      │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • DbalEmailSyncService (sync engine)                 │  │
│  │  • SmarterMailApiService (provider integration)       │  │
│  │  • EmailMessageRepository (queries)                   │  │
│  │  • CalendarService (event management)                 │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │       PostgreSQL Database (Multi-tenant)              │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • public schema (users, tenants)                     │  │
│  │  • tenant_X_email_manager (email data)                │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │          SmarterMail Server                           │  │
│  ├──────────────────────────────────────────────────────┤  │
│  │  • Email accounts                                     │  │
│  │  • Calendar & contacts                                │  │
│  │  • Folders & messages                                 │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔄 Data Flow & Sync Strategy

### Offline-First Architecture

**Core Principle**: All user actions happen locally first, then sync to server in background.

```
User Action → Local SQLite (instant) → Sync Queue → Platform API (background)
                      ↓                                        ↓
                 UI Updates                              Update on success
                 Immediately                             Retry on failure
```

### Sync Scenarios

#### 1. **Initial Sync** (First Launch or Account Add)
```
1. User logs in → Get JWT token
2. Fetch user's email accounts from API
3. For each account:
   a. Fetch folder structure
   b. Fetch last 30 days of messages (headers only)
   c. Fetch calendar events (last 90 days + future)
4. Store everything in local SQLite
5. Mark initial sync complete
6. Start background sync service
```

#### 2. **Background Sync** (Every 5 minutes or on trigger)
```
1. Check for new messages (incremental since last sync)
2. Download new message headers
3. Check for updated flags (read, starred, deleted)
4. Sync calendar events (new/updated/deleted)
5. Process outbox (send queued emails)
6. Update sync status
```

#### 3. **Real-time Updates** (When online)
```
1. Subscribe to Mercure topics:
   - /email/{userId}/messages/new
   - /email/{userId}/sync/progress
   - /calendar/{userId}/events/updated
2. Receive SSE notifications
3. Fetch specific updates from API
4. Update local database
5. Show desktop notification
```

#### 4. **Offline Operations**
```
User Action (offline) → Local DB → Sync Queue
                                        ↓
                            When connection restored:
                                        ↓
                            Process queue in order:
                            - Send queued emails
                            - Update flags
                            - Move/delete messages
                            - Create/update calendar events
```

### Conflict Resolution

**Strategy**: Server wins (last-write-wins)

```
Conflict Detected (local vs server) → Compare timestamps
                                           ↓
                        Server timestamp > Local timestamp?
                                           ↓
                                    Yes → Accept server version
                                    No  → Keep local version
                                           ↓
                                    Log conflict for user review
```

---

## 🗄️ Local Database Schema (SQLite)

### Tables

```sql
-- User accounts and authentication
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY,
    platform_user_id INTEGER NOT NULL,
    email TEXT NOT NULL,
    full_name TEXT,
    jwt_token TEXT,
    refresh_token TEXT,
    token_expires_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Email accounts (from EmailAccount entity)
CREATE TABLE email_accounts (
    id INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    server_account_id INTEGER, -- Platform database ID
    email_address TEXT NOT NULL,
    provider TEXT DEFAULT 'smartermail',
    server_host TEXT,
    server_port INTEGER DEFAULT 443,
    use_ssl BOOLEAN DEFAULT 1,
    auto_sync BOOLEAN DEFAULT 1,
    sync_interval_minutes INTEGER DEFAULT 5,
    last_sync_at DATETIME,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Folders (from EmailFolder entity)
CREATE TABLE email_folders (
    id INTEGER PRIMARY KEY,
    email_account_id INTEGER REFERENCES email_accounts(id),
    server_folder_id INTEGER, -- Platform database ID
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    type TEXT, -- inbox, sent, drafts, trash, junk, archive, custom
    parent_id INTEGER REFERENCES email_folders(id),
    last_uid INTEGER DEFAULT 0,
    total_messages INTEGER DEFAULT 0,
    unread_messages INTEGER DEFAULT 0,
    last_sync_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(email_account_id, path)
);

-- Messages (from EmailMessage entity)
CREATE TABLE email_messages (
    id INTEGER PRIMARY KEY,
    email_account_id INTEGER REFERENCES email_accounts(id),
    folder_id INTEGER REFERENCES email_folders(id),
    server_message_id INTEGER, -- Platform database ID
    uid TEXT NOT NULL,
    message_id TEXT, -- RFC 822 Message-ID
    subject TEXT,
    body_plain TEXT,
    body_html TEXT,
    from_address TEXT,
    from_name TEXT,
    to_addresses TEXT, -- JSON array
    cc_addresses TEXT, -- JSON array
    bcc_addresses TEXT, -- JSON array
    received_date DATETIME,
    sent_date DATETIME,
    is_read BOOLEAN DEFAULT 0,
    is_flagged BOOLEAN DEFAULT 0,
    is_deleted BOOLEAN DEFAULT 0,
    is_draft BOOLEAN DEFAULT 0,
    has_attachments BOOLEAN DEFAULT 0,
    size INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(email_account_id, folder_id, uid)
);

-- Attachments (from EmailAttachment entity)
CREATE TABLE email_attachments (
    id INTEGER PRIMARY KEY,
    message_id INTEGER REFERENCES email_messages(id),
    server_attachment_id INTEGER, -- Platform database ID
    filename TEXT NOT NULL,
    mime_type TEXT,
    size INTEGER,
    local_path TEXT, -- Path to cached file
    content_hash TEXT, -- SHA256 for deduplication
    is_downloaded BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Calendar accounts (from CalendarAccount entity)
CREATE TABLE calendar_accounts (
    id INTEGER PRIMARY KEY,
    email_account_id INTEGER REFERENCES email_accounts(id),
    server_calendar_id INTEGER,
    name TEXT NOT NULL,
    description TEXT,
    type TEXT DEFAULT 'personal', -- personal, shared, public, room
    color TEXT DEFAULT '#3B82F6',
    is_visible BOOLEAN DEFAULT 1,
    is_default BOOLEAN DEFAULT 0,
    last_sync_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Calendar events (from CalendarEvent entity)
CREATE TABLE calendar_events (
    id INTEGER PRIMARY KEY,
    calendar_account_id INTEGER REFERENCES calendar_accounts(id),
    server_event_id INTEGER,
    title TEXT NOT NULL,
    description TEXT,
    location TEXT,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    all_day BOOLEAN DEFAULT 0,
    is_recurring BOOLEAN DEFAULT 0,
    recurrence_rule TEXT, -- RRULE format
    attendees TEXT, -- JSON array
    organizer_email TEXT,
    status TEXT DEFAULT 'confirmed', -- confirmed, tentative, cancelled
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Sync queue for offline operations
CREATE TABLE sync_queue (
    id INTEGER PRIMARY KEY,
    operation_type TEXT NOT NULL, -- send_email, update_flag, move_message, delete_message, create_event, etc.
    entity_type TEXT NOT NULL, -- email_message, calendar_event, etc.
    entity_id INTEGER,
    payload TEXT, -- JSON with operation details
    retry_count INTEGER DEFAULT 0,
    last_error TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    processed_at DATETIME
);

-- Sync logs
CREATE TABLE sync_logs (
    id INTEGER PRIMARY KEY,
    email_account_id INTEGER REFERENCES email_accounts(id),
    status TEXT, -- running, completed, failed
    started_at DATETIME,
    completed_at DATETIME,
    folders_synced INTEGER DEFAULT 0,
    messages_synced INTEGER DEFAULT 0,
    errors_count INTEGER DEFAULT 0,
    error_message TEXT
);

-- Indexes for performance
CREATE INDEX idx_messages_folder_date ON email_messages(folder_id, received_date DESC);
CREATE INDEX idx_messages_read ON email_messages(folder_id, is_read);
CREATE INDEX idx_messages_uid ON email_messages(email_account_id, uid);
CREATE INDEX idx_attachments_message ON email_attachments(message_id);
CREATE INDEX idx_calendar_events_date ON calendar_events(calendar_account_id, start_time);
CREATE INDEX idx_sync_queue_status ON sync_queue(operation_type, processed_at);
```

---

## 🔌 Platform API Specification

### Authentication

#### 1. Login
```http
POST /api/v1/auth/token
Content-Type: application/json

{
    "email": "user@example.com",
    "password": "password"
}
```

**Response**:
```json
{
    "token": "eyJhbGciOiJSUzI1NiIs...",
    "refresh_token": "def50200...",
    "expires_in": 3600,
    "user": {
        "id": 123,
        "email": "user@example.com",
        "full_name": "John Doe",
        "requires_2fa": false
    }
}
```

#### 2. Two-Factor Authentication
```http
POST /api/v1/auth/2fa
Content-Type: application/json
Authorization: Bearer {partial_token}

{
    "code": "123456"
}
```

**Response**:
```json
{
    "token": "eyJhbGciOiJSUzI1NiIs...",
    "refresh_token": "def50200...",
    "expires_in": 3600
}
```

#### 3. Refresh Token
```http
POST /api/v1/auth/refresh
Content-Type: application/json

{
    "refresh_token": "def50200..."
}
```

**Response**:
```json
{
    "token": "eyJhbGciOiJSUzI1NiIs...",
    "expires_in": 3600
}
```

---

### Email Accounts API

#### 1. List Email Accounts
```http
GET /api/v1/email/accounts
Authorization: Bearer {token}
```

**Response**:
```json
{
    "accounts": [
        {
            "id": 1,
            "email_address": "john@example.com",
            "provider": "smartermail",
            "server_host": "mail.example.com",
            "server_port": 443,
            "use_ssl": true,
            "auto_sync": true,
            "sync_interval_minutes": 15,
            "last_sync_at": "2025-10-30T10:30:00Z",
            "is_active": true,
            "folder_count": 8,
            "total_messages": 1523
        }
    ]
}
```

#### 2. Add Email Account
```http
POST /api/v1/email/accounts
Authorization: Bearer {token}
Content-Type: application/json

{
    "email_address": "john@example.com",
    "password": "password",
    "provider": "smartermail",
    "server_host": "mail.example.com",
    "server_port": 443,
    "use_ssl": true,
    "auto_sync": true,
    "sync_interval_minutes": 15
}
```

**Response**:
```json
{
    "id": 1,
    "email_address": "john@example.com",
    "provider": "smartermail",
    "status": "created",
    "sync_initiated": true
}
```

#### 3. Trigger Sync
```http
POST /api/v1/email/accounts/{id}/sync
Authorization: Bearer {token}
```

**Response**:
```json
{
    "status": "sync_initiated",
    "account_id": 1,
    "sync_log_id": 456
}
```

#### 4. Get Sync Status
```http
GET /api/v1/email/accounts/{id}/sync-status
Authorization: Bearer {token}
```

**Response**:
```json
{
    "account_id": 1,
    "status": "running",
    "started_at": "2025-10-30T10:35:00Z",
    "progress": {
        "folders_synced": 3,
        "total_folders": 8,
        "messages_synced": 245,
        "current_folder": "INBOX"
    }
}
```

---

### Folders API

#### 1. List Folders
```http
GET /api/v1/email/accounts/{accountId}/folders
Authorization: Bearer {token}
```

**Response**:
```json
{
    "folders": [
        {
            "id": 1,
            "name": "Inbox",
            "path": "INBOX",
            "type": "inbox",
            "parent_id": null,
            "total_messages": 523,
            "unread_messages": 12,
            "last_sync_at": "2025-10-30T10:35:00Z"
        },
        {
            "id": 2,
            "name": "Sent Items",
            "path": "Sent Items",
            "type": "sent",
            "parent_id": null,
            "total_messages": 892,
            "unread_messages": 0,
            "last_sync_at": "2025-10-30T10:35:00Z"
        }
    ]
}
```

---

### Messages API

#### 1. List Messages (Paginated)
```http
GET /api/v1/email/folders/{folderId}/messages?page=1&limit=50&since_uid=1000
Authorization: Bearer {token}
```

**Response**:
```json
{
    "messages": [
        {
            "id": 1234,
            "uid": "1523",
            "message_id": "<abc123@example.com>",
            "subject": "Meeting tomorrow",
            "from_address": "sender@example.com",
            "from_name": "John Sender",
            "to_addresses": ["john@example.com"],
            "received_date": "2025-10-30T09:15:00Z",
            "is_read": false,
            "is_flagged": false,
            "has_attachments": true,
            "size": 45231,
            "snippet": "Hi John, just wanted to confirm our meeting..."
        }
    ],
    "pagination": {
        "page": 1,
        "limit": 50,
        "total": 523,
        "has_more": true
    }
}
```

#### 2. Get Full Message
```http
GET /api/v1/email/messages/{id}
Authorization: Bearer {token}
```

**Response**:
```json
{
    "id": 1234,
    "uid": "1523",
    "message_id": "<abc123@example.com>",
    "subject": "Meeting tomorrow",
    "body_plain": "Hi John, just wanted to confirm...",
    "body_html": "<html><body>Hi John, just wanted to confirm...</body></html>",
    "from_address": "sender@example.com",
    "from_name": "John Sender",
    "to_addresses": ["john@example.com"],
    "cc_addresses": [],
    "bcc_addresses": [],
    "received_date": "2025-10-30T09:15:00Z",
    "sent_date": "2025-10-30T09:14:00Z",
    "is_read": false,
    "is_flagged": false,
    "is_deleted": false,
    "is_draft": false,
    "has_attachments": true,
    "size": 45231,
    "attachments": [
        {
            "id": 567,
            "filename": "document.pdf",
            "mime_type": "application/pdf",
            "size": 123456,
            "download_url": "/api/v1/email/attachments/567/download"
        }
    ]
}
```

#### 3. Send Email
```http
POST /api/v1/email/messages
Authorization: Bearer {token}
Content-Type: application/json

{
    "account_id": 1,
    "to": ["recipient@example.com"],
    "cc": [],
    "bcc": [],
    "subject": "Test email",
    "body_html": "<p>This is a test</p>",
    "body_plain": "This is a test",
    "attachments": [
        {
            "filename": "document.pdf",
            "content": "base64_encoded_content",
            "mime_type": "application/pdf"
        }
    ]
}
```

**Response**:
```json
{
    "status": "sent",
    "message_id": "<generated@example.com>",
    "sent_at": "2025-10-30T10:45:00Z"
}
```

#### 4. Update Message Flags
```http
PUT /api/v1/email/messages/{id}/flags
Authorization: Bearer {token}
Content-Type: application/json

{
    "is_read": true,
    "is_flagged": false
}
```

**Response**:
```json
{
    "id": 1234,
    "is_read": true,
    "is_flagged": false,
    "updated_at": "2025-10-30T10:46:00Z"
}
```

#### 5. Move Message
```http
POST /api/v1/email/messages/{id}/move
Authorization: Bearer {token}
Content-Type: application/json

{
    "target_folder_id": 5
}
```

**Response**:
```json
{
    "id": 1234,
    "folder_id": 5,
    "moved_at": "2025-10-30T10:47:00Z"
}
```

#### 6. Delete Message
```http
DELETE /api/v1/email/messages/{id}
Authorization: Bearer {token}
```

**Response**:
```json
{
    "status": "deleted",
    "deleted_at": "2025-10-30T10:48:00Z"
}
```

---

### Calendar API

#### 1. List Events
```http
GET /api/v1/calendar/events?start=2025-10-01&end=2025-10-31
Authorization: Bearer {token}
```

**Response**:
```json
{
    "events": [
        {
            "id": 789,
            "calendar_account_id": 1,
            "title": "Team Meeting",
            "description": "Weekly team sync",
            "location": "Conference Room A",
            "start_time": "2025-10-30T14:00:00Z",
            "end_time": "2025-10-30T15:00:00Z",
            "all_day": false,
            "is_recurring": false,
            "attendees": [
                {
                    "email": "john@example.com",
                    "name": "John Doe",
                    "status": "accepted"
                }
            ],
            "organizer_email": "manager@example.com",
            "status": "confirmed"
        }
    ]
}
```

#### 2. Create Event
```http
POST /api/v1/calendar/events
Authorization: Bearer {token}
Content-Type: application/json

{
    "calendar_account_id": 1,
    "title": "Team Meeting",
    "description": "Weekly team sync",
    "location": "Conference Room A",
    "start_time": "2025-10-30T14:00:00Z",
    "end_time": "2025-10-30T15:00:00Z",
    "all_day": false,
    "attendees": ["john@example.com", "jane@example.com"]
}
```

**Response**:
```json
{
    "id": 789,
    "status": "created",
    "created_at": "2025-10-30T10:50:00Z"
}
```

---

### Real-time Updates (Mercure Hub)

#### Subscribe to Topics
```javascript
const eventSource = new EventSource(
    'https://platform.example.com/.well-known/mercure?topic=/email/123/messages/new',
    { headers: { 'Authorization': 'Bearer ' + jwtToken } }
);

eventSource.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('New message:', data);
};
```

**Topics**:
- `/email/{userId}/messages/new` - New message notifications
- `/email/{userId}/messages/updated` - Message flag updates
- `/email/{userId}/sync/progress` - Sync progress updates
- `/calendar/{userId}/events/new` - New calendar events
- `/calendar/{userId}/events/updated` - Event updates

---

## 📱 Desktop App UI/UX Design

### Main Window Layout

```
┌──────────────────────────────────────────────────────────────────┐
│  [☰] Mail    Calendar    Tasks    [🔍] Search    [⚙️] Settings   │  ← Navbar
├──────────────────────────────────────────────────────────────────┤
│                                                                    │
│  ┌─────────┬──────────────┬──────────────────────────────────┐  │
│  │Folders  │Message List  │Reading Pane                       │  │
│  │         │              │                                   │  │
│  │📥 Inbox │From: John    │Subject: Meeting tomorrow          │  │
│  │  (12)   │Subject: Meet │                                   │  │
│  │         │Date: 10:30   │Hi John,                          │  │
│  │📤 Sent  │              │                                   │  │
│  │         │From: Jane    │Just wanted to confirm our        │  │
│  │📝 Drafts│Subject: Proj │meeting tomorrow at 2pm...        │  │
│  │         │Date: 09:15   │                                   │  │
│  │🗑️ Trash │              │[Reply] [Reply All] [Forward]     │  │
│  │         │From: Bob     │                                   │  │
│  │📁 Custom│Subject: Q4   │                                   │  │
│  │  Folder │Date: Yesterday│                                  │  │
│  │         │              │                                   │  │
│  │[+ New]  │[Load More]   │[Attachments: document.pdf]       │  │
│  └─────────┴──────────────┴──────────────────────────────────┘  │
│                                                                    │
│  [✉️ New Email]                   Last sync: 2 minutes ago [🔄]  │  ← Status bar
└──────────────────────────────────────────────────────────────────┘
```

### Email Composer

```
┌──────────────────────────────────────────────────────────────────┐
│  New Message                                         [_] [□] [×]  │
├──────────────────────────────────────────────────────────────────┤
│  To:      [recipient@example.com                           ] [+] │
│  Cc:      [                                                ] [+] │
│  Subject: [                                                     ] │
│  ─────────────────────────────────────────────────────────────── │
│  [B] [I] [U] [≡] [🔗] [📎]                                       │
│  ─────────────────────────────────────────────────────────────── │
│  │                                                              │  │
│  │  [Compose your email here...]                             │  │
│  │                                                              │  │
│  │                                                              │  │
│  │                                                              │  │
│  ─────────────────────────────────────────────────────────────── │
│  Attachments: [document.pdf (123 KB) [×]]                        │
│                                                                    │
│  [Send]  [Save Draft]  [Discard]                                 │
└──────────────────────────────────────────────────────────────────┘
```

### Calendar View

```
┌──────────────────────────────────────────────────────────────────┐
│  [◀] October 2025 [▶]         [Day] [Week] [Month] [+ New Event] │
├──────────────────────────────────────────────────────────────────┤
│  Sun    Mon    Tue    Wed    Thu    Fri    Sat                   │
│  ─────────────────────────────────────────────────────────────── │
│         1      2      3      4      5      6                      │
│                      [10am                                        │
│                       Team                                        │
│                       Mtg]                                        │
│  ─────────────────────────────────────────────────────────────── │
│  7      8      9      10     11     12     13                     │
│                                                                    │
│  ─────────────────────────────────────────────────────────────── │
│  14     15     16     17     18     19     20                     │
│         [2pm                                                      │
│          Client                                                   │
│          Call]                                                    │
│  ─────────────────────────────────────────────────────────────── │
│  21     22     23     24     25     26     27                     │
│                                                                    │
│  ─────────────────────────────────────────────────────────────── │
│  28     29     30     31                                          │
│              [Today]                                              │
└──────────────────────────────────────────────────────────────────┘
```

### System Tray Integration

```
┌─────────────────────────┐
│ 📧 EvolveMailPro        │
├─────────────────────────┤
│ 12 unread messages      │
│                         │
│ ✉ john@example.com (8)  │
│ ✉ jane@example.com (4)  │
│                         │
│ ──────────────────────  │
│ 🔄 Sync Now             │
│ ⚙️  Settings            │
│ 🚪 Quit                 │
└─────────────────────────┘
```

---

## 📋 Implementation Phases

### **Phase 1: Platform REST API** (Backend - Symfony)
**Duration**: 2-3 weeks
**Effort**: Medium

**Tasks**:
1. Install and configure LexikJWTAuthenticationBundle
2. Create API controllers:
   - `AuthApiController` - Login, 2FA, refresh tokens
   - `EmailAccountApiController` - CRUD for email accounts
   - `EmailMessageApiController` - Message operations
   - `EmailFolderApiController` - Folder listing
   - `CalendarApiController` - Event management
3. Add API routes with `/api/v1/` prefix
4. Implement JWT authentication for API routes
5. Add CORS configuration for desktop app
6. Create API documentation (OpenAPI/Swagger)
7. Add rate limiting and throttling
8. Write integration tests for all endpoints

**Deliverables**:
- ✅ JWT authentication working
- ✅ All API endpoints functional
- ✅ API documentation published
- ✅ Postman/Insomnia collection for testing

---

### **Phase 2: Desktop App MVP** (Desktop Client)
**Duration**: 3-4 weeks
**Effort**: High

**Tasks**:
1. **Project Setup**:
   - Initialize Tauri project
   - Set up Vue 3 with TypeScript
   - Configure Tailwind CSS + DaisyUI
   - Set up Prisma with SQLite

2. **Authentication**:
   - Login screen
   - JWT token storage (keyring)
   - Token refresh logic
   - 2FA support

3. **Email Account Management**:
   - Add account flow
   - Account list/edit/delete
   - Connection testing

4. **Email Client (Basic)**:
   - Three-pane layout (folders | messages | reading pane)
   - Folder list with unread counts
   - Message list with pagination
   - Message reading pane
   - Mark as read/unread
   - Flag messages
   - Delete messages

5. **Composer (Basic)**:
   - New email form
   - To/Cc/Subject fields
   - Plain text editor
   - Send email

6. **Local Database**:
   - SQLite schema implementation
   - Basic sync logic (pull messages)
   - Message caching

7. **Background Service**:
   - Auto-sync every 5 minutes
   - Manual sync button
   - Sync status indicator

**Deliverables**:
- ✅ Working login flow
- ✅ Can read emails offline
- ✅ Can send basic emails
- ✅ Background sync functional
- ✅ Cross-platform builds (Windows, Mac, Linux)

---

### **Phase 3: Full Email Features**
**Duration**: 2-3 weeks
**Effort**: Medium-High

**Tasks**:
1. **Attachments**:
   - Download and cache attachments
   - View attachments in message
   - Attach files when composing
   - Inline images support

2. **Rich Text Editor**:
   - Integrate ProseMirror
   - Formatting toolbar (bold, italic, lists, links)
   - HTML email composition
   - Signature support

3. **Advanced Composer**:
   - Reply/Reply All/Forward
   - Quoted message formatting
   - Draft auto-save
   - Email templates

4. **Search**:
   - Full-text search in local database
   - Search by sender, subject, date
   - Search filters
   - Search while offline

5. **Multiple Accounts**:
   - Account switcher
   - Unified inbox (all accounts)
   - Per-account settings

6. **Folder Management**:
   - Create/rename/delete folders
   - Move messages between folders
   - Folder hierarchy

**Deliverables**:
- ✅ Full email composition with attachments
- ✅ Search working offline
- ✅ Multiple account support
- ✅ Folder management

---

### **Phase 4: Calendar Integration**
**Duration**: 2-3 weeks
**Effort**: Medium-High

**Tasks**:
1. **Calendar Views**:
   - Month view
   - Week view
   - Day view
   - Agenda view

2. **Event Management**:
   - Create/edit/delete events
   - Recurring events
   - All-day events
   - Event reminders

3. **Meeting Invitations**:
   - Send meeting invitations
   - Accept/decline invitations
   - Track attendee responses

4. **Calendar Sync**:
   - Sync with SmarterMail calendar
   - Background sync
   - Offline event creation

5. **Notifications**:
   - Desktop notifications for new emails
   - Calendar event reminders
   - Meeting invitations

**Deliverables**:
- ✅ Full calendar functionality
- ✅ Meeting invitations working
- ✅ Desktop notifications
- ✅ Calendar sync with server

---

### **Phase 5: Polish & Native Integration**
**Duration**: 2-3 weeks
**Effort**: Medium

**Tasks**:
1. **OS-Specific Features**:
   - **Windows**:
     - Action Center notifications
     - Jump Lists (recent emails/events)
     - Start Menu integration
   - **macOS**:
     - Touch Bar support
     - Menu Bar extras
     - Notification Center
   - **Linux**:
     - System tray
     - Desktop files (.desktop)
     - DBus notifications

2. **Performance Optimization**:
   - Lazy loading for large mailboxes
   - Virtual scrolling for message lists
   - Database indexing optimization
   - Memory usage optimization

3. **Security Enhancements**:
   - Encrypted local database
   - Secure token storage (OS keyring)
   - Auto-lock on idle
   - Master password option

4. **User Experience**:
   - Keyboard shortcuts
   - Drag & drop (files, emails)
   - Theme support (dark/light mode)
   - Settings panel
   - Import/export settings
   - Backup/restore

5. **Auto-Update**:
   - Auto-update mechanism
   - Release channels (stable, beta)
   - Update notifications

**Deliverables**:
- ✅ Native OS integrations working
- ✅ Performance optimized
- ✅ Security hardened
- ✅ Auto-update system
- ✅ Production-ready builds

---

## 🔐 Security Considerations

### Desktop App Security

1. **Token Storage**:
   - Store JWT tokens in OS-native secure storage (Keyring)
   - Never store tokens in plain text
   - Auto-rotate refresh tokens

2. **Local Database Encryption**:
   - Use SQLCipher for encrypted SQLite database
   - Master password or OS-derived encryption key
   - Encrypt sensitive fields (passwords)

3. **Transport Security**:
   - Enforce HTTPS for all API calls
   - Certificate pinning for API endpoints
   - Validate SSL certificates

4. **Code Security**:
   - Code signing for releases
   - Update signature verification
   - Sandboxed renderer process (Tauri default)

5. **Data Minimization**:
   - Cache only necessary data
   - Auto-delete old cached messages (e.g., >90 days)
   - Secure deletion when removing accounts

### Platform API Security

1. **Authentication**:
   - JWT tokens with short expiration (1 hour)
   - Refresh tokens with longer expiration (30 days)
   - Revokable tokens

2. **Authorization**:
   - Validate user owns requested resources
   - Role-based access control
   - Tenant isolation

3. **Rate Limiting**:
   - Per-user rate limits
   - Progressive backoff for failed requests
   - DDoS protection

4. **Input Validation**:
   - Validate all API inputs
   - Sanitize HTML content
   - Prevent XSS and SQL injection

5. **Audit Logging**:
   - Log all API access
   - Track failed authentication attempts
   - Monitor suspicious activity

---

## 🧪 Testing Strategy

### Unit Tests
- **Backend (Symfony)**:
  - Service layer tests
  - API controller tests
  - Repository tests
- **Desktop (Tauri)**:
  - Rust backend logic
  - Sync service tests
  - Database operations

### Integration Tests
- API endpoint tests (Symfony)
- API client tests (desktop app)
- Sync flow tests
- Authentication flow tests

### E2E Tests
- **Playwright** or **Tauri WebDriver**:
  - Login flow
  - Send/receive email
  - Calendar event creation
  - Offline mode
  - Sync after reconnect

### Manual Testing
- Cross-platform testing (Windows, Mac, Linux)
- Performance testing (large mailboxes)
- Network failure scenarios
- Long-running sync tests

---

## 📊 Development Effort Summary

| Phase | Duration | Effort | Complexity |
|-------|----------|--------|------------|
| Phase 1: Platform API | 2-3 weeks | Medium | Medium |
| Phase 2: Desktop MVP | 3-4 weeks | High | High |
| Phase 3: Full Email | 2-3 weeks | Medium-High | Medium |
| Phase 4: Calendar | 2-3 weeks | Medium-High | Medium |
| Phase 5: Polish | 2-3 weeks | Medium | Medium |
| **Total** | **12-16 weeks** | **Very High** | **High** |

**Team Size**: 1-2 developers (full-time)
**Skills Required**:
- Symfony/PHP (backend API)
- Rust (Tauri backend)
- Vue 3/TypeScript (frontend)
- SQLite/Prisma (database)
- API design
- Desktop app development

---

## 🚀 Getting Started

### Prerequisites

**Backend (Symfony)**:
- PHP 8.2+
- Composer
- PostgreSQL 17
- Redis (for caching)

**Desktop App**:
- Node.js 18+
- Rust 1.70+
- Tauri CLI
- Platform-specific build tools (see Tauri docs)

### Quick Start

#### 1. **Platform API Setup**
```bash
cd /home/john/sources/eiq-manager

# Install JWT bundle
composer require lexik/jwt-authentication-bundle

# Generate JWT keys
php bin/console lexik:jwt:generate-keypair

# Create API controllers
php bin/console make:controller Api/AuthApiController
php bin/console make:controller Api/EmailAccountApiController
php bin/console make:controller Api/EmailMessageApiController

# Clear cache
php bin/console cache:clear
```

#### 2. **Desktop App Setup**
```bash
# Create new Tauri project
npm create tauri-app@latest evolve-mail-pro

# Choose:
# - Package manager: npm
# - UI template: Vue + TypeScript
# - UI flavor: vue-ts

cd evolve-mail-pro

# Install dependencies
npm install
npm install tailwindcss daisyui
npm install @prisma/client
npm install axios pinia

# Install Tauri plugins
cargo add tauri-plugin-store
cargo add tauri-plugin-notification
cargo add tauri-plugin-sql --features sqlite

# Run in development
npm run tauri dev
```

---

## 📚 Additional Resources

### Documentation to Create
- [ ] API Reference (OpenAPI spec)
- [ ] Desktop App User Guide
- [ ] Developer Setup Guide
- [ ] Security Best Practices
- [ ] Troubleshooting Guide

### External Documentation
- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Vue 3 Documentation](https://vuejs.org/)
- [Prisma Documentation](https://www.prisma.io/docs)
- [SmarterMail API Docs](https://mail.smartertools.com/Documentation/api/)

---

## 🤝 Stakeholder Communication

### Weekly Updates
- Progress on current phase
- Blockers and challenges
- Demo of new features
- Next week's goals

### Milestone Demos
- End of each phase demo
- User feedback collection
- Adjustment planning

### Launch Planning
- Beta testing program
- Marketing materials
- Release notes
- Support documentation

---

## ✅ Success Criteria

### MVP Success (Phase 2)
- [ ] Users can log in with platform credentials
- [ ] Users can read emails offline
- [ ] Users can send basic emails
- [ ] Background sync works reliably
- [ ] Runs on Windows, Mac, and Linux

### Full Launch Success (Phase 5)
- [ ] Complete Outlook-style feature parity
- [ ] Offline mode works seamlessly
- [ ] Calendar integration functional
- [ ] Native OS notifications working
- [ ] Performance metrics met:
  - App launch < 3 seconds
  - Sync 1000 messages < 30 seconds
  - UI remains responsive during sync
- [ ] Security audit passed
- [ ] Positive user feedback (beta)

---

## 📞 Next Steps

1. **Review & Approve** this architecture document
2. **Kickoff Meeting** with development team
3. **Set Up Development Environment**
4. **Start Phase 1**: Build Platform REST API
5. **Weekly Check-ins** to track progress

---

**Document Version**: 1.0
**Last Updated**: 2025-10-30
**Author**: Claude (AI Assistant)
**Status**: Planning Phase - Awaiting Approval
