<template>
  <div class="flex flex-col h-full bg-base-100">
    <!-- Workspace Bar (matching EIQ web) -->
    <div class="sticky top-0 z-40 bg-base-200 px-4 py-3 flex flex-nowrap items-center justify-between gap-4 shadow-sm">
      <!-- Left: Module branding + Breadcrumbs -->
      <div class="flex items-center gap-2 shrink-0 min-w-0">
        <div class="flex items-center gap-2 shrink-0">
          <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
          <h2 class="font-bold text-lg leading-tight">EvolveMail</h2>
        </div>
        <div class="text-sm breadcrumbs hidden sm:block">
          <ul>
            <li>
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
              </svg>
            </li>
            <li>{{ selectedFolder?.name || 'Inbox' }}</li>
          </ul>
        </div>
      </div>

      <!-- Right: Utility icons -->
      <div class="flex flex-nowrap items-center gap-1 shrink-0">
        <button class="btn btn-xs btn-ghost btn-square hidden sm:flex" title="Calendar">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
        </button>
        <button class="btn btn-xs btn-ghost btn-square hidden sm:flex" title="Tasks">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
        <button @click="emailStore.refresh" class="btn btn-xs btn-ghost btn-square" title="Refresh">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
        </button>
        <button class="btn btn-xs btn-ghost btn-square hidden sm:flex" title="Settings">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Toolbar (matching EIQ web) -->
    <div class="sticky top-[52px] z-30 bg-base-100 border-y border-base-300 px-2 sm:px-4 py-2 sm:py-3 flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-2 sm:gap-4 shadow-sm">
      <!-- Left: Action buttons -->
      <div class="flex items-center gap-1 sm:gap-2 overflow-x-auto">
        <!-- New Email - Primary button -->
        <button @click="emailStore.openComposer" class="btn btn-xs sm:btn-sm btn-primary gap-1 whitespace-nowrap">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
          </svg>
          <span class="hidden sm:inline">New Email</span>
        </button>

        <!-- Delete -->
        <button
          @click="deleteEmail"
          :disabled="!selectedEmail"
          class="btn btn-xs sm:btn-sm btn-ghost gap-1"
          title="Delete (Del/Backspace)"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
          <span class="hidden lg:inline">Delete</span>
        </button>

        <!-- Archive -->
        <button
          @click="archiveEmail"
          :disabled="!selectedEmail"
          class="btn btn-xs sm:btn-sm btn-ghost gap-1"
          title="Archive"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
          </svg>
          <span class="hidden lg:inline">Archive</span>
        </button>

        <!-- Move (with dropdown) -->
        <div class="dropdown">
          <button
            tabindex="0"
            :disabled="!selectedEmail"
            class="btn btn-xs sm:btn-sm btn-ghost gap-1"
            title="Move"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
            <span class="hidden lg:inline">Move</span>
            <svg class="w-3 h-3 hidden lg:inline" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>
          <ul tabindex="0" class="dropdown-content z-10 menu p-2 shadow bg-white rounded-box w-52 mt-2">
            <li v-for="folder in emailStore.folders" :key="folder.id">
              <a @click="moveToFolder(folder)">{{ folder.name }}</a>
            </li>
          </ul>
        </div>

        <!-- Reply -->
        <button
          @click="replyToEmail"
          :disabled="!selectedEmail"
          class="btn btn-xs sm:btn-sm btn-ghost gap-1"
          title="Reply (R)"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
          </svg>
          <span class="hidden lg:inline">Reply</span>
        </button>

        <!-- Reply All -->
        <button
          @click="replyAllToEmail"
          :disabled="!selectedEmail"
          class="btn btn-xs sm:btn-sm btn-ghost gap-1 hidden sm:flex"
          title="Reply All"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
          <span class="hidden lg:inline">Reply All</span>
        </button>

        <!-- Forward -->
        <button
          @click="forwardEmail"
          :disabled="!selectedEmail"
          class="btn btn-xs sm:btn-sm btn-ghost gap-1"
          title="Forward (F)"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
          </svg>
          <span class="hidden lg:inline">Forward</span>
        </button>

        <!-- Mark as Read/Unread -->
        <button
          @click="toggleReadStatus"
          :disabled="!selectedEmail"
          class="btn btn-xs sm:btn-sm btn-ghost gap-1"
          :title="selectedEmail?.is_read ? 'Mark as Unread' : 'Mark as Read'"
        >
          <svg v-if="selectedEmail?.is_read" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 19v-8.93a2 2 0 01.89-1.664l7-4.666a2 2 0 012.22 0l7 4.666A2 2 0 0121 10.07V19M3 19a2 2 0 002 2h14a2 2 0 002-2M3 19l6.75-4.5M21 19l-6.75-4.5M3 10l6.75 4.5M21 10l-6.75 4.5m0 0l-1.14.76a2 2 0 01-2.22 0l-1.14-.76" />
          </svg>
          <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
          <span class="hidden lg:inline">{{ selectedEmail?.is_read ? 'Mark Unread' : 'Mark Read' }}</span>
        </button>

        <!-- Flag/Star -->
        <button
          @click="toggleFlag"
          :disabled="!selectedEmail"
          class="btn btn-xs sm:btn-sm btn-ghost gap-1"
          :title="selectedEmail?.is_flagged ? 'Remove flag' : 'Flag message'"
        >
          <svg
            class="w-4 h-4"
            :class="selectedEmail?.is_flagged ? 'text-warning' : ''"
            :fill="selectedEmail?.is_flagged ? 'currentColor' : 'none'"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" />
          </svg>
          <span class="hidden lg:inline">{{ selectedEmail?.is_flagged ? 'Unflag' : 'Flag' }}</span>
        </button>
      </div>

      <!-- Right: Search box -->
      <div class="relative">
        <div class="flex items-center gap-1 sm:gap-2">
          <input
            v-model="searchQuery"
            @input="handleSearch"
            type="text"
            placeholder="Search..."
            class="input input-xs sm:input-sm input-bordered w-full sm:w-48 lg:w-64"
            autocomplete="off"
          />
          <button type="button" class="btn btn-xs sm:btn-sm btn-ghost btn-square">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Three-column Layout (matching EIQ web) -->
    <div class="flex overflow-hidden" style="height: calc(100vh - 140px);">
      <!-- Left Panel: Folders Sidebar -->
      <div class="bg-base-100 border-r border-base-300 overflow-y-auto flex-shrink-0 hidden lg:block" style="width: 256px;">
        <div class="p-2">
          <!-- Add Account Button -->
          <div class="px-2 py-2">
            <button class="btn btn-sm btn-ghost btn-block justify-start gap-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              Add Account
            </button>
          </div>

          <div class="divider my-1"></div>

          <!-- Folders List -->
          <div class="menu menu-sm">
            <li v-for="folder in emailStore.folders" :key="folder.id">
              <a
                @click="emailStore.selectFolder(folder)"
                :class="{ 'active': selectedFolder?.id === folder.id }"
                class="flex items-center justify-between"
              >
                <div class="flex items-center gap-2">
                  <component :is="getFolderIcon(folder.type)" class="w-4 h-4" />
                  <span class="truncate">{{ folder.name }}</span>
                </div>
                <span v-if="folder.unread_count > 0" class="badge badge-xs badge-primary">
                  {{ folder.unread_count }}
                </span>
              </a>
            </li>
          </div>
        </div>

        <!-- Storage Quota -->
        <div v-if="emailStore.quota" class="p-4 border-t border-base-300">
          <div class="text-xs text-base-content/70 mb-2">
            Storage: {{ formatBytes(emailStore.quota.used) }} / {{ formatBytes(emailStore.quota.total) }}
          </div>
          <progress
            class="progress progress-primary w-full"
            :value="emailStore.quota.percentage"
            max="100"
          ></progress>
        </div>
      </div>

      <!-- Center Panel: Message List -->
      <div class="flex-1 bg-base-100 border-r border-base-300 overflow-hidden flex flex-col min-w-0">
        <!-- Message list header -->
        <div class="px-2 sm:px-4 py-2 sm:py-3 border-b border-base-300 flex items-center gap-2 shrink-0">
          <h2 class="font-semibold text-sm sm:text-base">{{ selectedFolder?.name.toUpperCase() || 'INBOX' }}</h2>
          <span class="text-xs sm:text-sm text-base-content/60">
            <span class="font-semibold">{{ emailStore.pagination.total }} messages in this folder</span>
            <span v-if="selectedFolder?.unread_count > 0" class="text-warning">
              ({{ selectedFolder.unread_count }} unread)
            </span>
          </span>
        </div>

        <!-- Message list -->
        <div class="flex-1 overflow-y-auto">
          <div v-if="emailStore.loading && emails.length === 0" class="flex items-center justify-center h-full">
            <div class="text-center">
              <div class="loading loading-spinner loading-lg text-primary mb-4"></div>
              <p class="text-base-content/60">Loading messages...</p>
            </div>
          </div>

          <div v-else-if="emails.length === 0" class="flex items-center justify-center h-full text-base-content/60">
            <div class="text-center">
              <svg class="w-16 h-16 mx-auto mb-2 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
              </svg>
              <p>No messages in this folder</p>
            </div>
          </div>

          <div v-else class="divide-y divide-base-300">
            <div
              v-for="email in emails"
              :key="email.id"
              @click="selectEmail(email)"
              class="group relative flex items-start gap-2 px-2 sm:px-4 py-3 sm:py-4 hover:bg-base-200 transition-colors cursor-pointer"
              :class="{
                'bg-base-200': selectedEmail?.id === email.id
              }"
              style="min-height: 44px;"
            >
              <!-- Avatar -->
              <div class="w-10 h-10 rounded-full bg-primary text-primary-content flex items-center justify-center text-sm font-bold shrink-0">
                {{ email.from.name ? email.from.name.charAt(0).toUpperCase() : '?' }}
              </div>

              <!-- Message content -->
              <div class="flex-1 min-w-0" :class="{ 'font-semibold': !email.is_read }">
                <!-- From name and date/time -->
                <div class="flex items-start justify-between gap-2 mb-1">
                  <span class="text-sm truncate">{{ email.from.name || email.from.email }}</span>
                  <div class="text-xs text-base-content/60 shrink-0 text-right">
                    <div class="hidden sm:block">{{ formatDateTime(email.date) }}</div>
                    <div class="text-[10px] sm:hidden">{{ formatDateShort(email.date) }}</div>
                    <div class="text-[10px]">{{ formatTime(email.date) }}</div>
                  </div>
                </div>

                <!-- Subject with attachment indicator -->
                <div class="text-sm truncate mb-1 flex items-center gap-1">
                  <svg v-if="email.has_attachments" class="w-3 h-3 text-base-content/60" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                  </svg>
                  {{ email.subject || '(No subject)' }}
                </div>

                <!-- Preview - hidden on mobile -->
                <div class="text-xs text-base-content/60 truncate hidden sm:block">
                  {{ getEmailPreview(email) }}
                </div>
              </div>
            </div>

            <!-- Load More -->
            <div v-if="emailStore.pagination.has_more" class="p-4 text-center">
              <button
                @click="emailStore.loadMore"
                class="btn btn-sm btn-outline"
                :disabled="emailStore.loading"
              >
                <span v-if="emailStore.loading" class="loading loading-spinner"></span>
                <span v-else>Load More</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Panel: Reading Pane -->
      <div class="flex-1 bg-base-100 overflow-hidden flex flex-col min-w-0">
        <div v-if="!selectedEmail" class="flex-1 flex items-center justify-center text-base-content/60">
          <div class="text-center px-4">
            <svg class="w-16 h-16 sm:w-24 sm:h-24 mx-auto mb-4 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 19v-8.93a2 2 0 01.89-1.664l7-4.666a2 2 0 012.22 0l7 4.666A2 2 0 0121 10.07V19M3 19a2 2 0 002 2h14a2 2 0 002-2M3 19l6.75-4.5M21 19l-6.75-4.5M3 10l6.75 4.5M21 10l-6.75 4.5m0 0l-1.14.76a2 2 0 01-2.22 0l-1.14-.76" />
            </svg>
            <p class="text-base sm:text-lg font-medium">Select a message to read</p>
          </div>
        </div>

        <EmailDetail v-else :email="selectedEmail" />
      </div>
    </div>

    <!-- Composer Modal -->
    <EmailComposer v-if="emailStore.composerOpen" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useEmailStore } from '../stores/emailStore'
