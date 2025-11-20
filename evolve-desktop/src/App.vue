<template>
  <SetupWizard v-if="!setupCompleted" />
  <AppLayout v-else-if="route.meta.layout !== 'auth'">
    <router-view />
  </AppLayout>
  <router-view v-else />

  <!-- Session Expired Modal -->
  <div v-if="showSessionExpired" class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Session Expired</h3>
      <p class="py-4">Your session has expired. Please log in again to continue.</p>
      <div class="modal-action">
        <button @click="handleRelogin" class="btn btn-primary">Log In Again</button>
      </div>
    </div>
  </div>

  <!-- Update Available Modal -->
  <div v-if="updateAvailable" class="modal modal-open">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Update Available</h3>
      <p class="py-4">
        Version {{ updateInfo?.version }} is available. Would you like to download and install it now?
      </p>
      <div v-if="updateInfo?.body" class="text-sm bg-base-200 p-3 rounded mb-4 max-h-48 overflow-y-auto">
        <p class="font-semibold mb-2">Release Notes:</p>
        <div v-html="updateInfo.body"></div>
      </div>
      <div v-if="downloadProgress" class="mb-4">
        <div class="text-sm mb-2">{{ downloadStatus }}</div>
        <progress class="progress progress-primary w-full" :value="downloadProgress" max="100"></progress>
      </div>
      <div class="modal-action">
        <button v-if="!isDownloading" @click="skipUpdate" class="btn btn-ghost">Later</button>
        <button v-if="!isDownloading" @click="installUpdate" class="btn btn-primary">Install Update</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AppLayout from '@/layouts/AppLayout.vue'
import SetupWizard from '@/components/SetupWizard.vue'
import axios from 'axios'
import { eventBus, EVENTS } from '@/core/event-bus'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

const route = useRoute()
const router = useRouter()
const setupCompleted = ref(false)
const showSessionExpired = ref(false)
const updateAvailable = ref(false)
const updateInfo = ref<any>(null)
const isDownloading = ref(false)
const downloadProgress = ref(0)
const downloadStatus = ref('')

// Listen for logout events
eventBus.on(EVENTS.USER_LOGGED_OUT, () => {
  showSessionExpired.value = true
})

function handleRelogin() {
  showSessionExpired.value = false
  // Clear any stale data
  localStorage.removeItem('user')
  localStorage.removeItem('tokens')
  // Force hard redirect to login
  window.location.href = '/login'
}

onMounted(async () => {
  console.log('[App.vue] Checking setup status...')

  // Check if setup has been completed
  const completed = localStorage.getItem('setup_completed')
  let apiUrl = localStorage.getItem('api_url')

  console.log('[App.vue] Setup completed:', completed, 'API URL:', apiUrl)

  // If no setup completed, try production server first
  if (completed !== 'true' || !apiUrl) {
    const productionUrl = import.meta.env.VITE_PRODUCTION_URL || 'https://evolvepreneuriq.app'

    console.log('[App.vue] No setup found, testing production server:', productionUrl)

    try {
      // Test production server - use Tauri HTTP in desktop, axios in browser
      let response, data

      // Check if running in Tauri
      if (window.__TAURI__) {
        // Use Tauri HTTP plugin (bypasses CORS in desktop app)
        const { fetch: tauriFetch } = await import('@tauri-apps/plugin-http')
        response = await tauriFetch(`${productionUrl}/api/health`, {
          method: 'GET',
          connectTimeout: 5000
        })

        console.log('[App.vue] Production server response (Tauri):', response.status)

        if (response.ok) {
          data = await response.json()
        }
      } else {
        // Use axios in browser (for dev mode)
        response = await axios.get(`${productionUrl}/api/health`, {
          timeout: 5000
        })

        console.log('[App.vue] Production server response (Browser):', response.status)

        if (response.status === 200) {
          data = response.data
        }
      }

      if (data) {
        console.log('[App.vue] Production server data:', data)

        // Auto-configure with production server
        localStorage.setItem('api_url', productionUrl)
        localStorage.setItem('server_name', data?.name || 'EvolveApp Production Server')
        localStorage.setItem('setup_completed', 'true')

        // Update axios base URL
        axios.defaults.baseURL = productionUrl

        console.log('[App.vue] Auto-configured with production server!')

        setupCompleted.value = true
        return
      }
    } catch (error: any) {
      console.error('[App.vue] Failed to connect to production server:', error.message)
      // Production server not reachable, show setup wizard
    }
  }

  setupCompleted.value = completed === 'true' && !!apiUrl
  console.log('[App.vue] Final setup status:', setupCompleted.value)

  // Check for updates after setup is complete
  if (setupCompleted.value) {
    checkForUpdates()
  }
})

async function checkForUpdates() {
  try {
    console.log('[App.vue] Checking for updates...')
    const update = await check()

    if (update) {
      console.log(`[App.vue] Update available: ${update.version} from ${update.date}`)
      updateAvailable.value = true
      updateInfo.value = {
        version: update.version,
        date: update.date,
        body: update.body
      }

      // Store update object for installation
      ;(window as any).__pendingUpdate = update
    } else {
      console.log('[App.vue] No updates available')
    }
  } catch (error: any) {
    console.error('[App.vue] Failed to check for updates:', error.message)
  }
}

async function installUpdate() {
  try {
    const update = (window as any).__pendingUpdate
    if (!update) return

    isDownloading.value = true
    downloadStatus.value = 'Downloading update...'

    await update.downloadAndInstall((event: any) => {
      switch (event.event) {
        case 'Started':
          downloadStatus.value = `Downloading ${Math.round(event.data.contentLength / 1024 / 1024)}MB...`
          downloadProgress.value = 0
          break
        case 'Progress':
          const downloaded = event.data.chunkLength
          const total = event.data.contentLength || 1
          downloadProgress.value = Math.round((downloaded / total) * 100)
          break
        case 'Finished':
          downloadStatus.value = 'Installing update...'
          downloadProgress.value = 100
          break
      }
    })

    console.log('[App.vue] Update installed, relaunching...')
    await relaunch()
  } catch (error: any) {
    console.error('[App.vue] Failed to install update:', error.message)
    downloadStatus.value = `Error: ${error.message}`
    isDownloading.value = false
  }
}

function skipUpdate() {
  updateAvailable.value = false
  delete (window as any).__pendingUpdate
}
})
</script>

<style>
/* Global styles are in src/style.css */
</style>
