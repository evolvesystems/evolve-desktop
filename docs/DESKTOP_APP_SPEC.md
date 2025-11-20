# EvolveApp Desktop - Product Specification & Design

## Executive Summary

A simple, clean desktop app for business management. Focus: **EMAIL and CHAT first**. Everything else is secondary.

## Core Philosophy

1. **Simple over complex** - No fancy architecture until it's needed
2. **Working over perfect** - Ship working features, refine later
3. **Visible over hidden** - Everything should be obvious and clickable
4. **Fast over fancy** - Performance matters more than animations

## Primary User Flow

```
Login → Dashboard → Click "Emails" or "Chat" → Use the app
```

That's it. No wizards, no complex setup, no hidden menus.

## Layout Design

### Master Layout
```
┌─────────────────────────────────────────────────┐
│ [Left Sidebar - 240px] │ [Main Content Area]   │
│                        │                        │
│ 🏠 Dashboard          │  ┌──────────────────┐ │
│ 📧 Emails             │  │  Page Header     │ │
│ 💬 Chat               │  ├──────────────────┤ │
│ ⚙️ Settings           │  │                  │ │
│                        │  │  Page Content    │ │
│ ─────────────────      │  │                  │ │
│ [User Info]            │  │                  │ │
│ John North             │  └──────────────────┘ │
│ john@example.com       │                        │
│ 🚪 Logout              │                        │
└─────────────────────────────────────────────────┘
```

### Key UI Rules

1. **Sidebar is ALWAYS visible** - No hamburger menus, no collapsing
2. **Big, clear buttons** - Minimum 44px height for all clickable items
3. **Clear active states** - Current page is highlighted
4. **No dropdowns in sidebar** - Everything is one click
5. **User info always visible** - No hidden profile menus

## Email Module Design

### Email View Layout
```
┌──────────────────────────────────────────────────────────┐
│ Email                                    [Search] [New]   │
├──────────────┬──────────────────┬────────────────────────┤
│ Folders      │ Email List       │ Email Content          │
│ 180px        │ 320px            │ Rest of space          │
│              │                  │                        │
│ 📥 Inbox (5) │ ■ John Smith     │ Subject: Re: Meeting   │
│ 📤 Sent      │   Meeting today  │ From: john@...         │
│ 📝 Drafts    │   Hey, about...  │ To: me@...             │
│ 🗑️ Trash     │   2:30 PM        │ ─────────────────────  │
│              │                  │                        │
│              │ □ Sarah Lee      │ Email body content...  │
│              │   Project update │                        │
│              │   Lorem ipsum... │                        │
│              │   Yesterday      │                        │
│              │                  │ [Reply] [Forward]      │
└──────────────┴──────────────────┴────────────────────────┘
```

### Email Features - MVP

**MUST HAVE:**
- ✅ View inbox
- ✅ Read emails
- ✅ Compose new email
- ✅ Reply to email
- ✅ Delete email
- ✅ Search emails

**NICE TO HAVE (v2):**
- Forward emails
- Attachments
- Multiple accounts
- Folders/labels
- Rich text formatting

## Chat Module Design

### Chat View Layout
```
┌──────────────────────────────────────────────────────────┐
│ Team Chat                                    [Search]     │
├──────────────┬───────────────────────────────────────────┤
│ Conversations│ Active Conversation                       │
│ 240px        │ Rest of space                             │
│              │                                           │
│ 🟢 John      │ #general                                  │
│   Hey there  │ ─────────────────────────────────────────│
│   5:30 PM    │                                           │
│              │ John: Hey team!          10:00 AM        │
│ 🟢 Sarah     │ Sarah: Good morning!     10:01 AM        │
│   Meeting... │ You: Ready for standup   10:02 AM        │
│   2:00 PM    │                                           │
│              │ ─────────────────────────────────────────│
│ #general     │ [Type message...] [Send]                 │
│ #support     │                                           │
└──────────────┴───────────────────────────────────────────┘
```

### Chat Features - MVP

**MUST HAVE:**
- ✅ Send/receive messages
- ✅ 1-on-1 conversations
- ✅ Group channels
- ✅ See who's online
- ✅ Message history

**NICE TO HAVE (v2):**
- File sharing
- @mentions
- Reactions
- Threads
- Video calls

## Settings Page

Simple form with sections:
1. **Account** - Name, email, password
2. **Server** - API URL configuration
3. **Appearance** - Theme (light/dark)
4. **About** - Version, help, logout

## Technical Architecture

### Keep It Simple

```
src/
├── views/
│   ├── SimpleDashboard.vue  ← Main dashboard
│   ├── SimpleEmail.vue      ← Email view
│   ├── SimpleChat.vue       ← Chat view
│   └── Settings.vue         ← Settings
├── components/
│   ├── Sidebar.vue          ← Reusable sidebar
│   └── ... (as needed)
├── stores/
│   ├── auth.ts              ← Authentication
│   ├── email.ts             ← Email data
│   └── chat.ts              ← Chat data
└── services/
    ├── emailService.ts      ← Email API calls
    └── chatService.ts       ← Chat API calls
```

### No Complex Module System

- Regular Vue components
- Simple routing
- Pinia stores for state
- Axios for API calls
- **That's it**

## Development Phases

### Phase 1: Foundation (CURRENT)
- ✅ Login/logout
- ✅ Simple dashboard with sidebar
- ⏳ Email view (in progress)
- ⏳ Make navigation actually work

### Phase 2: Email MVP
- [ ] Display inbox
- [ ] Read email
- [ ] Compose/send email
- [ ] Reply to email
- [ ] Delete email

### Phase 3: Chat MVP
- [ ] Display conversations
- [ ] Send messages
- [ ] Receive messages (polling)
- [ ] Online indicators

### Phase 4: Polish
- [ ] Better styling
- [ ] Loading states
- [ ] Error handling
- [ ] Settings page

## Design Principles

### Colors
- **Primary**: Blue (#3B82F6)
- **Success**: Green (#10B981)
- **Error**: Red (#EF4444)
- **Background**: White/Light gray
- **Sidebar**: Light gray (#F3F4F6)

### Typography
- **Headings**: Bold, clear
- **Body**: Regular, readable
- **Minimum size**: 14px

### Spacing
- **Minimum touch target**: 44px
- **Padding**: 16px standard
- **Gap between items**: 8px

## What NOT to Build

- ❌ Calendar (not MVP)
- ❌ File manager
- ❌ Complex CRM
- ❌ Video calls
- ❌ Mobile app
- ❌ Offline mode
- ❌ Plugin system
- ❌ Advanced permissions

## Success Criteria

**Phase 1 Success** = User can:
1. Login without confusion
2. See a clear dashboard
3. Click "Emails" and see their inbox
4. Click "Chat" and see their conversations
5. Navigate between pages easily
6. Logout

**NO hidden menus, NO complex workflows, NO confusion.**

## Current Problems to Fix

1. ❌ Navigation doesn't work properly
2. ❌ Email view is broken/tiny
3. ❌ User menu is off-screen
4. ❌ Too many unused features
5. ❌ Complex module architecture causing issues

## Next Steps

1. Fix SimpleDashboard sidebar navigation (make it actually work)
2. Build SimpleEmail.vue (just inbox list + read email)
3. Test it works end-to-end
4. Ship it

---

**Remember**: Working and simple beats complex and broken.