import type { Email, EmailFolder } from '../types/email'
import EmailDetail from '../components/EmailDetail.vue'
import EmailComposer from '../components/EmailComposer.vue'
import { format, formatDistanceToNow } from 'date-fns'

const emailStore = useEmailStore()

const searchQuery = ref('')
let searchTimeout: number | null = null

const emails = computed(() => emailStore.emails)
const selectedEmail = computed(() => emailStore.selectedEmail)
const selectedFolder = computed(() => emailStore.selectedFolder)

function getFolderIcon(type: string) {
  // Return appropriate icon component or default
  return 'div'
}

function handleSearch() {
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }

  searchTimeout = window.setTimeout(() => {
    emailStore.searchEmails(searchQuery.value)
  }, 300)
}

function selectEmail(email: Email) {
  emailStore.selectEmail(email)
  emailStore.fetchEmail(email.id)
}

function deleteEmail() {
  if (selectedEmail.value) {
    // TODO: Implement delete
    console.log('Delete email:', selectedEmail.value.id)
  }
}

function archiveEmail() {
  if (selectedEmail.value) {
    // TODO: Implement archive
    console.log('Archive email:', selectedEmail.value.id)
  }
}

function moveToFolder(folder: EmailFolder) {
  if (selectedEmail.value) {
    // TODO: Implement move
    console.log('Move email to:', folder.name)
  }
}

