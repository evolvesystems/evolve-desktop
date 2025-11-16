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

const route = useRoute()
const setupCompleted = ref(false)

onMounted(() => {
  // Check if setup has been completed
  const completed = localStorage.getItem('setup_completed')
  const apiUrl = localStorage.getItem('api_url')

  setupCompleted.value = completed === 'true' && !!apiUrl
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
