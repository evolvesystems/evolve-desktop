<template>
  <SetupWizard v-if="!setupCompleted" />
  <component v-else :is="layout">
    <router-view />
  </component>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import MainLayout from '@/layouts/MainLayout.vue'
import SetupWizard from '@/components/SetupWizard.vue'
import axios from 'axios'

const route = useRoute()
const setupCompleted = ref(false)

onMounted(async () => {
  console.log('[App.vue] Checking setup status...')

  // Check if setup has been completed
  const completed = localStorage.getItem('setup_completed')
  let apiUrl = localStorage.getItem('api_url')

  console.log('[App.vue] Setup completed:', completed, 'API URL:', apiUrl)

  // If no setup completed, try production server first
  if (completed !== 'true' || !apiUrl) {
    const productionUrl = 'https://evolvepreneuriq.app'

    console.log('[App.vue] No setup found, testing production server:', productionUrl)

    try {
      // Test production server
      const response = await axios.get(`${productionUrl}/api/health`, {
        timeout: 5000
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

const layout = computed(() => {
  // Use different layouts based on route meta
  if (route.meta.layout === 'auth') {
    // For auth pages, just render the content without layout
    return 'div'
  }

  return MainLayout
})
</script>

<style>
/* Global styles are in src/style.css */
</style>
