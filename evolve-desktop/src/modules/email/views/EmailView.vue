<template>
  <!-- EXACT CLONE OF EIQ EMAIL MANAGER 3-COLUMN LAYOUT -->
  <div class="h-full flex flex-col bg-base-100">

    <!-- TOOLBAR - Action buttons like EIQ -->
    <div class="sticky top-0 z-30 bg-base-100 border-b border-base-300 px-4 py-3 flex items-center justify-between gap-4 shadow-sm">
      <!-- Left: Action buttons -->
      <div class="flex items-center gap-2 overflow-x-auto">
        <!-- New Email - Primary button -->
        <button type="button" class="btn btn-sm btn-primary gap-1 whitespace-nowrap">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
          </svg>
          <span class="hidden sm:inline">New Email</span>
        </button>

        <!-- Delete -->
        <button class="btn btn-sm btn-ghost gap-1" title="Delete">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
          <span class="hidden lg:inline">Delete</span>
        </button>

        <!-- Archive -->
        <button class="btn btn-sm btn-ghost gap-1" title="Archive">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
          </svg>
          <span class="hidden lg:inline">Archive</span>
        </button>

        <!-- Reply -->
        <button class="btn btn-sm btn-ghost gap-1" title="Reply (R)">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
          </svg>
          <span class="hidden lg:inline">Reply</span>
        </button>

        <!-- Forward -->
        <button class="btn btn-sm btn-ghost gap-1" title="Forward (F)">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
          </svg>
          <span class="hidden lg:inline">Forward</span>
        </button>

        <!-- Mark as Read/Unread -->
        <button class="btn btn-sm btn-ghost gap-1" title="Mark as Read">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
          <span class="hidden lg:inline">Mark Read</span>
        </button>

        <!-- Flag/Star -->
        <button class="btn btn-sm btn-ghost gap-1" title="Flag message">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" />
          </svg>
          <span class="hidden lg:inline">Flag</span>
        </button>
      </div>

      <!-- Right: Search box -->
      <div class="relative">
        <input type="text" placeholder="Search..." class="input input-sm input-bordered w-48 lg:w-64">
        <button class="btn btn-sm btn-ghost btn-square absolute right-0 top-0">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </button>
      </div>
    </div>

    <!-- THREE-COLUMN LAYOUT - Exactly like EIQ -->
    <div class="flex-1 flex overflow-hidden">

      <!-- LEFT PANEL: Accounts & Folders Sidebar (256px) -->
      <div class="bg-base-100 border-r border-base-300 overflow-y-auto flex-shrink-0" style="width: 256px;">
        <div class="p-2">
          <!-- Add Account Button -->
          <div class="px-2 py-2">
            <button class="btn btn-sm btn-ghost btn-block justify-start gap-2">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              Add Account
            </button>
          </div>

          <div class="divider my-1"></div>

          <!-- Real Email Accounts from API -->
          <div v-for="account in emailAccounts" :key="account.id" class="mb-2">
            <!-- Account Header - Collapsible -->
            <div class="collapse collapse-arrow collapse-open bg-base-200/50 rounded-lg">
              <input type="checkbox" checked />
              <div class="collapse-title py-2 px-3 pr-10 min-h-0">
                <div class="flex items-center gap-2">
                  <div class="w-8 h-8 rounded-full bg-primary text-primary-content flex items-center justify-center text-xs font-bold shrink-0">
                    {{ getInitials(account.name || account.email) }}
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="text-xs font-semibold truncate">{{ account.name }}</div>
                    <div class="text-[10px] text-base-content/60 truncate">{{ account.email }}</div>
                  </div>
                </div>
              </div>
              <div class="collapse-content px-2 py-0">
                <!-- Folders -->
                <ul class="menu menu-xs p-0">
                  <li v-for="folder in emailFolders" :key="folder.id">
                    <a :class="{ 'active': folder.type === 'inbox' }">
                      <span class="text-xs">{{ folder.name }}</span>
                      <span v-if="folder.unread_count > 0" class="badge badge-xs badge-primary">{{ folder.unread_count }}</span>
                    </a>
                  </li>
                </ul>
              </div>
            </div>
          </div>

          <!-- Empty state if no accounts -->
          <div v-if="!loading && emailAccounts.length === 0" class="px-4 py-6 text-center text-sm text-base-content/60">
            No email accounts configured
          </div>
        </div>
      </div>

      <!-- CENTER PANEL: Message List -->
      <div class="flex-1 bg-base-100 border-r border-base-300 overflow-hidden flex flex-col min-w-0">
        <!-- Message list header -->
        <div class="px-4 py-3 border-b border-base-300 flex items-center gap-2 shrink-0">
          <h2 class="font-semibold text-base">INBOX</h2>
          <span class="text-sm text-base-content/60">{{ messages.length }} messages</span>
        </div>

        <!-- Message list -->
        <div class="flex-1 overflow-y-auto">
          <!-- Loading state -->
          <div v-if="loading" class="flex items-center justify-center py-12">
            <span class="loading loading-spinner loading-lg text-primary"></span>
          </div>

          <!-- Error state -->
          <div v-else-if="errorMessage" class="flex flex-col items-center justify-center py-12 px-6">
            <div class="alert alert-error max-w-md">
              <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <div>
                <h3 class="font-bold">Failed to load emails</h3>
                <div class="text-xs">{{ errorMessage }}</div>
              </div>
            </div>
          </div>

          <!-- Empty state -->
          <div v-else-if="messages.length === 0" class="flex flex-col items-center justify-center py-12 text-base-content/40">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 mb-4 opacity-30" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
            <p class="text-lg">No messages found</p>
          </div>

          <!-- Messages -->
          <div v-else>
            <!-- Select All Header -->
            <div class="sticky top-0 bg-base-100 border-b border-base-300 px-4 py-2 flex items-center gap-2 z-10">
              <input type="checkbox" class="checkbox checkbox-sm" title="Select all messages">
              <span class="text-xs text-base-content/60">Select all</span>
            </div>

            <div class="divide-y divide-base-300">
              <div
                v-for="message in messages"
                :key="message.id"
                @click="selectMessage(message)"
                :class="[
                  'group relative flex items-start gap-2 px-4 py-4 hover:bg-base-200 transition-colors cursor-pointer',
                  { 'font-semibold': !message.isRead }
                ]"
              >
                <!-- Checkbox -->
                <div class="relative z-20 shrink-0" @click.stop>
                  <input type="checkbox" class="checkbox checkbox-sm">
                </div>

                <!-- Avatar -->
                <div class="w-10 h-10 rounded-full bg-primary text-primary-content flex items-center justify-center text-sm font-bold shrink-0">
                  {{ getInitials(message.fromName || message.fromAddress) }}
                </div>

                <!-- Message content -->
                <div class="flex-1 min-w-0">
                  <!-- From name and date/time -->
                  <div class="flex items-start justify-between gap-2 mb-1">
                    <span class="text-sm truncate">{{ message.fromName || message.fromAddress }}</span>
                    <div class="text-xs text-base-content/60 shrink-0 text-right">
                      <div>{{ formatDate(message.receivedDate) }}</div>
                      <div class="text-[10px]">{{ formatTime(message.receivedDate) }}</div>
                    </div>
                  </div>

                  <!-- Subject with attachment indicator -->
                  <div class="text-sm truncate mb-1 flex items-center gap-1">
                    <svg v-if="message.hasAttachments" xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 text-base-content/60" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                    </svg>
                    {{ message.subject || '(No subject)' }}
                  </div>

                  <!-- Preview -->
                  <div class="text-xs text-base-content/60 truncate hidden sm:block">
                    {{ message.snippet || '' }}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- RIGHT PANEL: Reading Pane -->
      <div class="flex-1 bg-base-100 overflow-hidden flex flex-col min-w-0">
        <!-- Empty state - select a message -->
        <div v-if="!selectedMessage" class="flex-1 flex items-center justify-center text-base-content/60">
          <div class="text-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-24 h-24 mx-auto mb-4 opacity-20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
            <p class="text-lg">Select a message to read</p>
          </div>
        </div>

        <!-- Message viewer -->
        <div v-else class="flex-1 flex flex-col overflow-hidden">
          <!-- Message header -->
          <div class="border-b border-base-300 p-6 shrink-0">
            <!-- Subject -->
            <h2 class="text-2xl font-semibold mb-4">{{ selectedMessage.subject || '(No subject)' }}</h2>

            <!-- From/To/Date info -->
            <div class="space-y-2 text-sm">
              <div class="flex items-center gap-3">
                <div class="w-10 h-10 rounded-full bg-primary text-primary-content flex items-center justify-center text-sm font-bold shrink-0">
                  {{ getInitials(selectedMessage.fromName || selectedMessage.fromAddress) }}
                </div>
                <div class="flex-1">
                  <div class="font-semibold">{{ selectedMessage.fromName || selectedMessage.fromAddress }}</div>
                  <div class="text-xs text-base-content/60">{{ selectedMessage.fromAddress }}</div>
                </div>
                <div class="text-xs text-base-content/60 text-right">
                  {{ formatDate(selectedMessage.receivedDate) }} at {{ formatTime(selectedMessage.receivedDate) }}
                </div>
              </div>

              <div class="flex items-start gap-2 text-xs text-base-content/60">
                <span class="font-semibold">To:</span>
                <span>{{ selectedMessage.toAddresses?.join(', ') || 'me' }}</span>
              </div>
            </div>
          </div>

          <!-- Message body -->
          <div class="flex-1 overflow-y-auto p-6">
            <div v-if="selectedMessage.bodyHtml" v-html="selectedMessage.bodyHtml" class="prose max-w-none"></div>
            <div v-else-if="selectedMessage.body" class="whitespace-pre-wrap">{{ selectedMessage.body }}</div>
            <div v-else class="text-base-content/40 italic">No message content</div>
          </div>

          <!-- Attachments (if any) -->
          <div v-if="selectedMessage.hasAttachments && selectedMessage.attachments?.length" class="border-t border-base-300 p-4 shrink-0">
            <div class="text-sm font-semibold mb-2">Attachments ({{ selectedMessage.attachments.length }})</div>
            <div class="flex flex-wrap gap-2">
              <div
                v-for="attachment in selectedMessage.attachments"
                :key="attachment.id"
                class="flex items-center gap-2 px-3 py-2 bg-base-200 rounded-lg hover:bg-base-300 cursor-pointer transition-colors"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-base-content/60" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                </svg>
                <span class="text-xs font-medium">{{ attachment.filename }}</span>
                <span class="text-[10px] text-base-content/60">({{ formatFileSize(attachment.size) }})</span>
              </div>
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { emailService, type EmailMessage } from '@/services/emailService'
import axios from 'axios'

