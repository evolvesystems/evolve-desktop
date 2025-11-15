<template>
  <router-link
    :to="moduleRoute"
    v-slot="{ isActive }"
  >
    <button
      class="w-full h-12 flex items-center justify-center hover:bg-base-300 transition-colors relative group"
      :class="{ 'bg-base-300 border-l-4 border-primary': isActive }"
      :title="module.metadata.name"
    >
      <!-- Icon (you can use heroicons or module-specific icons) -->
      <span class="text-2xl" v-if="module.metadata.icon">{{ module.metadata.icon }}</span>
      <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
      </svg>

      <!-- Tooltip -->
      <div class="absolute left-full ml-2 px-2 py-1 bg-base-content text-base-100 text-sm rounded opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-50">
        {{ module.metadata.name }}
      </div>
    </button>
  </router-link>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ModulePlugin } from '@/core/types/module'

const props = defineProps<{
  module: ModulePlugin
}>()

const moduleRoute = computed(() => {
  const routes = props.module.getRoutes()
  return routes.length > 0 ? routes[0].path : '#'
})
</script>
