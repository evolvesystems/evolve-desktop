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
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AppLayout from '@/layouts/AppLayout.vue'
import SetupWizard from '@/components/SetupWizard.vue'
import axios from 'axios'
import { eventBus, EVENTS } from '@/core/event-bus'

const route = useRoute()
const router = useRouter()
const setupCompleted = ref(false)
const showSessionExpired = ref(false)

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
      // Test production server
      const response = await axios.get(`${productionUrl}/api/health`, {
        timeout: 10000
      })

      console.log('[App.vue] Production server response:', response.status, response.data)

      if (response.status === 200) {
        // Auto-configure with production server
        localStorage.setItem('api_url', productionUrl)
        localStorage.setItem('server_name', response.data?.name || 'EvolveApp Production Server')
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
})
</script>

<style>
/* Global styles are in src/style.css */
</style>