// Real data from API
const loading = ref(true)
const messages = ref<EmailMessage[]>([])
const selectedMessage = ref<EmailMessage | null>(null)
const currentFolderId = ref<number | null>(null)
const errorMessage = ref<string | null>(null)
const emailAccounts = ref<any[]>([])
const emailFolders = ref<any[]>([])

onMounted(async () => {
  console.log('[EmailView] Loading real emails from API...')
  console.log('[EmailView] Current axios baseURL:', axios.defaults.baseURL)
  console.log('[EmailView] Auth header:', axios.defaults.headers.common['Authorization'])

  try {
    // Fetch email accounts, folders, and messages in parallel
    const [emailsResponse, accountsResponse, foldersResponse] = await Promise.all([
      emailService.getEmails({ limit: 50 }),
      emailService.getAccounts(),
      emailService.getFolders()
    ])

    messages.value = emailsResponse.data || []
    emailAccounts.value = accountsResponse
    emailFolders.value = foldersResponse

    console.log('[EmailView] Loaded', messages.value.length, 'real emails')
    console.log('[EmailView] Loaded', emailAccounts.value.length, 'email accounts')
    console.log('[EmailView] Loaded', emailFolders.value.length, 'folders')
  } catch (error: any) {
    console.error('[EmailView] Failed to load emails:', error)
    console.error('[EmailView] Error response:', error.response)
    console.error('[EmailView] Error status:', error.response?.status)
    console.error('[EmailView] Error data:', error.response?.data)

    if (error.response?.status === 401) {
      errorMessage.value = 'Authentication required. Please log in again.'
    } else if (error.response?.status === 404) {
      errorMessage.value = 'Email API endpoint not found. Please check EIQ Manager configuration.'
    } else if (error.response?.status === 500) {
      errorMessage.value = 'Server error loading emails. This may mean you have no email accounts configured in EIQ Manager, or there\'s a database issue. Try accessing the email manager in the web app first to set up your email accounts.'
    } else if (error.message?.includes('Network Error')) {
      errorMessage.value = 'Cannot connect to EIQ Manager. Please check if the server is running.'
    } else {
      errorMessage.value = error.response?.data?.message || error.message || 'Failed to load emails. Please try again.'
    }
  } finally {
    loading.value = false
  }
})

function selectMessage(message: EmailMessage) {
  selectedMessage.value = message
  if (!message.isRead) {
    emailService.markAsRead(message.id)
    message.isRead = true
  }
}

function formatDate(dateString: string): string {
  const date = new Date(dateString)
  const today = new Date()
  const isToday = date.toDateString() === today.toDateString()
  const yesterday = new Date(today)
  yesterday.setDate(yesterday.getDate() - 1)
  const isYesterday = date.toDateString() === yesterday.toDateString()

  if (isToday) return 'Today'
  if (isYesterday) return 'Yesterday'
  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
}

function formatTime(dateString: string): string {
  const date = new Date(dateString)
  return date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit', hour12: true })
}

function getInitials(name: string): string {
  if (!name) return '?'
  return name.charAt(0).toUpperCase()
}

function formatFileSize(bytes: number): string {
  if (!bytes) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}
</script>
