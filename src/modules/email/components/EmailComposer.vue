<template>
  <div class="modal modal-open">
    <div class="modal-box max-w-4xl h-[90vh] flex flex-col p-0">
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-base-300">
        <h3 class="text-lg font-semibold">{{ composerTitle }}</h3>
        <button @click="handleClose" class="btn btn-sm btn-ghost btn-circle">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Form -->
      <div class="flex-1 overflow-y-auto p-4 space-y-3">
        <!-- To Field -->
        <div class="form-control">
          <div class="flex items-center gap-2">
            <label class="label w-16 justify-start p-0">
              <span class="label-text">To:</span>
            </label>
            <input
              v-model="form.to"
              type="text"
              placeholder="recipient@example.com"
              class="input input-sm input-bordered flex-1 bg-base-100"
              @blur="validateEmail('to')"
            />
            <button @click="showCc = !showCc" class="btn btn-xs btn-ghost">Cc</button>
            <button @click="showBcc = !showBcc" class="btn btn-xs btn-ghost">Bcc</button>
          </div>
          <div v-if="errors.to" class="label">
            <span class="label-text-alt text-error">{{ errors.to }}</span>
          </div>
        </div>

        <!-- Cc Field -->
        <div v-if="showCc" class="form-control">
          <div class="flex items-center gap-2">
            <label class="label w-16 justify-start p-0">
              <span class="label-text">Cc:</span>
            </label>
            <input
              v-model="form.cc"
              type="text"
              placeholder="cc@example.com"
              class="input input-sm input-bordered flex-1 bg-base-100"
            />
          </div>
        </div>

        <!-- Bcc Field -->
        <div v-if="showBcc" class="form-control">
          <div class="flex items-center gap-2">
            <label class="label w-16 justify-start p-0">
              <span class="label-text">Bcc:</span>
            </label>
            <input
              v-model="form.bcc"
              type="text"
              placeholder="bcc@example.com"
              class="input input-sm input-bordered flex-1 bg-base-100"
            />
          </div>
        </div>

        <!-- Subject Field -->
        <div class="form-control">
          <div class="flex items-center gap-2">
            <label class="label w-16 justify-start p-0">
              <span class="label-text">Subject:</span>
            </label>
            <input
              v-model="form.subject"
              type="text"
              placeholder="Email subject"
              class="input input-sm input-bordered flex-1 bg-base-100"
            />
          </div>
        </div>

        <!-- Formatting Toolbar -->
        <div class="border-y border-base-300 py-2 flex items-center gap-1">
          <button @click="applyFormat('bold')" class="btn btn-xs btn-ghost btn-square" title="Bold">
            <span class="font-bold">B</span>
          </button>
          <button @click="applyFormat('italic')" class="btn btn-xs btn-ghost btn-square" title="Italic">
            <span class="italic">I</span>
          </button>
          <button @click="applyFormat('underline')" class="btn btn-xs btn-ghost btn-square" title="Underline">
            <span class="underline">U</span>
          </button>

          <div class="divider divider-horizontal mx-1"></div>

          <button @click="applyFormat('insertUnorderedList')" class="btn btn-xs btn-ghost" title="Bullet List">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
          <button @click="applyFormat('insertOrderedList')" class="btn btn-xs btn-ghost" title="Numbered List">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>

          <div class="divider divider-horizontal mx-1"></div>

          <label class="btn btn-xs btn-ghost gap-1" title="Attach File">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
            </svg>
            Attach
            <input
              type="file"
              multiple
              class="hidden"
              @change="handleFileSelect"
            />
          </label>
        </div>

        <!-- Body Editor -->
        <div class="form-control flex-1">
          <div
            ref="editorRef"
            contenteditable="true"
            @input="handleBodyInput"
            class="textarea textarea-bordered min-h-[300px] p-3 focus:outline-none bg-base-100"
            style="height: auto; white-space: pre-wrap;"
            placeholder="Write your message..."
          ></div>
        </div>

        <!-- Attachments -->
        <div v-if="attachments.length > 0" class="space-y-2">
          <div class="text-sm font-medium">Attachments ({{ attachments.length }})</div>
          <div class="flex flex-wrap gap-2">
            <div
              v-for="(file, index) in attachments"
              :key="index"
              class="flex items-center gap-2 px-3 py-2 bg-base-200 rounded-lg text-sm"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
              </svg>
              <span class="font-medium">{{ file.name }}</span>
              <span class="text-base-content/50">({{ formatBytes(file.size) }})</span>
              <button
                @click="removeAttachment(index)"
                class="btn btn-xs btn-ghost btn-circle ml-2"
              >
                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer Actions -->
      <div class="flex items-center justify-between p-4 border-t border-base-300">
        <div class="flex gap-2">
          <button
            @click="handleSend"
            class="btn btn-primary"
            :disabled="sending || !canSend"
          >
            <span v-if="sending" class="loading loading-spinner"></span>
            <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
            </svg>
            Send
          </button>

          <button @click="handleSaveDraft" class="btn btn-ghost">
            Save Draft
          </button>
        </div>

        <button @click="handleClose" class="btn btn-ghost">
          Discard
        </button>
      </div>
    </div>
    <div class="modal-backdrop" @click="handleClose"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useEmailStore } from '../stores/emailStore'
