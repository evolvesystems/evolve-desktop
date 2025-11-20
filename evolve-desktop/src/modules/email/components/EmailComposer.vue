<template>
  <div class="modal modal-open">
    <div class="modal-box max-w-4xl h-[80vh] flex flex-col p-0">
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-base-300">
        <h3 class="text-lg font-bold">
          {{ composeData.type === 'reply' ? 'Reply' : composeData.type === 'reply-all' ? 'Reply All' : composeData.type === 'forward' ? 'Forward' : 'New Message' }}
        </h3>
        <button class="btn btn-sm btn-circle btn-ghost" @click="handleClose">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Form -->
      <div class="flex-1 overflow-y-auto p-4 space-y-3">
        <!-- To -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">To</span>
          </label>
          <input
            v-model="form.to"
            type="text"
            placeholder="recipient@example.com (comma-separated for multiple)"
            class="input input-bordered"
          />
        </div>

        <!-- Cc -->
        <div class="form-control">
          <label class="label">
            <span class="label-text">Cc</span>
          </label>
          <input
            v-model="form.cc"
            type="text"
            placeholder="cc@example.com (optional)"
            class="input input-bordered"
          />
        </div>

        <!-- Bcc -->
        <div class="form-control">
          <label class="label">
            <span class="label-text">Bcc</span>
          </label>
          <input
            v-model="form.bcc"
            type="text"
            placeholder="bcc@example.com (optional)"
            class="input input-bordered"
          />
        </div>

        <!-- Subject -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-semibold">Subject</span>
          </label>
          <input
            v-model="form.subject"
            type="text"
            placeholder="Email subject"
            class="input input-bordered"
          />
        </div>

        <!-- Body -->
        <div class="form-control flex-1">
          <label class="label">
            <span class="label-text font-semibold">Message</span>
          </label>
          <textarea
            v-model="form.body"
            class="textarea textarea-bordered h-64 font-mono"
            placeholder="Type your message here..."
          ></textarea>
        </div>

        <!-- Attachments -->
        <div class="form-control">
          <label class="label">
            <span class="label-text">Attachments</span>
          </label>
          <input
            type="file"
            multiple
            class="file-input file-input-bordered"
            @change="handleFileChange"
          />
          <div v-if="form.attachments.length > 0" class="mt-2">
            <div class="flex flex-wrap gap-2">
              <div
                v-for="(file, index) in form.attachments"
                :key="index"
                class="badge badge-lg gap-2"
              >
                {{ file.name }}
                <button
                  class="btn btn-xs btn-ghost btn-circle"
                  @click="removeAttachment(index)"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="flex items-center justify-between p-4 border-t border-base-300">
        <button class="btn btn-ghost" @click="handleClose">Cancel</button>
        <button
          class="btn btn-primary"
          :disabled="isSending || !isFormValid"
          @click="handleSend"
        >
          <span v-if="isSending" class="loading loading-spinner loading-sm"></span>
          <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
          </svg>
          {{ isSending ? 'Sending...' : 'Send' }}
        </button>
      </div>
    </div>
    <div class="modal-backdrop" @click="handleClose"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useEmailStore } from '@/stores/email'

const props = defineProps<{
  composeData?: {
    to?: string[]
    cc?: string[]
    bcc?: string[]
    subject?: string
    body?: string
    type?: 'new' | 'reply' | 'reply-all' | 'forward'
    originalEmail?: any
  }
}>()

const emit = defineEmits<{
  'close': []
  'sent': []
}>()

const emailStore = useEmailStore()

const form = ref({
  to: '',
  cc: '',
  bcc: '',
  subject: '',
  body: '',
  attachments: [] as File[]
})

const isSending = ref(false)

const isFormValid = computed(() => {
  return form.value.to.trim() !== '' && form.value.subject.trim() !== ''
})

onMounted(() => {
  if (props.composeData) {
    form.value.to = props.composeData.to?.join(', ') || ''
    form.value.cc = props.composeData.cc?.join(', ') || ''
    form.value.bcc = props.composeData.bcc?.join(', ') || ''
    form.value.subject = props.composeData.subject || ''
    form.value.body = props.composeData.body || ''
  }
})

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files) {
    form.value.attachments = [...form.value.attachments, ...Array.from(target.files)]
  }
}

function removeAttachment(index: number) {
  form.value.attachments.splice(index, 1)
}

function handleClose() {
  if (confirm('Discard this message?')) {
    emit('close')
  }
}

async function handleSend() {
  if (!isFormValid.value) return

  isSending.value = true

  try {
    const success = await emailStore.sendEmail({
      to: form.value.to.split(',').map(e => e.trim()).filter(Boolean),
      cc: form.value.cc ? form.value.cc.split(',').map(e => e.trim()).filter(Boolean) : undefined,
      bcc: form.value.bcc ? form.value.bcc.split(',').map(e => e.trim()).filter(Boolean) : undefined,
      subject: form.value.subject,
      body: form.value.body,
      attachments: form.value.attachments.length > 0 ? form.value.attachments : undefined
    })

    if (success) {
      emit('sent')
    } else {
      alert('Failed to send email. Please try again.')
    }
  } catch (error) {
    console.error('Failed to send email:', error)
    alert('Failed to send email. Please try again.')
  } finally {
    isSending.value = false
  }
}
</script>
