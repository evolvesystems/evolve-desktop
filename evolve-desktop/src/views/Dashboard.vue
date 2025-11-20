<template>
  <div class="p-8">
    <!-- Success Banner -->
    <div v-if="showBanner" class="alert alert-success mb-6 shadow-md">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span>🚀 NEW: Email & Chat modules are now available! Changes updated at {{ updateTime }}</span>
    </div>

    <!-- Page Heading -->
    <div class="mb-8">
      <h1 class="text-4xl font-bold text-base-content mb-2">Dashboard</h1>
      <p class="text-lg text-base-content/60">Welcome back, {{ authStore.user?.name || 'User' }}</p>
    </div>

    <!-- Stats Grid - 4 cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <!-- Unread Emails -->
      <div class="stats shadow bg-base-100">
        <div class="stat">
          <div class="stat-figure text-primary">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>
          <div class="stat-title">Unread</div>
          <div class="stat-value text-primary">0</div>
        </div>
      </div>

      <!-- Events -->
      <div class="stats shadow bg-base-100">
        <div class="stat">
          <div class="stat-figure text-secondary">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
          </div>
          <div class="stat-title">Events</div>
          <div class="stat-value text-secondary">0</div>
        </div>
      </div>

      <!-- Active Chats -->
      <div class="stats shadow bg-base-100">
        <div class="stat">
          <div class="stat-figure text-accent">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z" />
            </svg>
          </div>
          <div class="stat-title">Active Chats</div>
          <div class="stat-value text-accent">0</div>
        </div>
      </div>

      <!-- Tasks -->
      <div class="stats shadow bg-base-100">
        <div class="stat">
          <div class="stat-figure text-info">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
          </div>
          <div class="stat-title">Reports</div>
          <div class="stat-value text-info">0</div>
        </div>
      </div>
    </div>

    <!-- Installed Modules Section -->
    <div class="mb-8">
      <h2 class="text-2xl font-bold text-base-content mb-4">Installed Modules</h2>

      <!-- Loading state -->
      <div v-if="loading" class="text-center py-12">
        <span class="loading loading-spinner loading-lg text-primary"></span>
        <p class="mt-4 text-base-content/60">Loading modules...</p>
      </div>

      <!-- Modules grid -->
      <div v-else-if="availableModules.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="module in availableModules"
          :key="module.key"
          @click="!module.isComingSoon && goToModule(module.key)"
          :class="[
            'card bg-base-100 shadow-md transition-shadow border border-base-300',
            module.isComingSoon ? 'opacity-60 cursor-not-allowed' : 'hover:shadow-xl cursor-pointer'
          ]"
        >
          <div class="card-body">
            <div class="flex items-start gap-3">
              <div class="text-4xl">
                {{ getModuleIcon(module.icon) }}
              </div>
              <div class="flex-1">
                <h3 class="card-title text-lg">
                  {{ module.name }}
                  <span v-if="module.isComingSoon" class="badge badge-sm badge-secondary ml-2">Soon</span>
                </h3>
                <p class="text-sm text-base-content/60">{{ module.description }}</p>
                <div class="mt-2">
                  <span class="badge badge-sm">{{ module.category }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty state -->
      <div v-else class="text-center py-12 text-base-content/50">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 mx-auto mb-4 opacity-30" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
        </svg>
        <p>No modules available</p>
      </div>
    </div>

    <!-- Recent Activity Section -->
    <div class="mb-8">
      <h2 class="text-2xl font-bold text-base-content mb-4">Recent Activity</h2>
      <div class="card bg-base-100 shadow-md border border-base-300">
        <div class="card-body">
          <div class="flex flex-col items-center justify-center py-12 text-base-content/40">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <p class="text-lg">No recent activity</p>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { modulesService, type DesktopModule } from '@/services/modulesService'

const router = useRouter()
const authStore = useAuthStore()

const showBanner = ref(true)
const updateTime = ref(new Date().toLocaleTimeString())
const availableModules = ref<DesktopModule[]>([])
const loading = ref(true)

onMounted(async () => {
  try {
    // Fetch available modules from API
    availableModules.value = await modulesService.getAvailableModules()
    console.log('[Dashboard] Loaded modules:', availableModules.value.map(m => m.key).join(', '))
  } catch (error) {
    console.error('[Dashboard] Failed to load modules:', error)
  } finally {
    loading.value = false
  }
})

function goToModule(moduleName: string) {
  router.push(`/${moduleName}`)
}

function getModuleIcon(icon: string): string {
  // Map icon names to SVG or emoji
  const iconMap: Record<string, string> = {
    'mail': '@',
    'message-square': '💬',
    'settings': '⚙️',
    'calendar': '📅',
    'check-square': '✅',
    'users': '👥',
    'file-text': '📄',
  }
  return iconMap[icon] || '📦'
}
</script>