import type { SendEmailRequest } from '../types/email'

const emailStore = useEmailStore()

const form = ref({
  to: '',
  cc: '',
  bcc: '',
  subject: '',
  body: ''
})

const showCc = ref(false)
const showBcc = ref(false)
const attachments = ref<File[]>([])
const sending = ref(false)
const editorRef = ref<HTMLDivElement | null>(null)

const errors = ref({
  to: ''
})

const composerTitle = computed(() => {
  return 'New Message'
})

const canSend = computed(() => {
  return form.value.to.trim().length > 0 && !errors.value.to
})

function validateEmail(field: 'to' | 'cc' | 'bcc') {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  const emails = form.value[field].split(',').map(e => e.trim()).filter(e => e)

  if (field === 'to' && emails.length === 0) {
    errors.value.to = 'At least one recipient is required'
    return false
  }

  const invalidEmails = emails.filter(email => !emailRegex.test(email))
  if (invalidEmails.length > 0) {
    errors.value.to = `Invalid email address: ${invalidEmails[0]}`
    return false
  }

  errors.value.to = ''
  return true
}

function handleBodyInput() {
  if (editorRef.value) {
    form.value.body = editorRef.value.innerHTML
  }
}

function applyFormat(command: string) {
  document.execCommand(command, false)
  editorRef.value?.focus()
}

function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files) {
    attachments.value.push(...Array.from(target.files))
  }
  target.value = ''
}

function removeAttachment(index: number) {
  attachments.value.splice(index, 1)
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

async function handleSend() {
  if (!validateEmail('to')) return

  sending.value = true

  try {
    const emailData: SendEmailRequest = {
      to: form.value.to.split(',').map(e => e.trim()).filter(e => e),
      subject: form.value.subject,
      body_html: form.value.body,
      body_text: editorRef.value?.innerText || ''
    }

    if (form.value.cc) {
      emailData.cc = form.value.cc.split(',').map(e => e.trim()).filter(e => e)
    }

    if (form.value.bcc) {
      emailData.bcc = form.value.bcc.split(',').map(e => e.trim()).filter(e => e)
    }

    // TODO: Handle file attachments upload
    // if (attachments.value.length > 0) {
    //   emailData.attachments = await uploadAttachments(attachments.value)
    // }

    await emailStore.sendEmail(emailData)

    // Close composer on success
    emailStore.closeComposer()
  } catch (error) {
    console.error('Failed to send email:', error)
    alert('Failed to send email. Please try again.')
  } finally {
    sending.value = false
  }
}

async function handleSaveDraft() {
  // TODO: Implement save draft functionality
  console.log('Save draft')
}

function handleClose() {
  if (form.value.to || form.value.subject || form.value.body) {
    if (confirm('Discard this draft?')) {
      emailStore.closeComposer()
    }
  } else {
    emailStore.closeComposer()
  }
}

onMounted(() => {
  // Focus the To field
  setTimeout(() => {
    const toInput = document.querySelector('input[placeholder="recipient@example.com"]') as HTMLInputElement
    toInput?.focus()
  }, 100)
})
</script>

<style scoped>
[contenteditable]:empty:before {
  content: attr(placeholder);
  color: oklch(var(--bc) / 0.4);
  pointer-events: none;
}

[contenteditable]:focus {
  outline: none;
}
</style>
