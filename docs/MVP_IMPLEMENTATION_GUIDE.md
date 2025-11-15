# EIQ Desktop - MVP Implementation Guide

**Modules**: Email, Calendar, CRM
**Timeline**: 6 weeks (2 developers)
**Status**: Production-Ready Guide
**Date**: 2025-11-14

---

## 📋 Table of Contents

1. [Week-by-Week Roadmap](#week-by-week-roadmap)
2. [Email Module Implementation](#email-module-implementation)
3. [Calendar Module Implementation](#calendar-module-implementation)
4. [CRM Module Implementation](#crm-module-implementation)
5. [Integration Points](#integration-points)
6. [Testing Strategy](#testing-strategy)

---

## 🗓️ Week-by-Week Roadmap

### Week 1: Foundation & Email (Backend)
**Backend Developer**:
- [ ] Set up Symfony API routes (`/api/v1/`)
- [ ] Implement JWT authentication bundle
- [ ] Create Email API controllers
  - `EmailAccountApiController`
  - `EmailFolderApiController`
  - `EmailMessageApiController`
- [ ] Add CORS configuration
- [ ] Write API tests

**Frontend Developer**:
- [ ] Initialize Tauri project
- [ ] Set up Vue 3 + TypeScript
- [ ] Configure Tailwind CSS + DaisyUI
- [ ] Implement plugin system core
- [ ] Create SQLite database schema

**Deliverable**: API endpoints working, Desktop shell ready

---

### Week 2: Email Module (Frontend)
**Backend Developer**:
- [ ] Implement Calendar API endpoints
- [ ] Add real-time updates (Mercure)
- [ ] Optimize email sync logic
- [ ] Add rate limiting

**Frontend Developer**:
- [ ] Build Email module plugin
- [ ] Create 3-pane email layout
- [ ] Implement folder list component
- [ ] Implement message list component
- [ ] Implement reading pane
- [ ] Add basic composer

**Deliverable**: Email module functional (read-only)

---

### Week 3: Email Completion + Calendar Start
**Backend Developer**:
- [ ] Implement attachment upload/download
- [ ] Add search endpoints
- [ ] Calendar event CRUD APIs
- [ ] Meeting invitation APIs

**Frontend Developer**:
- [ ] Complete email composer
- [ ] Add attachment support
- [ ] Implement offline sync for email
- [ ] Start Calendar module
- [ ] Create calendar views (month/week/day)

**Deliverable**: Email module complete, Calendar started

---

### Week 4: Calendar + CRM Backend
**Backend Developer**:
- [ ] Implement CRM Contact API
- [ ] Implement CRM Company API
- [ ] Add search/filter endpoints
- [ ] Bulk operations API

**Frontend Developer**:
- [ ] Complete Calendar module
- [ ] Event CRUD operations
- [ ] Meeting invitations UI
- [ ] Calendar-Email integration
- [ ] Start CRM module structure

**Deliverable**: Calendar complete, CRM API ready

---

### Week 5: CRM Implementation
**Backend Developer**:
- [ ] CRM Deal API endpoints
- [ ] CRM Activity API
- [ ] Analytics endpoints
- [ ] Sync optimization

**Frontend Developer**:
- [ ] CRM contact list UI
- [ ] Contact detail view
- [ ] Contact create/edit forms
- [ ] Search and filtering
- [ ] Company management (basic)

**Deliverable**: CRM module functional

---

### Week 6: Polish & Integration
**Both Developers**:
- [ ] Cross-module integration
  - Email → CRM (create contact)
  - Calendar → Email (meeting invites)
  - CRM → Email (contact emails)
- [ ] Background sync testing
- [ ] Offline mode testing
- [ ] UI/UX refinements
- [ ] Performance optimization
- [ ] Bug fixes
- [ ] Documentation
- [ ] Prepare builds (Windows/Mac/Linux)

**Deliverable**: MVP Ready for Beta Release

---

## 📧 Email Module Implementation

### File Structure

```
src/modules/email/
├── index.ts                      # Plugin export
├── module.ts                     # Module class
├── routes.ts                     # Route definitions
│
├── components/
│   ├── EmailView.vue             # Main 3-pane layout
│   ├── FolderList.vue            # Left: Folder tree
│   ├── MessageList.vue           # Middle: Message list
│   ├── MessageReader.vue         # Right: Reading pane
│   ├── Composer.vue              # Email composer modal
│   ├── AttachmentList.vue        # Attachment display
│   └── SearchBar.vue             # Email search
│
├── stores/
│   ├── email.ts                  # Email state (Pinia)
│   ├── folders.ts                # Folder state
│   └── composer.ts               # Composer state
│
├── services/
│   ├── EmailService.ts           # Business logic
│   ├── EmailApiClient.ts         # API calls
│   └── EmailSync.ts              # Sync logic
│
├── database/
│   └── schema.ts                 # SQLite schema
│
└── types/
    └── email.types.ts            # TypeScript types
```

### Key Components

#### EmailView.vue (3-Pane Layout)

```vue
<template>
  <div class="flex h-full">
    <!-- Left: Folders -->
    <div class="w-64 border-r">
      <FolderList
        :folders="folders"
        :selected-folder="selectedFolder"
        @select="onFolderSelect"
      />
    </div>

    <!-- Middle: Message List -->
    <div class="w-96 border-r">
      <SearchBar @search="onSearch" />
      <MessageList
        :messages="messages"
        :selected-message="selectedMessage"
        :loading="loading"
        @select="onMessageSelect"
        @load-more="loadMore"
      />
    </div>

    <!-- Right: Reading Pane -->
    <div class="flex-1">
      <MessageReader
        v-if="selectedMessage"
        :message="selectedMessage"
        @reply="onReply"
        @forward="onForward"
        @delete="onDelete"
      />
      <div v-else class="flex items-center justify-center h-full text-gray-400">
        Select a message to read
      </div>
    </div>

    <!-- Composer Modal -->
    <Composer
      v-if="showComposer"
      :draft="composerDraft"
      @send="onSend"
      @close="showComposer = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useEmailStore } from '../stores/email';
import FolderList from './FolderList.vue';
import MessageList from './MessageList.vue';
import MessageReader from './MessageReader.vue';
import Composer from './Composer.vue';
import SearchBar from './SearchBar.vue';

const emailStore = useEmailStore();

const selectedFolder = ref(null);
const selectedMessage = ref(null);
const showComposer = ref(false);
const composerDraft = ref(null);

const folders = computed(() => emailStore.folders);
const messages = computed(() => emailStore.messages);
const loading = computed(() => emailStore.loading);

onMounted(async () => {
  await emailStore.loadFolders();
});

function onFolderSelect(folder: any) {
  selectedFolder.value = folder;
  selectedMessage.value = null;
  emailStore.loadMessages(folder.id);
}

function onMessageSelect(message: any) {
  selectedMessage.value = message;
  if (!message.is_read) {
    emailStore.markAsRead(message.id);
  }
}

function onReply() {
  composerDraft.value = {
    type: 'reply',
    original: selectedMessage.value,
  };
  showComposer.value = true;
}

function onSend(email: any) {
  emailStore.sendEmail(email);
  showComposer.value = false;
}
</script>
```

#### Email Store (Pinia)

```typescript
// src/modules/email/stores/email.ts

import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri';
import { EmailApiClient } from '../services/EmailApiClient';

export const useEmailStore = defineStore('email', {
  state: () => ({
    accounts: [] as any[],
    folders: [] as any[],
    messages: [] as any[],
    selectedAccount: null as any,
    loading: false,
    syncing: false,
  }),

  actions: {
    async loadAccounts() {
      this.accounts = await invoke('get_email_accounts');
      if (this.accounts.length > 0 && !this.selectedAccount) {
        this.selectedAccount = this.accounts[0];
      }
    },

    async loadFolders() {
      if (!this.selectedAccount) return;

      this.loading = true;
      try {
        // Try local first
        this.folders = await invoke('get_email_folders', {
          accountId: this.selectedAccount.id,
        });

        // Background sync
        this.syncFolders();
      } finally {
        this.loading = false;
      }
    },

    async loadMessages(folderId: number) {
      this.loading = true;
      try {
        // Load from local database
        this.messages = await invoke('get_email_messages', {
          folderId,
          limit: 50,
        });

        // Background sync
        this.syncMessages(folderId);
      } finally {
        this.loading = false;
      }
    },

    async syncFolders() {
      if (this.syncing) return;

      this.syncing = true;
      try {
        const apiClient = new EmailApiClient();
        const folders = await apiClient.getFolders(this.selectedAccount.id);

        // Save to local database
        await invoke('save_email_folders', {
          accountId: this.selectedAccount.id,
          folders,
        });

        // Reload
        this.folders = await invoke('get_email_folders', {
          accountId: this.selectedAccount.id,
        });
      } finally {
        this.syncing = false;
      }
    },

    async syncMessages(folderId: number) {
      const apiClient = new EmailApiClient();

      // Get last sync timestamp
      const lastSync = await invoke('get_last_sync', { folderId });

      // Fetch new messages
      const messages = await apiClient.getMessages(folderId, lastSync);

      // Save to local database
      await invoke('save_email_messages', {
        folderId,
        messages,
      });

      // Reload
      this.messages = await invoke('get_email_messages', {
        folderId,
        limit: 50,
      });
    },

    async sendEmail(email: any) {
      // Save to outbox first (offline support)
      await invoke('save_to_outbox', { email });

      try {
        const apiClient = new EmailApiClient();
        await apiClient.sendEmail(email);

        // Mark as sent
        await invoke('mark_as_sent', { email });
      } catch (error) {
        // Will retry from outbox later
        console.error('Failed to send email:', error);
      }
    },

    async markAsRead(messageId: number) {
      // Update local immediately
      await invoke('mark_message_read', { messageId });

      // Update on server (background)
      const apiClient = new EmailApiClient();
      apiClient.updateFlags(messageId, { is_read: true });

      // Update state
      const msg = this.messages.find(m => m.id === messageId);
      if (msg) msg.is_read = true;
    },
  },
});
```

---

## 📅 Calendar Module Implementation

### Key Components

#### CalendarView.vue

```vue
<template>
  <div class="calendar-container">
    <!-- View Switcher -->
    <div class="flex justify-between items-center p-4 border-b">
      <div class="flex gap-2">
        <button @click="previousPeriod" class="btn btn-sm">←</button>
        <button @click="today" class="btn btn-sm">Today</button>
        <button @click="nextPeriod" class="btn btn-sm">→</button>
      </div>

      <h2 class="text-xl font-bold">{{ currentPeriodLabel }}</h2>

      <div class="flex gap-2">
        <button
          v-for="view in ['day', 'week', 'month']"
          :key="view"
          @click="currentView = view"
          :class="{'btn-primary': currentView === view}"
          class="btn btn-sm"
        >
          {{ view }}
        </button>
        <button @click="showEventModal = true" class="btn btn-sm btn-primary">
          + New Event
        </button>
      </div>
    </div>

    <!-- Calendar Grid -->
    <component
      :is="viewComponent"
      :events="events"
      :current-date="currentDate"
      @event-click="onEventClick"
      @date-click="onDateClick"
    />

    <!-- Event Modal -->
    <EventEditor
      v-if="showEventModal"
      :event="selectedEvent"
      @save="onSaveEvent"
      @delete="onDeleteEvent"
      @close="showEventModal = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useCalendarStore } from '../stores/calendar';
import MonthView from './MonthView.vue';
import WeekView from './WeekView.vue';
import DayView from './DayView.vue';
import EventEditor from './EventEditor.vue';

const calendarStore = useCalendarStore();

const currentView = ref('month');
const currentDate = ref(new Date());
const showEventModal = ref(false);
const selectedEvent = ref(null);

const events = computed(() => calendarStore.events);

const viewComponent = computed(() => {
  switch (currentView.value) {
    case 'day': return DayView;
    case 'week': return WeekView;
    case 'month': return MonthView;
  }
});

const currentPeriodLabel = computed(() => {
  // Format based on current view and date
  const options: any = { month: 'long', year: 'numeric' };
  return currentDate.value.toLocaleDateString('en-US', options);
});

function today() {
  currentDate.value = new Date();
}

function previousPeriod() {
  // Adjust based on current view
  const date = new Date(currentDate.value);
  if (currentView.value === 'day') {
    date.setDate(date.getDate() - 1);
  } else if (currentView.value === 'week') {
    date.setDate(date.getDate() - 7);
  } else {
    date.setMonth(date.getMonth() - 1);
  }
  currentDate.value = date;
}

function nextPeriod() {
  const date = new Date(currentDate.value);
  if (currentView.value === 'day') {
    date.setDate(date.getDate() + 1);
  } else if (currentView.value === 'week') {
    date.setDate(date.getDate() + 7);
  } else {
    date.setMonth(date.getMonth() + 1);
  }
  currentDate.value = date;
}

function onEventClick(event: any) {
  selectedEvent.value = event;
  showEventModal.value = true;
}

function onDateClick(date: Date) {
  selectedEvent.value = {
    start_time: date,
    end_time: new Date(date.getTime() + 3600000), // +1 hour
  };
  showEventModal.value = true;
}

function onSaveEvent(event: any) {
  calendarStore.saveEvent(event);
  showEventModal.value = false;
}

function onDeleteEvent(eventId: number) {
  calendarStore.deleteEvent(eventId);
  showEventModal.value = false;
}
</script>
```

### Calendar-Email Integration

```typescript
// src/modules/calendar/services/CalendarEmailIntegration.ts

import { EventBus, EVENTS } from '@/core/plugin-system/EventBus';
import { useCalendarStore } from '../stores/calendar';

export class CalendarEmailIntegration {
  private eventBus: EventBus;

  constructor() {
    this.eventBus = EventBus.getInstance();
    this.setupListeners();
  }

  private setupListeners() {
    // When meeting invitation email received
    this.eventBus.on('email:meeting-invitation', (data: any) => {
      this.handleMeetingInvitation(data);
    });

    // When calendar event created
    this.eventBus.on('calendar:event-created', (event: any) => {
      if (event.attendees && event.attendees.length > 0) {
        this.sendMeetingInvitations(event);
      }
    });
  }

  private handleMeetingInvitation(data: any) {
    // Parse ICS file from email
    const event = this.parseICS(data.ics);

    // Show notification
    this.eventBus.emit(EVENTS.NOTIFICATION, {
      type: 'info',
      title: 'Meeting Invitation',
      message: `${data.from} invited you to: ${event.title}`,
      actions: [
        {
          label: 'Accept',
          handler: () => this.acceptInvitation(event),
          primary: true,
        },
        {
          label: 'Decline',
          handler: () => this.declineInvitation(event),
        },
      ],
    });
  }

  private async sendMeetingInvitations(event: any) {
    // Create ICS file
    const ics = this.generateICS(event);

    // Send email via Email module
    this.eventBus.emit('email:compose', {
      to: event.attendees,
      subject: `Meeting: ${event.title}`,
      body: this.generateInvitationEmail(event),
      attachments: [
        {
          filename: 'invitation.ics',
          content: ics,
          mime_type: 'text/calendar',
        },
      ],
    });
  }

  private parseICS(ics: string): any {
    // ICS parsing logic
    return {};
  }

  private generateICS(event: any): string {
    // ICS generation logic
    return '';
  }

  private generateInvitationEmail(event: any): string {
    return `
      <html>
        <body>
          <h2>${event.title}</h2>
          <p><strong>When:</strong> ${event.start_time} - ${event.end_time}</p>
          <p><strong>Where:</strong> ${event.location || 'TBD'}</p>
          <p>${event.description || ''}</p>
        </body>
      </html>
    `;
  }
}
```

---

## 👥 CRM Module Implementation

### Contact List Component

```vue
<!-- src/modules/crm/components/ContactList.vue -->

<template>
  <div class="contact-list">
    <!-- Search & Filters -->
    <div class="p-4 border-b">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search contacts..."
        class="input input-bordered w-full"
        @input="onSearch"
      />

      <div class="flex gap-2 mt-2">
        <select v-model="filterTag" class="select select-sm">
          <option value="">All Tags</option>
          <option v-for="tag in availableTags" :key="tag">{{ tag }}</option>
        </select>

        <select v-model="sortBy" class="select select-sm">
          <option value="name">Name</option>
          <option value="recent">Recently Added</option>
          <option value="score">Lead Score</option>
        </select>

        <button @click="showCreateModal = true" class="btn btn-sm btn-primary ml-auto">
          + New Contact
        </button>
      </div>
    </div>

    <!-- Contact List -->
    <div class="overflow-y-auto">
      <div
        v-for="contact in filteredContacts"
        :key="contact.id"
        @click="selectContact(contact)"
        :class="{'bg-base-200': selectedContact?.id === contact.id}"
        class="p-4 border-b cursor-pointer hover:bg-base-100"
      >
        <div class="flex items-center gap-3">
          <!-- Avatar -->
          <div class="avatar placeholder">
            <div class="bg-primary text-primary-content rounded-full w-10">
              <span>{{ getInitials(contact) }}</span>
            </div>
          </div>

          <!-- Info -->
          <div class="flex-1">
            <div class="font-semibold">
              {{ contact.first_name }} {{ contact.last_name }}
            </div>
            <div class="text-sm text-gray-500">
              {{ contact.email }}
            </div>
            <div class="flex gap-1 mt-1">
              <span
                v-for="tag in contact.tags"
                :key="tag"
                class="badge badge-sm"
              >
                {{ tag }}
              </span>
            </div>
          </div>

          <!-- Lead Score -->
          <div class="text-right">
            <div class="text-2xl font-bold">{{ contact.lead_score }}</div>
            <div class="text-xs text-gray-500">score</div>
          </div>
        </div>
      </div>

      <!-- Load More -->
      <div v-if="hasMore" class="p-4 text-center">
        <button @click="loadMore" class="btn btn-sm">
          Load More
        </button>
      </div>
    </div>

    <!-- Create Modal -->
    <ContactEditor
      v-if="showCreateModal"
      :contact="null"
      @save="onCreate"
      @close="showCreateModal = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useCrmStore } from '../stores/crm';
import ContactEditor from './ContactEditor.vue';

const crmStore = useCrmStore();

const searchQuery = ref('');
const filterTag = ref('');
const sortBy = ref('name');
const showCreateModal = ref(false);
const selectedContact = ref(null);

const contacts = computed(() => crmStore.contacts);
const availableTags = computed(() => crmStore.allTags);
const hasMore = computed(() => crmStore.hasMore);

const filteredContacts = computed(() => {
  let results = contacts.value;

  // Search filter
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    results = results.filter(c =>
      c.first_name?.toLowerCase().includes(query) ||
      c.last_name?.toLowerCase().includes(query) ||
      c.email?.toLowerCase().includes(query)
    );
  }

  // Tag filter
  if (filterTag.value) {
    results = results.filter(c => c.tags?.includes(filterTag.value));
  }

  // Sort
  results = [...results].sort((a, b) => {
    if (sortBy.value === 'name') {
      return a.last_name.localeCompare(b.last_name);
    } else if (sortBy.value === 'score') {
      return (b.lead_score || 0) - (a.lead_score || 0);
    } else {
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    }
  });

  return results;
});

function getInitials(contact: any): string {
  return (contact.first_name?.[0] || '') + (contact.last_name?.[0] || '');
}

function selectContact(contact: any) {
  selectedContact.value = contact;
  emit('select', contact);
}

function onSearch() {
  // Debounced search
}

function loadMore() {
  crmStore.loadMore();
}

function onCreate(contact: any) {
  crmStore.createContact(contact);
  showCreateModal.value = false;
}

const emit = defineEmits(['select']);
</script>
```

### CRM-Email Integration (Create Contact from Email)

```typescript
// src/modules/crm/services/CrmEmailIntegration.ts

import { EventBus } from '@/core/plugin-system/EventBus';
import { useCrmStore } from '../stores/crm';

export class CrmEmailIntegration {
  private eventBus: EventBus;
  private crmStore: any;

  constructor() {
    this.eventBus = EventBus.getInstance();
    this.crmStore = useCrmStore();
    this.setupListeners();
  }

  private setupListeners() {
    // Add "Create Contact" button to email reading pane
    this.eventBus.on('email:message-selected', (message: any) => {
      this.checkIfContactExists(message.from_address);
    });
  }

  private async checkIfContactExists(email: string) {
    const exists = await this.crmStore.findContactByEmail(email);

    if (!exists) {
      // Show quick action to create contact
      this.eventBus.emit('ui:show-quick-action', {
        label: 'Add to CRM',
        icon: 'user-plus',
        handler: () => this.createContactFromEmail(email),
      });
    }
  }

  private createContactFromEmail(email: string) {
    // Parse name from email if possible
    const [firstName, lastName] = this.parseNameFromEmail(email);

    // Create contact
    this.crmStore.createContact({
      first_name: firstName,
      last_name: lastName,
      email,
      tags: ['email-import'],
    });

    // Show success notification
    this.eventBus.emit('ui:notification', {
      type: 'success',
      title: 'Contact Created',
      message: `${firstName} ${lastName} added to CRM`,
    });
  }

  private parseNameFromEmail(email: string): [string, string] {
    // Simple parsing logic
    const localPart = email.split('@')[0];
    const parts = localPart.split('.');

    if (parts.length >= 2) {
      return [
        parts[0].charAt(0).toUpperCase() + parts[0].slice(1),
        parts[1].charAt(0).toUpperCase() + parts[1].slice(1),
      ];
    }

    return [localPart, ''];
  }
}
```

---

## 🔗 Integration Points

### Cross-Module Event Examples

```typescript
// Example: Email → CRM
eventBus.emit('contact:created', {
  source: 'email',
  contact: { /* contact data */ }
});

// Example: Calendar → Email
eventBus.emit('email:compose', {
  to: ['john@example.com'],
  subject: 'Meeting: Product Demo',
  attachments: [{ filename: 'invite.ics', content: icsData }]
});

// Example: CRM → Calendar
eventBus.emit('calendar:create-event', {
  title: 'Sales Call with John Doe',
  start_time: new Date(),
  attendees: ['john@example.com'],
  linked_contact_id: 123
});
```

---

## 🧪 Testing Strategy

### Unit Tests

```typescript
// Example: tests/modules/email/stores/email.spec.ts

import { setActivePinia, createPinia } from 'pinia';
import { describe, it, expect, beforeEach } from 'vitest';
import { useEmailStore } from '@/modules/email/stores/email';

describe('Email Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it('loads accounts on initialization', async () => {
    const store = useEmailStore();
    await store.loadAccounts();

    expect(store.accounts).toBeDefined();
    expect(store.accounts.length).toBeGreaterThan(0);
  });

  it('marks message as read', async () => {
    const store = useEmailStore();
    store.messages = [{ id: 1, is_read: false }];

    await store.markAsRead(1);

    expect(store.messages[0].is_read).toBe(true);
  });
});
```

### Integration Tests

```typescript
// Example: tests/integration/email-crm.spec.ts

import { describe, it, expect } from 'vitest';
import { EventBus } from '@/core/plugin-system/EventBus';
import { CrmEmailIntegration } from '@/modules/crm/services/CrmEmailIntegration';

describe('Email-CRM Integration', () => {
  it('creates contact when email received from unknown sender', async () => {
    const integration = new CrmEmailIntegration();
    const eventBus = EventBus.getInstance();

    // Simulate email received
    eventBus.emit('email:message-selected', {
      from_address: 'john.doe@example.com',
      from_name: 'John Doe'
    });

    // Should offer to create contact
    // Assertion logic here
  });
});
```

---

## ✅ MVP Acceptance Criteria

### Email Module
- [ ] User can add email account
- [ ] User can view folder tree
- [ ] User can view message list with pagination
- [ ] User can read messages
- [ ] User can compose and send emails
- [ ] User can reply/forward
- [ ] User can download attachments
- [ ] Offline mode works (read cached emails)
- [ ] Background sync runs every 5 minutes
- [ ] Search works across all messages

### Calendar Module
- [ ] User can view month/week/day views
- [ ] User can create events
- [ ] User can edit/delete events
- [ ] User can set reminders
- [ ] User can send meeting invitations via email
- [ ] User receives notifications for upcoming events
- [ ] Events sync with server
- [ ] Offline creation queued for sync

### CRM Module
- [ ] User can view contact list
- [ ] User can search/filter contacts
- [ ] User can create contacts
- [ ] User can edit contact details
- [ ] User can add tags to contacts
- [ ] User can create contact from email
- [ ] Contacts sync with server
- [ ] Lead scores displayed

### Cross-Module
- [ ] Email → CRM: Create contact from sender
- [ ] Calendar → Email: Send meeting invites
- [ ] CRM → Email: View contact emails
- [ ] Global search works across all modules
- [ ] Unified notification center

---

**Next Steps**: Begin Week 1 implementation. Backend developer starts with Symfony API, frontend developer initializes Tauri project.
