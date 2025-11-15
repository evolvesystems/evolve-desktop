<template>
  <div class="p-6">
    <div class="mb-6">
      <h1 class="text-3xl font-bold">Dashboard</h1>
      <p class="text-base-content/70 mt-1">Welcome back, {{ authStore.user?.name }}</p>
    </div>

    <!-- Quick Stats -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
      <div class="stats shadow">
        <div class="stat">
          <div class="stat-figure text-primary">
            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>
          <div class="stat-title">Unread Emails</div>
          <div class="stat-value text-primary">0</div>
        </div>
      </div>

      <div class="stats shadow">
        <div class="stat">
          <div class="stat-figure text-secondary">
            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
          </div>
          <div class="stat-title">Upcoming Events</div>
          <div class="stat-value text-secondary">0</div>
        </div>
      </div>

      <div class="stats shadow">
        <div class="stat">
          <div class="stat-figure text-accent">
            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
            </svg>
          </div>
          <div class="stat-title">Contacts</div>
          <div class="stat-value text-accent">0</div>
        </div>
      </div>

      <div class="stats shadow">
        <div class="stat">
          <div class="stat-figure text-info">
            <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
          </div>
          <div class="stat-title">Active Deals</div>
          <div class="stat-value text-info">0</div>
        </div>
      </div>
    </div>

    <!-- Installed Modules -->
    <div class="card bg-base-100 shadow-xl mb-6">
      <div class="card-body">
        <h2 class="card-title">Installed Modules</h2>

        <div v-if="installedModules.length === 0" class="text-center py-12 text-base-content/50">
          <svg class="w-16 h-16 mx-auto mb-4 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
          </svg>
          <p>No modules installed yet</p>
          <button class="btn btn-primary btn-sm mt-4" @click="() => router.push('/settings')">
            Browse Modules
          </button>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div
            v-for="module in installedModules"
            :key="module.metadata.id"
            class="card bg-base-200 cursor-pointer hover:bg-base-300 transition-colors"
            @click="navigateToModule(module)"
          >
            <div class="card-body">
              <div class="flex items-start gap-3">
                <div class="text-3xl">{{ module.metadata.icon }}</div>
                <div class="flex-1">
                  <h3 class="font-semibold">{{ module.metadata.name }}</h3>
                  <p class="text-xs text-base-content/70">{{ module.metadata.description }}</p>
                  <div class="mt-2">
                    <span class="badge badge-sm">{{ module.metadata.category }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Activity -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title">Recent Activity</h2>

        <div class="text-center py-12 text-base-content/50">
          <svg class="w-16 h-16 mx-auto mb-4 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <p>No recent activity</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { moduleRegistry } from '@/core/plugin-system'
import type { ModulePlugin } from '@/core/types/module'

const router = useRouter()
const authStore = useAuthStore()

const installedModules = computed(() => {
  return moduleRegistry
    .getAllInstallations()
    .filter((inst) => inst.enabled)
    .map((inst) => moduleRegistry.getModule(inst.moduleId))
    .filter((mod) => mod !== undefined) as ModulePlugin[]
})

function navigateToModule(module: ModulePlugin) {
  const routes = module.getRoutes()
  if (routes.length > 0) {
    router.push(routes[0].path)
  }
}
</script>
