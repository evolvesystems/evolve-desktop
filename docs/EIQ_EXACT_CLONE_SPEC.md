# EvolveApp Desktop - EXACT EIQ Web App Clone

## Purpose
Create a desktop app that looks and works EXACTLY like the EIQ web app. No variations, no "improvements", just copy it exactly.

---

## Layout Analysis from ep.png Screenshot

### Screen Structure

```
┌────────────────────────────────────────────────────────────────┐
│ [Narrow Left Sidebar] │ [Main Content Area - Full Width]       │
│ 64px width            │                                         │
│                       │ ┌─────────────────────────────────────┐│
│ ┌─────┐              │ │ Top Header Bar                      ││
│ │  E  │ Logo         │ │ ☰ dashboard    [search⌘K][🔔][user]││
│ └─────┘              │ └─────────────────────────────────────┘│
│                       │                                         │
│   📧 Email icon      │ ┌─────────────────────────────────────┐│
│   (purple/blue       │ │ 🚀 NEW: Email & Chat modules...     ││
│    @ symbol)         │ │ (Green success banner)               ││
│                       │ └─────────────────────────────────────┘│
│                       │                                         │
│                       │ Dashboard                               │
│                       │ Welcome back, John North                │
│                       │                                         │
│                       │ ┌───┐ ┌───┐ ┌───┐ ┌───┐              │
│                       │ │ 0 │ │ 0 │ │ 0 │ │ 0 │  (Stats)      │
│                       │ └───┘ └───┘ └───┘ └───┘              │
│                       │                                         │
│                       │ Installed Modules                       │
│                       │ ┌───────────────────────┐             │
│                       │ │ 📧 Email              │             │
│                       │ │ Full-featured email..  │             │
│                       │ │ [communication]        │             │
│                       │ └───────────────────────┘             │
│                       │                                         │
│   👤 JN              │ Recent Activity                         │
│   (purple circle     │ (Empty state with clock icon)           │
│    at bottom)        │                                         │
└────────────────────────────────────────────────────────────────┘
```

---

## EXACT Layout Specifications

### 1. Left Sidebar (Narrow Icon Bar)

