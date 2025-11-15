<template>
  <dialog :class="['modal', { 'modal-open': appStore.searchOpen }]">
    <div class="modal-box w-11/12 max-w-2xl p-0">
      <!-- Search Input -->
      <div class="p-4 border-b border-base-300">
        <div class="relative">
          <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-base-content/50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <input
            ref="searchInput"
            v-model="query"
            type="text"
            placeholder="Search across all modules..."
            class="input input-bordered w-full pl-10 pr-4"
            @input="handleSearch"
            @keydown.esc="appStore.closeSearch"
          />
        </div>
      </div>

      <!-- Search Results -->
      <div class="max-h-96 overflow-auto p-2">
        <div v-if="loading" class="flex items-center justify-center py-12">
          <span class="loading loading-spinner loading-lg"></span>
        </div>

        <div v-else-if="query && results.length === 0" class="text-center py-12 text-base-content/50">
          <svg class="w-12 h-12 mx-auto mb-2 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M12 12h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <p class="text-sm">No results found</p>
        </div>

        <div v-else-if="!query" class="text-center py-12 text-base-content/50">
          <svg class="w-12 h-12 mx-auto mb-2 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
          <p class="text-sm">Start typing to search</p>
        </div>

        <div v-else class="space-y-1">
          <div
            v-for="result in results"
            :key="`${result.moduleId}-${result.type}-${result.title}`"
            @click="handleResultClick(result)"
            class="p-3 rounded-lg hover:bg-base-200 cursor-pointer transition-colors"
          >
            <div class="flex items-start gap-3">
              <div class="flex-shrink-0 mt-1">
                <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center">
                  <span class="text-xs font-medium text-primary">{{ getModuleName(result.moduleId).charAt(0) }}</span>
                </div>
              </div>

              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-1">
                  <span class="badge badge-sm">{{ getModuleName(result.moduleId) }}</span>
                  <span class="badge badge-sm badge-outline">{{ result.type }}</span>
                </div>
                <h4 class="font-medium text-sm">{{ result.title }}</h4>
                <p v-if="result.subtitle" class="text-xs text-base-content/70 mt-1">{{ result.subtitle }}</p>
              </div>

              <div v-if="result.score" class="flex-shrink-0">
                <div class="badge badge-ghost badge-sm">{{ Math.round(result.score) }}%</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="p-3 border-t border-base-300 bg-base-200 flex items-center justify-between text-xs text-base-content/50">
        <div class="flex items-center gap-4">
          <span class="flex items-center gap-1">
            <kbd class="kbd kbd-xs">↑</kbd>
            <kbd class="kbd kbd-xs">↓</kbd>
            to navigate
          </span>
          <span class="flex items-center gap-1">
            <kbd class="kbd kbd-xs">↵</kbd>
            to select
          </span>
        </div>
        <span class="flex items-center gap-1">
          <kbd class="kbd kbd-xs">esc</kbd>
          to close
        </span>
      </div>
    </div>

    <form method="dialog" class="modal-backdrop">
      <button @click="appStore.closeSearch">close</button>
    </form>
  </dialog>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useAppStore } from '@/stores/app'
import { moduleRegistry } from '@/core/plugin-system'
import type { SearchResult } from '@/core/types/module'

const router = useRouter()
const appStore = useAppStore()

const searchInput = ref<HTMLInputElement | null>(null)
const query = ref('')
const results = ref<SearchResult[]>([])
const loading = ref(false)
let searchTimeout: number | null = null

// Focus input when modal opens
watch(() => appStore.searchOpen, (isOpen) => {
  if (isOpen) {
    nextTick(() => {
      searchInput.value?.focus()
    })
  } else {
    query.value = ''
    results.value = []
  }
})

async function handleSearch() {
  // Clear previous timeout
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }

  const searchQuery = query.value.trim()

  if (!searchQuery) {
    results.value = []
    return
  }

  // Debounce search
  searchTimeout = window.setTimeout(async () => {
    loading.value = true

    try {
      const searchResults = await moduleRegistry.search(searchQuery)
      results.value = searchResults
    } catch (error) {
      console.error('Search error:', error)
      results.value = []
    } finally {
      loading.value = false
    }
  }, 300)
}

function getModuleName(moduleId: string): string {
  const module = moduleRegistry.getModule(moduleId)
  return module?.metadata.name || moduleId
}

function handleResultClick(result: SearchResult) {
  // Emit search event for modules to handle
  appStore.closeSearch()

  // You can implement custom navigation logic here
  // For now, just log the result
  console.log('Selected result:', result)

  // Navigate to module if needed
  // router.push(`/modules/${result.moduleId}`)
}

// Keyboard shortcut for opening search
document.addEventListener('keydown', (e) => {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault()
    appStore.toggleSearch()
  }
})
</script>
