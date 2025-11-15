<template>
  <component :is="layout">
    <router-view />
  </component>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import MainLayout from '@/layouts/MainLayout.vue'

const route = useRoute()

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