function replyToEmail() {
  if (selectedEmail.value) {
    // TODO: Implement reply
    console.log('Reply to:', selectedEmail.value.id)
  }
}

function replyAllToEmail() {
  if (selectedEmail.value) {
    // TODO: Implement reply all
    console.log('Reply all to:', selectedEmail.value.id)
  }
}

function forwardEmail() {
  if (selectedEmail.value) {
    // TODO: Implement forward
    console.log('Forward:', selectedEmail.value.id)
  }
}

function toggleReadStatus() {
  if (selectedEmail.value) {
    // TODO: Implement toggle read
    console.log('Toggle read status:', selectedEmail.value.id)
  }
}

function toggleFlag() {
  if (selectedEmail.value) {
    // TODO: Implement toggle flag
    console.log('Toggle flag:', selectedEmail.value.id)
  }
}

function formatDateTime(date: string) {
  return format(new Date(date), 'MMM d, h:mm a')
}

function formatDateShort(date: string) {
  return format(new Date(date), 'MMM d')
}

function formatTime(date: string) {
  return format(new Date(date), 'h:mm a')
}

function getEmailPreview(email: Email): string {
  const text = email.body_text || email.body_html?.replace(/<[^>]*>/g, '') || ''
  return text.substring(0, 100)
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

onMounted(async () => {
  await emailStore.fetchFolders()
  await emailStore.fetchEmails()
  await emailStore.fetchQuota()
})
</script>
