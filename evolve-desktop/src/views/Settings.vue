<template>
  <div class="p-6">
    <div class="mb-6">
      <h1 class="text-3xl font-bold">Settings</h1>
      <p class="text-base-content/70 mt-1">Manage your application preferences</p>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
      <!-- Settings Menu -->
      <div class="lg:col-span-1">
        <div class="menu bg-base-200 rounded-box">
          <li :class="{ 'active': activeTab === 'general' }">
            <a @click="activeTab = 'general'">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              General
            </a>
          </li>
          <li :class="{ 'active': activeTab === 'appearance' }">
            <a @click="activeTab = 'appearance'">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
              </svg>
              Appearance
            </a>
          </li>
          <li :class="{ 'active': activeTab === 'modules' }">
            <a @click="activeTab = 'modules'">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
              </svg>
              Modules
            </a>
          </li>
          <li :class="{ 'active': activeTab === 'account' }">
            <a @click="activeTab = 'account'">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
              </svg>
              Account
            </a>
          </li>
        </div>
      </div>

      <!-- Settings Content -->
      <div class="lg:col-span-3">
        <!-- General Settings -->
        <div v-if="activeTab === 'general'" class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">General Settings</h2>

            <!-- Server Connection -->
            <div class="mb-6 p-4 bg-base-200 rounded-lg">
              <h3 class="font-semibold mb-2">Server Connection</h3>
              <p class="text-sm text-base-content/70 mb-3">
                Current server: <span class="font-mono text-primary">{{ currentServer }}</span>
              </p>
              <button @click="changeServer" class="btn btn-sm btn-outline">
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
                Change Server
              </button>
            </div>

            <div class="divider"></div>

            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text">Start on system boot</span>
                <input type="checkbox" class="toggle toggle-primary" />
              </label>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text">Minimize to system tray</span>
                <input type="checkbox" class="toggle toggle-primary" checked />
              </label>
            </div>

            <div class="form-control">
              <label class="label cursor-pointer">
                <span class="label-text">Show desktop notifications</span>
                <input type="checkbox" class="toggle toggle-primary" checked />
              </label>
            </div>
          </div>
        </div>

        <!-- Appearance Settings -->
        <div v-if="activeTab === 'appearance'" class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">Appearance</h2>

            <div class="form-control">
              <label class="label">
                <span class="label-text">Theme</span>
              </label>
              <select
                v-model="appStore.theme"
                @change="appStore.setTheme(appStore.theme)"
                class="select select-bordered"
              >
                <option value="light">Light</option>
                <option value="dark">Dark</option>
                <option value="auto">Auto (System)</option>
              </select>
            </div>

            <div class="form-control mt-4">
              <label class="label">
                <span class="label-text">Font Size</span>
              </label>
              <input type="range" min="12" max="18" value="14" class="range range-primary" step="1" />
              <div class="w-full flex justify-between text-xs px-2">
                <span>Small</span>
                <span>Medium</span>
                <span>Large</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Modules Settings -->
        <div v-if="activeTab === 'modules'" class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">Modules</h2>
            <p class="text-sm text-base-content/70 mb-4">Manage installed modules and their settings</p>

            <div v-if="moduleInstallations.length === 0" class="text-center py-12 text-base-content/50">
              <p>No modules installed</p>
            </div>

            <div v-else class="space-y-2">
              <div
                v-for="installation in moduleInstallations"
                :key="installation.moduleId"
                class="flex items-center justify-between p-4 rounded-lg bg-base-200"
              >
                <div class="flex items-center gap-3">
                  <div class="text-2xl">{{ getModule(installation.moduleId)?.metadata.icon }}</div>
                  <div>
                    <h3 class="font-semibold">{{ getModule(installation.moduleId)?.metadata.name }}</h3>
                    <p class="text-xs text-base-content/70">v{{ installation.version }}</p>
                  </div>
                </div>

                <div class="flex items-center gap-2">
                  <input
                    type="checkbox"
                    :checked="installation.enabled"
                    @change="toggleModule(installation.moduleId)"
                    class="toggle toggle-primary"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Account Settings -->
        <div v-if="activeTab === 'account'" class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">Account</h2>

            <div class="form-control">
              <label class="label">
                <span class="label-text">Name</span>
              </label>
              <input
                type="text"
                :value="authStore.user?.name"
                class="input input-bordered"
                placeholder="Your name"
              />
            </div>

            <div class="form-control mt-4">
              <label class="label">
                <span class="label-text">Email</span>
              </label>
              <input
                type="email"
                :value="authStore.user?.email"
                class="input input-bordered"
                placeholder="your@email.com"
                disabled
              />
            </div>

            <div class="divider"></div>

            <button class="btn btn-error btn-outline">Delete Account</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAppStore } from '@/stores/app'
import { useAuthStore } from '@/stores/auth'
import { moduleRegistry } from '@/core/plugin-system'

const appStore = useAppStore()
const authStore = useAuthStore()

const activeTab = ref('general')

const currentServer = computed(() => localStorage.getItem('api_url') || 'Not connected')

function changeServer() {
  if (confirm('This will log you out and reset the server connection. Continue?')) {
    localStorage.removeItem('setup_completed')
    localStorage.removeItem('api_url')
    localStorage.removeItem('server_name')
    localStorage.removeItem('user')
    localStorage.removeItem('tokens')
    window.location.reload()
  }
}

const moduleInstallations = computed(() => moduleRegistry.getAllInstallations())

function getModule(moduleId: string) {
  return moduleRegistry.getModule(moduleId)
}

async function toggleModule(moduleId: string) {
  const installation = moduleRegistry.getInstallation(moduleId)
  if (!installation) return

  try {
    if (installation.enabled) {
      await moduleRegistry.disable(moduleId)
    } else {
      await moduleRegistry.enable(moduleId)
    }
  } catch (error: any) {
    appStore.showToast('error', error.message)
  }
}
</script>
