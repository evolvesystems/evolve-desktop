<template>
  <div class="w-64 bg-base-200 border-r border-base-300 flex flex-col">
    <!-- Folder List -->
    <div class="flex-1 overflow-y-auto py-4">
      <div class="menu">
        <li v-for="folder in folders" :key="folder.id">
          <a
            :class="{ 'active': currentFolder?.id === folder.id }"
            @click="$emit('select-folder', folder)"
          >
            <span v-html="getFolderIcon(folder.type)" class="flex-shrink-0"></span>
            <span class="flex-1">{{ folder.name }}</span>
            <span v-if="folder.unreadCount > 0" class="badge badge-primary badge-sm">
              {{ folder.unreadCount }}
            </span>
          </a>
        </li>
      </div>

      <!-- Special Filters -->
      <div class="divider px-4">Filters</div>
      <div class="menu">
        <li>
          <a @click="$emit('filter', 'unread')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
            <span class="flex-1">Unread</span>
            <span v-if="unreadCount > 0" class="badge badge-sm">{{ unreadCount }}</span>
          </a>
        </li>
        <li>
          <a @click="$emit('filter', 'flagged')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 21v-4m0 0V5a2 2 0 012-2h6.5l1 1H21l-3 6 3 6h-8.5l-1-1H5a2 2 0 00-2 2zm9-13.5V9" />
            </svg>
            <span class="flex-1">Flagged</span>
            <span v-if="flaggedCount > 0" class="badge badge-sm">{{ flaggedCount }}</span>
          </a>
        </li>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { EmailFolder } from '@/services/emailService'

defineProps<{
  folders: EmailFolder[]
  currentFolder: EmailFolder | null
  unreadCount: number
  flaggedCount: number
}>()

defineEmits<{
  'select-folder': [folder: EmailFolder]
  'filter': [filter: 'unread' | 'flagged']
}>()

function getFolderIcon(type: string): string {
  const icons: Record<string, string> = {
    inbox: `<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
    </svg>`,
    sent: `<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
    </svg>`,
    drafts: `<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
    </svg>`,
    trash: `<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
    </svg>`,
    spam: `<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
    </svg>`,
    custom: `<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
    </svg>`
  }

  return icons[type] || icons.custom
}
</script>
