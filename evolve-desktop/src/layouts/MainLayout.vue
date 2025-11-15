<template>
  <div class="flex h-screen overflow-hidden bg-base-100">
    <!-- Sidebar -->
    <aside
      v-show="appStore.sidebarOpen"
      class="w-16 flex flex-col bg-base-200 border-r border-base-300"
    >
      <!-- Logo -->
      <div class="h-16 flex items-center justify-center border-b border-base-300">
        <div class="w-10 h-10 rounded-lg bg-primary flex items-center justify-center">
          <span class="text-primary-content font-bold text-lg">E</span>
        </div>
      </div>

      <!-- Module Navigation -->
      <nav class="flex-1 py-4 space-y-2">
        <ModuleNavItem
          v-for="module in enabledModules"
          :key="module.id"
          :module="module"
        />
      </nav>

      <!-- Bottom Actions -->
      <div class="py-4 space-y-2 border-t border-base-300">
        <button
          @click="() => router.push('/settings')"
          class="w-full h-12 flex items-center justify-center hover:bg-base-300 transition-colors"
          :class="{ 'bg-base-300': route.path === '/settings' }"
          title="Settings"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </button>

        <!-- Theme Toggle -->
        <button
          @click="toggleTheme"
          class="w-full h-12 flex items-center justify-center hover:bg-base-300 transition-colors"
          title="Toggle Theme"
        >
          <svg v-if="appStore.effectiveTheme === 'light'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
          </svg>
          <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
        </button>

        <!-- User Menu -->
        <div class="dropdown dropdown-top dropdown-end">
          <button
            tabindex="0"
            class="w-full h-12 flex items-center justify-center hover:bg-base-300 transition-colors"
            title="User Menu"
          >
            <div class="avatar">
              <div class="w-8 h-8 rounded-full bg-primary">
                <span class="text-primary-content text-sm font-medium flex items-center justify-center h-full">
                  {{ userInitials }}
                </span>
              </div>
            </div>
          </button>
          <ul tabindex="0" class="dropdown-content menu p-2 shadow-lg bg-base-200 rounded-box w-52 mb-2">
            <li class="menu-title">
              <span>{{ authStore.user?.name }}</span>
            </li>
            <li><a @click="() => router.push('/settings')">Settings</a></li>
            <li><a @click="handleLogout" class="text-error">Logout</a></li>
          </ul>
        </div>
      </div>
    </aside>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- Top Bar -->
      <header class="h-16 flex items-center justify-between px-4 border-b border-base-300 bg-base-100">
        <div class="flex items-center gap-4">
          <button
            @click="appStore.toggleSidebar"
            class="btn btn-ghost btn-sm btn-square"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>

          <h1 class="text-lg font-semibold">{{ pageTitle }}</h1>
        </div>

        <div class="flex items-center gap-2">
          <!-- Global Search -->
          <button
            @click="appStore.openSearch"
            class="btn btn-ghost btn-sm gap-2"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <span class="text-xs opacity-70">⌘K</span>
          </button>

          <!-- Notifications -->
          <div class="dropdown dropdown-end">
            <button tabindex="0" class="btn btn-ghost btn-sm btn-square relative">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
              </svg>
              <span
                v-if="appStore.unreadNotifications > 0"
                class="absolute top-0 right-0 w-5 h-5 bg-error text-error-content rounded-full text-xs flex items-center justify-center"
              >
                {{ appStore.unreadNotifications > 9 ? '9+' : appStore.unreadNotifications }}
              </span>
            </button>
            <div tabindex="0" class="dropdown-content mt-2 w-80 max-h-96 overflow-auto bg-base-200 rounded-box shadow-lg">
              <NotificationList />
            </div>
          </div>
        </div>
      </header>

      <!-- Page Content -->
      <main class="flex-1 overflow-auto">
        <router-view />
      </main>
    </div>

    <!-- Global Search Modal -->
    <GlobalSearch />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useAppStore } from '@/stores/app'
import { moduleRegistry } from '@/core/plugin-system'
import ModuleNavItem from '@/components/sidebar/ModuleNavItem.vue'
import NotificationList from '@/components/common/NotificationList.vue'
import GlobalSearch from '@/components/common/GlobalSearch.vue'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const appStore = useAppStore()

const enabledModules = computed(() => {
  return moduleRegistry
    .getAllInstallations()
    .filter((inst) => inst.enabled)
    .map((inst) => moduleRegistry.getModule(inst.moduleId))
    .filter((mod) => mod !== undefined)
})

const pageTitle = computed(() => {
  return route.meta.title || route.name || 'EvolveApp'
})

const userInitials = computed(() => {
  if (!authStore.user) return '?'
  const name = authStore.user.name
  return name
    .split(' ')
    .map((part) => part[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)
})

function toggleTheme() {
  const newTheme = appStore.effectiveTheme === 'light' ? 'dark' : 'light'
  appStore.setTheme(newTheme)
}

async function handleLogout() {
  await authStore.logout()
  router.push('/login')
}
</script>
