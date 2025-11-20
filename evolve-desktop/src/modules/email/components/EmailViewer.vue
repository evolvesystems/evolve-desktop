<template>
  <div class="flex-1 flex flex-col bg-base-100">
    <div v-if="isLoading" class="flex items-center justify-center h-full">
      <span class="loading loading-spinner loading-lg"></span>
    </div>

    <template v-else-if="email">
      <!-- Email Header -->
      <div class="p-6 border-b border-base-300">
        <div class="flex items-start justify-between mb-4">
          <h2 class="text-2xl font-bold flex-1 pr-4">{{ email.subject }}</h2>

          <!-- Actions -->
          <div class="flex gap-1">
            <button
              class="btn btn-sm btn-ghost"
              @click="$emit('reply', email)"
              title="Reply"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
              </svg>
            </button>
            <button
              class="btn btn-sm btn-ghost"
              @click="$emit('reply-all', email)"
              title="Reply All"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6m4 0l6 6-6 6" />
              </svg>
            </button>
            <button
              class="btn btn-sm btn-ghost"
              @click="$emit('forward', email)"
              title="Forward"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
              </svg>
            </button>
            <button
              class="btn btn-sm btn-ghost"
              :class="{ 'text-warning': email.isFlagged }"
              @click="$emit('toggle-flag', email.id)"
              title="Flag"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" :fill="email.isFlagged ? 'currentColor' : 'none'" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 21v-4m0 0V5a2 2 0 012-2h6.5l1 1H21l-3 6 3 6h-8.5l-1-1H5a2 2 0 00-2 2zm9-13.5V9" />
              </svg>
            </button>
            <button
              v-if="email.isRead"
              class="btn btn-sm btn-ghost"
              @click="$emit('mark-unread', email.id)"
              title="Mark as Unread"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
              </svg>
            </button>
            <button
              class="btn btn-sm btn-ghost text-error"
              @click="$emit('delete', email.id)"
              title="Delete"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>

        <!-- Sender Info -->
        <div class="flex items-center gap-3 mb-3">
          <div class="avatar placeholder">
            <div class="bg-neutral text-neutral-content rounded-full w-12 h-12">
              <span class="text-lg">{{ getInitials(email.fromName) }}</span>
            </div>
          </div>
          <div class="flex-1">
            <div class="font-semibold">{{ email.fromName }}</div>
            <div class="text-sm text-base-content/70">{{ email.fromAddress }}</div>
          </div>
          <div class="text-sm text-base-content/70">
            {{ formatFullDate(email.receivedDate) }}
          </div>
        </div>

        <!-- Recipients -->
        <div class="text-sm space-y-1">
          <div>
            <span class="text-base-content/70">To:</span>
            {{ email.toAddresses.join(', ') }}
          </div>
          <div v-if="email.ccAddresses && email.ccAddresses.length > 0">
            <span class="text-base-content/70">Cc:</span>
            {{ email.ccAddresses.join(', ') }}
          </div>
        </div>

        <!-- Attachments -->
        <div v-if="email.attachments && email.attachments.length > 0" class="mt-4">
          <div class="text-sm font-semibold mb-2">
            Attachments ({{ email.attachments.length }})
          </div>
          <div class="flex flex-wrap gap-2">
            <a
              v-for="attachment in email.attachments"
              :key="attachment.id"
              :href="attachment.downloadUrl"
              target="_blank"
              class="btn btn-sm btn-outline gap-2"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              {{ attachment.filename }}
              <span class="text-xs opacity-60">({{ formatFileSize(attachment.size) }})</span>
            </a>
          </div>
        </div>
      </div>

      <!-- Email Body -->
      <div class="flex-1 overflow-y-auto p-6">
        <div v-if="email.bodyHtml" v-html="email.bodyHtml" class="prose max-w-none"></div>
        <div v-else class="whitespace-pre-wrap">{{ email.body }}</div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { EmailMessage } from '@/services/emailService'

defineProps<{
  email: EmailMessage
  isLoading: boolean
}>()

defineEmits<{
  'reply': [email: EmailMessage]
  'reply-all': [email: EmailMessage]
  'forward': [email: EmailMessage]
  'delete': [emailId: number]
  'mark-unread': [emailId: number]
  'toggle-flag': [emailId: number]
}>()

function getInitials(name: string): string {
  return name
    .split(' ')
    .map(n => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)
}

function formatFullDate(dateString: string): string {
  const date = new Date(dateString)
  return date.toLocaleString('en-US', {
    weekday: 'short',
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit'
  })
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}
</script>

<style scoped>
.prose {
  color: inherit;
}

.prose :deep(a) {
  color: oklch(var(--p));
}

.prose :deep(img) {
  max-width: 100%;
  height: auto;
}
</style>
