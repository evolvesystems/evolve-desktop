<template>
  <div class="w-96 bg-base-100 border-r border-base-300 flex flex-col">
    <!-- Email List Header -->
    <div class="p-4 border-b border-base-300">
      <div class="flex items-center justify-between">
        <div class="text-sm font-semibold">
          {{ totalEmails }} emails
        </div>
        <div class="text-sm text-base-content/70">
          Page {{ currentPage }} of {{ totalPages }}
        </div>
      </div>
    </div>

    <!-- Email List -->
    <div class="flex-1 overflow-y-auto">
      <div v-if="isLoading" class="flex items-center justify-center h-full">
        <span class="loading loading-spinner loading-lg"></span>
      </div>

      <div v-else-if="emails.length === 0" class="flex items-center justify-center h-full text-base-content/50">
        <div class="text-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto mb-4 opacity-30" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
          </svg>
          <p>No emails found</p>
        </div>
      </div>

      <div v-else class="divide-y divide-base-300">
        <div
          v-for="email in emails"
          :key="email.id"
          class="p-4 cursor-pointer hover:bg-base-200 transition-colors"
          :class="{
            'bg-base-200': selectedEmail?.id === email.id,
            'font-semibold': !email.isRead
          }"
          @click="$emit('select-email', email)"
        >
          <div class="flex items-start gap-3">
            <!-- Avatar -->
            <div class="avatar placeholder">
              <div class="bg-neutral text-neutral-content rounded-full w-10 h-10">
                <span class="text-sm">{{ getInitials(email.fromName) }}</span>
              </div>
            </div>

            <!-- Email Content -->
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between gap-2 mb-1">
                <div class="font-medium truncate">{{ email.fromName }}</div>
                <div class="text-xs text-base-content/70 whitespace-nowrap">
                  {{ formatDate(email.receivedDate) }}
                </div>
              </div>
              <div class="text-sm truncate mb-1" :class="{ 'font-semibold': !email.isRead }">
                {{ email.subject }}
              </div>
              <div class="text-xs text-base-content/70 truncate">
                {{ email.snippet }}
              </div>

              <!-- Badges -->
              <div class="flex items-center gap-2 mt-2">
                <span v-if="email.hasAttachments" class="badge badge-xs">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                  </svg>
                </span>
                <button
                  v-if="email.isFlagged"
                  class="text-warning"
                  @click.stop="$emit('toggle-flag', email.id)"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M3 6a3 3 0 013-3h10a1 1 0 01.8 1.6L14.25 8l2.55 3.4A1 1 0 0116 13H6a1 1 0 00-1 1v3a1 1 0 11-2 0V6z" />
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <!-- Quick Actions -->
          <div class="flex gap-1 mt-2 opacity-0 group-hover:opacity-100 transition-opacity">
            <button
              class="btn btn-xs btn-ghost"
              @click.stop="$emit('toggle-flag', email.id)"
              :title="email.isFlagged ? 'Unflag' : 'Flag'"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 21v-4m0 0V5a2 2 0 012-2h6.5l1 1H21l-3 6 3 6h-8.5l-1-1H5a2 2 0 00-2 2zm9-13.5V9" />
              </svg>
            </button>
            <button
              class="btn btn-xs btn-ghost text-error"
              @click.stop="$emit('delete-email', email.id)"
              title="Delete"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="p-4 border-t border-base-300">
      <div class="flex justify-between items-center">
        <button
          class="btn btn-sm btn-ghost"
          :disabled="currentPage === 1"
          @click="$emit('prev-page')"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
          </svg>
          Previous
        </button>
        <div class="text-sm">
          {{ currentPage }} / {{ totalPages }}
        </div>
        <button
          class="btn btn-sm btn-ghost"
          :disabled="currentPage === totalPages"
          @click="$emit('next-page')"
        >
          Next
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { EmailMessage } from '@/services/emailService'

defineProps<{
  emails: EmailMessage[]
  selectedEmail: EmailMessage | null
  isLoading: boolean
  currentPage: number
  totalPages: number
  totalEmails: number
}>()

defineEmits<{
  'select-email': [email: EmailMessage]
  'next-page': []
  'prev-page': []
  'toggle-flag': [emailId: number]
  'delete-email': [emailId: number]
}>()

function getInitials(name: string): string {
  return name
    .split(' ')
    .map(n => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)
}

function formatDate(dateString: string): string {
  const date = new Date(dateString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))

  if (diffDays === 0) {
    return date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })
  } else if (diffDays === 1) {
    return 'Yesterday'
  } else if (diffDays < 7) {
    return date.toLocaleDateString('en-US', { weekday: 'short' })
  } else {
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
  }
}
</script>