**Width**: 64px (fixed, never changes)
**Background**: Light gray (#F3F4F6 or similar)
**Border**: Right border, light gray

**Contents (top to bottom)**:
1. **Logo** (top)
   - Rounded square
   - Purple/blue gradient background
   - White "E" letter
   - ~48px × 48px
   - Centered

2. **Icon buttons** (middle)
   - Email icon (📧 or mail icon)
   - Appears to be purple/blue when active
   - ~40px × 40px clickable area
   - Centered in sidebar

3. **User avatar** (bottom)
   - Circular avatar
   - Purple background
   - "JN" initials in white
   - ~32px diameter
   - Centered
   - At absolute bottom with ~16px padding

**Behavior**:
- Icons are clickable
- Active icon has colored background/highlight
- Hover shows tooltip with name

---

### 2. Main Content Area

**Layout**: Full width minus sidebar (calc(100% - 64px))
**Background**: White or very light gray

#### A. Top Header Bar

**Height**: ~64px
**Background**: White
**Border**: Bottom border, light gray
**Shadow**: Subtle shadow below

**Contents (left to right)**:
1. **Hamburger menu icon** (far left)
   - Three horizontal lines
   - ~24px × 24px
   - Likely toggles mobile sidebar (not used on desktop)

2. **Page title** (left)
   - "dashboard" in regular weight
   - ~20px font size
   - ~16px left margin from hamburger

3. **Search button** (far right area)
   - Magnifying glass icon
   - With "⌘K" keyboard shortcut hint
   - Gray background, rounded

4. **Notification bell** (far right area)
   - Bell icon 🔔
   - ~8px right of search

5. **User menu** (far right)
   - Appears to be a small icon/dropdown trigger
   - ~8px right of bell
   - Opens user dropdown menu

**Layout**: Flexbox, space-between alignment (left group vs right group)

---

#### B. Page Content

**Padding**: 32px all sides
**Max width**: Container with max-width ~1400px, centered

**Dashboard Page Structure**:

1. **Success Banner** (if present)
   - Green background (#10B981 or similar)
   - Checkmark icon on left
   - Message text
   - Full width
   - Rounded corners
   - ~16px margin bottom

2. **Page Heading**
   - "Dashboard" - Large, bold (32px+)
   - "Welcome back, [Name]" - Smaller, gray text below
   - ~32px margin bottom

3. **Stats Grid**
   - 4 cards in a row
   - Equal width, responsive grid
   - Each card shows:
     - Icon (colored)
     - Label
     - Number value
   - White background
   - Shadow
   - Rounded corners
   - ~24px margin bottom

4. **Installed Modules Section**
   - "Installed Modules" heading
   - Grid of module cards (3 per row on desktop)
   - Each card:
     - Icon (left)
     - Title
     - Description text
     - Category badge
     - White background
     - Shadow on hover
     - Clickable
   - ~24px margin bottom

5. **Recent Activity Section**
   - "Recent Activity" heading
   - Empty state with icon
   - Or list of activities

---

## Color Palette (from screenshot)

### Primary Colors
- **Sidebar background**: #F3F4F6 (light gray)
- **Logo background**: Purple/blue gradient (#8B5CF6 to #3B82F6)
- **Active icon**: Purple/blue (#8B5CF6)
- **Success green**: #10B981
- **Main background**: White #FFFFFF

### Text Colors
- **Headings**: #111827 (very dark gray/black)
- **Body**: #6B7280 (medium gray)
- **Light text**: #9CA3AF (light gray)

### UI Elements
- **Borders**: #E5E7EB (light gray)
- **Shadows**: rgba(0, 0, 0, 0.1)
- **Card backgrounds**: White #FFFFFF

---

## Typography

### Font Family
- Sans-serif (likely Inter, Roboto, or system default)

### Sizes
- **Large heading**: 32px, bold
- **Section heading**: 20px, bold
- **Page title**: 20px, regular
- **Body**: 14px, regular
- **Small text**: 12px, regular

### Weights
- **Bold**: 700
- **Semibold**: 600
- **Regular**: 400

---

## Component Specifications

### Module Card
```
┌──────────────────────────┐
│ 📧  Email               │
│     Full-featured...     │
│     [communication]      │
└──────────────────────────┘
```
- Icon: Large emoji or icon (left)
- Title: Bold, dark
- Description: Regular, gray, truncated
- Badge: Small, rounded, dark background
- Hover: Shadow increases
- Click: Navigate to module

### Stats Card
```
┌────────────┐
│ 📧  Icon   │
│ Unread     │
│ 0          │
└────────────┘
```
- Icon: Colored, medium size
- Label: Small, gray
- Value: Large number, colored

---

## Interaction Patterns

### Navigation
1. Click sidebar icon → Navigate to page
2. Click module card → Navigate to module
3. Click user avatar → Show dropdown menu
4. Click notification bell → Show notifications

### States
- **Hover**: Subtle background change
- **Active**: Colored background or border
- **Loading**: Spinner or skeleton
- **Empty**: Icon + message

---

## Responsive Behavior

### Desktop (1024px+)
- Sidebar visible, 64px
- Main content full width
- Module cards: 3 per row
- Stats: 4 per row

### Tablet (768px - 1023px)
- Sidebar visible, 64px
- Module cards: 2 per row
- Stats: 2 per row

### Mobile (< 768px)
- Sidebar hidden by default
- Hamburger menu to show/hide
- Module cards: 1 per row
- Stats: 2 per row

---

## File Structure to Match EIQ

```
src/
├── layouts/
│   └── EIQLayout.vue           ← EXACT copy of EIQ layout
│
├── views/
│   ├── Dashboard.vue           ← Homepage with stats + modules
│   ├── EmailInbox.vue          ← Email module page
│   ├── ChatView.vue            ← Chat module page
│   └── Settings.vue            ← Settings page
│
├── components/
│   ├── TopBar.vue              ← Header with search/notifications
│   ├── ModuleCard.vue          ← Clickable module card
│   ├── StatsCard.vue           ← Stats display card
│   └── Sidebar.vue             ← Icon sidebar
│
└── assets/
    └── styles/
        └── eiq-theme.css       ← EIQ color variables
```

---

## CSS Framework Match

EIQ appears to use:
- **Tailwind CSS** for utilities
- **Custom component styles**
- **Consistent spacing**: 4px, 8px, 16px, 24px, 32px
- **Consistent border radius**: 8px for cards, 4px for buttons
- **Consistent shadows**: Subtle, layered

Desktop app should use:
- **Same Tailwind config**
- **Same color variables**
- **Same spacing scale**
- **Same border radius values**

---

## Implementation Checklist

### Phase 1: Layout Structure
- [ ] Create EIQLayout.vue with exact dimensions
- [ ] Create Sidebar with 64px width, icons, avatar
- [ ] Create TopBar with search, notifications, user menu
- [ ] Test layout matches screenshot exactly

### Phase 2: Dashboard Page
- [ ] Create Dashboard.vue
- [ ] Add page heading with welcome message
- [ ] Add stats grid (4 cards)
- [ ] Add "Installed Modules" section
- [ ] Add module cards (clickable)
- [ ] Add "Recent Activity" section
- [ ] Test dashboard matches screenshot exactly

### Phase 3: Navigation
- [ ] Wire up sidebar icon clicks
- [ ] Wire up module card clicks
- [ ] Test all navigation works
- [ ] Add active states to sidebar icons

### Phase 4: Email Page
- [ ] Create EmailInbox.vue using EIQ email layout
- [ ] Test email page loads correctly
- [ ] Test navigation to/from email works

### Phase 5: Polish
- [ ] Match all colors exactly
- [ ] Match all spacing exactly
- [ ] Match all fonts exactly
- [ ] Match all interactions exactly

---

## Success Criteria

Desktop app is successful when:
1. ✅ Screenshot of desktop app looks identical to ep.png
2. ✅ Sidebar is exactly 64px wide
3. ✅ All colors match EIQ
4. ✅ All spacing matches EIQ
5. ✅ All navigation works
6. ✅ Module cards are clickable
7. ✅ Stats display correctly
8. ✅ User can navigate between Dashboard, Email, Chat, Settings
9. ✅ No broken layouts
10. ✅ No hidden menus or confusing UI

---

## What NOT to Do

❌ Don't add extra features not in EIQ
❌ Don't change the layout structure
❌ Don't use different colors "because they look better"
❌ Don't add animations not in EIQ
❌ Don't create custom components if EIQ uses simple HTML
❌ Don't over-engineer - copy exactly what works

---

## Design System Variables

```css
/* Colors */
--sidebar-bg: #F3F4F6;
--sidebar-border: #E5E7EB;
--primary: #8B5CF6;
--primary-hover: #7C3AED;
--success: #10B981;
--text-primary: #111827;
--text-secondary: #6B7280;
--text-light: #9CA3AF;
--border: #E5E7EB;
--background: #FFFFFF;

/* Spacing */
--space-1: 4px;
--space-2: 8px;
--space-3: 12px;
--space-4: 16px;
--space-6: 24px;
--space-8: 32px;

/* Sizing */
--sidebar-width: 64px;
--topbar-height: 64px;
--container-max: 1400px;

/* Border Radius */
--radius-sm: 4px;
--radius: 8px;
--radius-lg: 12px;
--radius-full: 9999px;

/* Shadows */
--shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
--shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
--shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
```

---

**REMEMBER: This is a CLONE spec. Copy exactly. Don't innovate. Don't improve. Just copy.**
