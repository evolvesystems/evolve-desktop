<template>
  <div class="flex flex-col h-full">
    <!-- Email Header -->
    <div class="border-b border-base-300 p-4">
      <!-- Subject -->
      <h2 class="text-xl font-semibold mb-4">{{ email.subject || '(No subject)' }}</h2>

      <!-- From/To Info -->
      <div class="space-y-2 text-sm mb-4">
        <div class="flex items-start gap-2">
          <span class="text-base-content/70 w-12">From:</span>
          <div class="flex-1">
            <span class="font-medium">{{ email.from.name || email.from.email }}</span>
            <span v-if="email.from.name" class="text-base-content/70 ml-2">&lt;{{ email.from.email }}&gt;</span>
          </div>
        </div>

        <div class="flex items-start gap-2">
          <span class="text-base-content/70 w-12">To:</span>
          <div class="flex-1">
            <span v-for="(recipient, index) in email.to" :key="index">
              <span class="font-medium">{{ recipient.name || recipient.email }}</span>
              <span v-if="recipient.name" class="text-base-content/70">&lt;{{ recipient.email }}&gt;</span>
              <span v-if="index < email.to.length - 1">, </span>
            </span>
          </div>
        </div>

        <div v-if="email.cc && email.cc.length > 0" class="flex items-start gap-2">
          <span class="text-base-content/70 w-12">Cc:</span>
          <div class="flex-1">
            <span v-for="(recipient, index) in email.cc" :key="index">
              <span class="font-medium">{{ recipient.name || recipient.email }}</span>
              <span v-if="index < email.cc.length - 1">, </span>
            </span>
          </div>
        </div>

        <div class="flex items-center gap-2">
          <span class="text-base-content/70 w-12">Date:</span>
          <span>{{ formatDate(email.date) }}</span>
        </div>
      </div>

      <!-- Action Toolbar -->
      <div class="flex items-center gap-2">
        <button @click="handleReply" class="btn btn-sm btn-outline gap-2">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
          </svg>
          Reply
        </button>

        <button @click="handleReplyAll" class="btn btn-sm btn-outline gap-2">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
          </svg>
          Reply All
        </button>

        <button @click="handleForward" class="btn btn-sm btn-outline gap-2">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
          </svg>
          Forward
        </button>

        <div class="flex-1"></div>

        <button
          @click="toggleFlag"
          class="btn btn-sm btn-ghost btn-square"
          :class="{ 'text-warning': email.is_flagged }"
          title="Flag"
        >
          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
            <path d="M14.4 6L14 4H5v17h2v-7h5.6l.4 2h7V6z" />
          </svg>
        </button>

        <div class="dropdown dropdown-end">
          <button tabindex="0" class="btn btn-sm btn-ghost btn-square">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
            </svg>
          </button>
          <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
            <li><a @click="handleMarkUnread">Mark as unread</a></li>
            <li><a @click="handleMoveToFolder">Move to folder...</a></li>
            <li class="border-t border-base-300 mt-1 pt-1">
              <a @click="handleDelete" class="text-error">Delete</a>
            </li>
          </ul>
        </div>
      </div>
    </div>

    <!-- Email Body -->
    <div class="flex-1 overflow-y-auto p-6">
      <!-- Attachments -->
      <div v-if="email.has_attachments && email.attachments && email.attachments.length > 0" class="mb-6">
        <div class="text-sm font-medium mb-2">
          {{ email.attachments.length }} attachment{{ email.attachments.length > 1 ? 's' : '' }}
        </div>
        <div class="flex flex-wrap gap-2">
          <a
            v-for="attachment in email.attachments"
            :key="attachment.id"
            :href="attachment.url"
            target="_blank"
            class="flex items-center gap-2 px-3 py-2 bg-base-200 hover:bg-base-300 rounded-lg text-sm"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
            </svg>
            <span class="font-medium">{{ attachment.filename }}</span>
            <span class="text-base-content/50">({{ formatBytes(attachment.size) }})</span>
          </a>
        </div>
      </div>

      <!-- Body Content -->
      <div v-if="email.body_html" class="prose prose-sm max-w-none" v-html="sanitizeHtml(email.body_html)"></div>
      <div v-else-if="email.body_text" class="whitespace-pre-wrap">{{ email.body_text }}</div>
      <div v-else class="text-base-content/50 italic">No content</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmailStore } from '../stores/emailStore'
import type { Email } from '../types/email'
import { format } from 'date-fns'
import DOMPurify from 'dompurify'

interface Props {
  email: Email
}

const props = defineProps<Props>()
const emailStore = useEmailStore()

function formatDate(date: string): string {
  return format(new Date(date), 'PPpp')
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

function sanitizeHtml(html: string): string {
  return DOMPurify.sanitize(html, {
    ALLOWED_TAGS: ['p', 'br', 'strong', 'em', 'u', 'a', 'ul', 'ol', 'li', 'blockquote', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'img', 'div', 'span'],
    ALLOWED_ATTR: ['href', 'src', 'alt', 'title', 'class', 'style']
  })
}

async function handleReply() {
  emailStore.openComposer()
  // TODO: Set reply context in composer
}

async function handleReplyAll() {
  emailStore.openComposer()
  // TODO: Set reply-all context in composer
}

async function handleForward() {
  emailStore.openComposer()
  // TODO: Set forward context in composer
}

async function toggleFlag() {
  await emailStore.markAsFlagged(props.email.id, !props.email.is_flagged)
}

async function handleMarkUnread() {
  await emailStore.markAsRead(props.email.id, false)
}

async function handleMoveToFolder() {
  // TODO: Show folder selection dialog
  console.log('Move to folder')
}

async function handleDelete() {
  if (confirm('Are you sure you want to delete this email?')) {
    await emailStore.deleteEmail(props.email.id)
  }
}
</script>

<style scoped>
.prose {
  color: inherit;
}
</style>
