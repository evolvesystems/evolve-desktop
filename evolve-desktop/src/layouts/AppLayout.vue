<template>
  <!-- EXACT CLONE OF EIQ WEB APP LAYOUT -->
  <div class="drawer">
    <input id="main-drawer" type="checkbox" class="drawer-toggle" v-model="drawerOpen" />

    <!-- MAIN CONTENT -->
    <div class="drawer-content h-screen flex bg-base-100">

      <!-- LEFT SIDEBAR - 64px icon-only sidebar exactly like EIQ -->
    <div class="w-16 bg-base-200 flex flex-col items-center py-4 border-r border-base-300">

      <!-- Logo at top -->
      <router-link to="/dashboard" class="mb-8">
        <div class="w-12 h-12 rounded-lg bg-gradient-to-br from-purple-600 to-blue-500 flex items-center justify-center text-white font-bold text-xl shadow-md">
          E
        </div>
      </router-link>

      <!-- Navigation Icons -->
      <div class="flex flex-col gap-4 flex-1">
        <router-link
          to="/email"
          class="sidebar-icon"
          title="Email"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
          </svg>
        </router-link>

        <router-link
          to="/chat"
          class="sidebar-icon"
          title="Chat"
        >
          💬
        </router-link>

        <router-link
          to="/settings"
          class="sidebar-icon"
          title="Settings"
        >
          ⚙️
        </router-link>
      </div>
    </div>

    <!-- MAIN CONTENT AREA -->
    <div class="flex-1 flex flex-col overflow-hidden">

      <!-- TOP HEADER BAR - Exactly like EIQ -->
      <div class="h-16 bg-base-100 border-b border-base-300 flex items-center justify-between px-6 shadow-sm">

        <!-- Left side: Hamburger + Page Title -->
        <div class="flex items-center gap-4">
          <label for="main-drawer" class="btn btn-ghost btn-sm btn-square drawer-button">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </label>
          <h1 class="text-lg font-normal text-base-content">{{ pageTitle }}</h1>
        </div>

        <!-- Right side: Search, Notifications, User Menu -->
        <div class="flex items-center gap-2">
          <!-- Search button -->
          <button class="btn btn-ghost btn-sm gap-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <kbd class="kbd kbd-sm">⌘K</kbd>
          </button>

          <!-- Notification bell -->
          <button class="btn btn-ghost btn-sm btn-circle">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
            </svg>
          </button>

          <!-- User menu dropdown with avatar -->
          <div class="dropdown dropdown-end">
            <label tabindex="0" class="btn btn-ghost btn-sm btn-circle avatar">
              <div class="w-8 h-8 rounded-full bg-primary text-primary-content flex items-center justify-center">
                <span class="text-xs font-semibold">{{ userInitials }}</span>
              </div>
            </label>
            <ul tabindex="0" class="dropdown-content menu p-2 shadow-lg bg-base-100 rounded-box w-52 border border-base-300 mt-2">
              <li class="menu-title"><span>{{ authStore.user?.name }}</span></li>
              <li class="menu-title text-xs opacity-60"><span>{{ authStore.user?.email }}</span></li>
              <div class="divider my-1"></div>
              <li><router-link to="/settings">Settings</router-link></li>
              <li><a @click="logout">Logout</a></li>
            </ul>
          </div>
        </div>
      </div>

      <!-- PAGE CONTENT -->
      <div class="flex-1 overflow-auto">
        <router-view />
      </div>
    </div>
  </div>

    <!-- DRAWER SIDE MENU (slides out when hamburger clicked) -->
    <div class="drawer-side z-50">
      <label for="main-drawer" class="drawer-overlay"></label>
      <div class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
        <!-- Drawer Header -->
        <div class="mb-6">
          <div class="flex items-center gap-3 mb-4">
            <div class="w-12 h-12 rounded-lg bg-gradient-to-br from-purple-600 to-blue-500 flex items-center justify-center text-white font-bold text-xl shadow-md">
              E
            </div>
            <div>
              <h2 class="font-bold text-lg">EvolveApp</h2>
              <p class="text-sm text-base-content/60">Desktop</p>
            </div>
          </div>
        </div>

        <!-- Main Navigation -->
        <ul class="space-y-1">
          <li>
            <router-link to="/dashboard" @click="closeDrawer" class="flex items-center gap-3">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
              </svg>
              <span>Dashboard</span>
            </router-link>
          </li>

          <li class="menu-title">
            <span>Communication</span>
          </li>

          <li>
            <router-link to="/email" @click="closeDrawer" class="flex items-center gap-3">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
              </svg>
              <span>Email</span>
            </router-link>
          </li>

          <li>
            <router-link to="/chat" @click="closeDrawer" class="flex items-center gap-3">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
              </svg>
              <span>Team Chat</span>
            </router-link>
          </li>

          <li class="menu-title">
            <span>Productivity</span>
          </li>

          <li>
            <a class="flex items-center gap-3 opacity-50 cursor-not-allowed">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <span>Calendar</span>
              <span class="badge badge-sm">Soon</span>
            </a>
          </li>

          <li>
            <a class="flex items-center gap-3 opacity-50 cursor-not-allowed">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
              </svg>
              <span>Tasks</span>
              <span class="badge badge-sm">Soon</span>
            </a>
          </li>

          <li class="menu-title">
            <span>Business</span>
          </li>

          <li>
            <a class="flex items-center gap-3 opacity-50 cursor-not-allowed">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
              </svg>
              <span>CRM</span>
              <span class="badge badge-sm">Soon</span>
            </a>
          </li>

          <li>
            <a class="flex items-center gap-3 opacity-50 cursor-not-allowed">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              <span>Invoicing</span>
              <span class="badge badge-sm">Soon</span>
            </a>
          </li>
        </ul>

        <!-- Divider -->
        <div class="divider"></div>

        <!-- Bottom Menu Items -->
        <ul class="space-y-1">
          <li>
            <router-link to="/settings" @click="closeDrawer" class="flex items-center gap-3">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <span>Settings</span>
            </router-link>
          </li>

          <li>
            <a @click="logout" class="flex items-center gap-3">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
              </svg>
              <span>Logout</span>
            </a>
          </li>
        </ul>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

const drawerOpen = ref(false)

const userInitials = computed(() => {
  if (!authStore.user) return '?'
  const name = authStore.user.name
  return name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2)
})

const pageTitle = computed(() => {
  // Get page title from route meta or name
  return route.meta.title as string || route.name as string || 'dashboard'
})

function closeDrawer() {
  drawerOpen.value = false
}

async function logout() {
  console.log('[AppLayout] Logout clicked')
  try {
    await authStore.logout()
    console.log('[AppLayout] Logout successful, redirecting to login...')
    await router.push('/login')
    console.log('[AppLayout] Redirected to login')
  } catch (error) {
    console.error('[AppLayout] Logout error:', error)
  }
}
</script>

<style scoped>
.sidebar-icon {
  @apply w-12 h-12 flex items-center justify-center text-2xl rounded-lg transition-colors cursor-pointer;
  @apply hover:bg-base-300;
}

.sidebar-icon.router-link-active {
  @apply bg-primary text-primary-content;
}

/* Make sure SVG icons in sidebar-icon take current color */
.sidebar-icon svg {
  @apply text-current;
}
</style>
