# EvolveApp Desktop - Clean Start Specification

## THE PROBLEM

We have 3 overlapping menu systems:
1. MainLayout with tiny 64px sidebar
2. Dashboard with module cards
3. SimpleDashboard with left sidebar

**This is confusing and broken. We need ONE system.**

## THE SOLUTION - Single Layout System

### One App Layout - That's It

```
┌─────────────────────────────────────────────────────────┐
│ [Top Bar - Full Width]                                  │
│ EvolveApp Desktop          [Search] [Notifications] [👤]│
├────────────┬────────────────────────────────────────────┤
│ SIDEBAR    │ MAIN CONTENT                               │
│ 200px      │ (Changes based on what you click)          │
│            │                                            │
│ Dashboard  │                                            │
│ Emails     │                                            │
│ Chat       │                                            │
│ Calendar   │                                            │
│ Contacts   │                                            │
│            │                                            │
│ ────────   │                                            │
│ Settings   │                                            │
│ Help       │                                            │
│ Logout     │                                            │
└────────────┴────────────────────────────────────────────┘
```

### Rules

1. **ONE layout file**: `AppLayout.vue`
2. **Sidebar ALWAYS visible**: No collapsing, no hiding
3. **Top bar ALWAYS visible**: Simple and clean
4. **Main content area**: Router view shows different pages
5. **NO module system**: Just regular Vue routes

## File Structure - CLEAN

```
src/
├── layouts/
│   └── AppLayout.vue          ← ONE layout, that's it
│
├── views/
│   ├── Dashboard.vue          ← Home page
│   ├── EmailInbox.vue         ← Email page
│   ├── ChatView.vue           ← Chat page
│   ├── Settings.vue           ← Settings page
│   └── Login.vue              ← Login (no layout)
│
├── components/
│   └── (only when needed)
│
├── stores/
│   ├── auth.ts
│   ├── email.ts
│   └── chat.ts
│
└── router/
    └── index.ts               ← Simple routes
```

## Router - Simple Routes

```typescript
const routes = [
  { path: '/login', component: Login },           // No layout
  { path: '/', component: Dashboard },            // Uses AppLayout
  { path: '/email', component: EmailInbox },      // Uses AppLayout
  { path: '/chat', component: ChatView },         // Uses AppLayout
  { path: '/settings', component: Settings },     // Uses AppLayout
]
```

## AppLayout.vue - The One Layout

```vue
<template>
  <div class="flex h-screen">
    <!-- SIDEBAR -->
    <aside class="w-64 bg-gray-800 text-white">
      <div class="p-4">
        <h1 class="text-xl font-bold">EvolveApp</h1>
      </div>

      <nav class="mt-8">
        <router-link to="/" class="nav-item">Dashboard</router-link>
        <router-link to="/email" class="nav-item">Emails</router-link>
        <router-link to="/chat" class="nav-item">Chat</router-link>
        <router-link to="/settings" class="nav-item">Settings</router-link>
      </nav>

      <div class="absolute bottom-4 left-4">
        <button @click="logout">Logout</button>
      </div>
    </aside>

    <!-- MAIN CONTENT -->
    <main class="flex-1 overflow-auto">
      <router-view />
    </main>
  </div>
</template>
```

## What to DELETE

### Files to Remove:
- ❌ `src/layouts/MainLayout.vue` (broken tiny sidebar)
- ❌ `src/views/SimpleDashboard.vue` (redundant)
- ❌ `src/core/plugin-system/*` (over-engineered)
- ❌ `src/modules/email/index.ts` (plugin stuff)
- ❌ `src/components/sidebar/ModuleNavItem.vue` (module system)

### Files to KEEP and SIMPLIFY:
- ✅ `src/views/Login.vue` (works fine)
- ✅ `src/stores/auth.ts` (works fine)
- ✅ `src/services/emailService.ts` (keep the API calls)
- ✅ `src/stores/email.ts` (keep the state)

## Step-by-Step Implementation

### Step 1: Create AppLayout.vue
Simple, clean, ONE layout that works.

### Step 2: Update Router
```typescript
// App.vue
<template>
  <component :is="layout">
    <router-view />
  </component>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import AppLayout from './layouts/AppLayout.vue'

const route = useRoute()
const layout = computed(() => {
  // No layout for login page
  if (route.path === '/login') {
    return 'div'
  }
  return AppLayout
})
</script>
```

### Step 3: Create Simple Views
Each view is just a regular component:

**Dashboard.vue**: Welcome screen with quick stats
**EmailInbox.vue**: Simple email list
**ChatView.vue**: Simple chat interface
**Settings.vue**: Settings form

### Step 4: Delete Module System
Remove all the plugin/module system code. We don't need it.

### Step 5: Test
Click through every nav item, make sure it works.

## Navigation Flow

```
Login Page (no sidebar)
    ↓ (successful login)
Dashboard (with sidebar)
    ↓ (click "Emails" in sidebar)
Email Inbox (same sidebar visible)
    ↓ (click "Chat" in sidebar)
Chat View (same sidebar visible)
    ↓ (click "Logout" in sidebar)
Login Page (no sidebar)
```

**ONE layout. SIMPLE navigation. NO confusion.**

## UI Design - Keep It Simple

### Colors
- Sidebar: Dark gray (#1F2937)
- Sidebar text: White
- Active nav item: Lighter background (#374151)
- Main content: White background
- Top bar: Light gray (#F9FAFB)

### Typography
- Sidebar nav: 16px
- Page headings: 24px bold
- Body text: 14px

### Spacing
- Nav items: 12px padding
- Page content: 32px padding
- Gaps: 16px standard

## What Makes This Better

1. **ONE layout system** instead of 3
2. **Clear navigation** - sidebar always visible
3. **Simple routing** - no module loader
4. **Easy to understand** - regular Vue components
5. **Easy to debug** - fewer moving parts

## Implementation Order

1. Create `AppLayout.vue` (ONE simple layout)
2. Update `App.vue` to use it
3. Create simple `Dashboard.vue`
4. Create simple `EmailInbox.vue`
5. Test navigation works
6. Delete old broken code
7. Build chat view
8. Done

**NO module system. NO plugin architecture. Just Vue Router and components.**

---

## Current Status

- ❌ 3 different layouts fighting each other
- ❌ Module system over-complicating things
- ❌ Navigation doesn't work
- ❌ Menus off-screen

## After This Spec

- ✅ 1 simple layout
- ✅ Simple Vue Router
- ✅ Navigation that works
- ✅ Everything visible and clickable

**Let's build THIS instead.**
