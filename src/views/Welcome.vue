<template>
  <div class="min-h-screen flex items-center justify-center bg-base-200 p-4">
    <div class="card w-full max-w-2xl bg-base-100 shadow-xl">
      <div class="card-body">
        <!-- Header -->
        <div class="text-center mb-8">
          <div class="w-20 h-20 mx-auto mb-4 rounded-2xl bg-primary flex items-center justify-center">
            <span class="text-primary-content font-bold text-3xl">E</span>
          </div>
          <h1 class="text-3xl font-bold">Welcome to EvolveApp</h1>
          <p class="text-base-content/70 mt-2">Let's get you set up in just a few seconds</p>
        </div>

        <!-- Setup Options -->
        <div class="space-y-4">
          <h2 class="text-lg font-semibold mb-3">Choose your setup:</h2>

          <!-- Cloud Option -->
          <label class="label cursor-pointer border-2 rounded-lg p-4 hover:bg-base-200 transition-colors" :class="{ 'border-primary bg-primary/5': selectedOption === 'cloud' }">
            <div class="flex-1">
              <div class="flex items-center gap-3 mb-2">
                <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
                </svg>
                <span class="font-semibold text-lg">EvolveApp Cloud</span>
                <span class="badge badge-primary badge-sm">Recommended</span>
              </div>
              <p class="text-sm text-base-content/70 ml-9">
                Hosted by EvolveApp. No setup required, automatic updates, secure and reliable.
              </p>
            </div>
            <input
              type="radio"
              name="setup"
              class="radio radio-primary"
              value="cloud"
              v-model="selectedOption"
            />
          </label>

          <!-- Self-Hosted Option -->
          <label class="label cursor-pointer border-2 rounded-lg p-4 hover:bg-base-200 transition-colors" :class="{ 'border-primary bg-primary/5': selectedOption === 'custom' }">
            <div class="flex-1">
              <div class="flex items-center gap-3 mb-2">
                <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01" />
                </svg>
                <span class="font-semibold text-lg">Self-Hosted Server</span>
              </div>
              <p class="text-sm text-base-content/70 ml-9">
                Connect to your own EIQ Manager installation. Full control over your data.
              </p>
            </div>
            <input
              type="radio"
              name="setup"
              class="radio radio-primary"
              value="custom"
              v-model="selectedOption"
            />
          </label>

          <!-- Local Development Option -->
          <label class="label cursor-pointer border-2 rounded-lg p-4 hover:bg-base-200 transition-colors" :class="{ 'border-primary bg-primary/5': selectedOption === 'local' }">
            <div class="flex-1">
              <div class="flex items-center gap-3 mb-2">
                <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                </svg>
                <span class="font-semibold text-lg">Local Development</span>
              </div>
              <p class="text-sm text-base-content/70 ml-9">
                Running EIQ Manager on your local machine (localhost:8547).
              </p>
            </div>
            <input
              type="radio"
              name="setup"
              class="radio radio-primary"
              value="local"
              v-model="selectedOption"
            />
          </label>

          <!-- Custom URL Input -->
          <div v-if="selectedOption === 'custom'" class="pl-9 mt-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text font-medium">Server URL</span>
              </label>
              <input
                v-model="customUrl"
                type="url"
                placeholder="https://your-server.com"
                class="input input-bordered"
                required
              />
              <label class="label">
                <span class="label-text-alt">Enter your EIQ Manager API URL</span>
              </label>
            </div>
          </div>

          <!-- Connection Info -->
          <div v-if="apiUrl" class="alert alert-info mt-4">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <div>
              <div class="font-medium">Will connect to:</div>
              <div class="text-sm font-mono">{{ apiUrl }}</div>
            </div>
          </div>

          <!-- Error Display -->
          <div v-if="error" class="alert alert-error">
            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>{{ error }}</span>
          </div>

          <!-- Success Display -->
          <div v-if="success" class="alert alert-success">
            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>Connection successful!</span>
          </div>

          <!-- Action Buttons -->
          <div class="flex gap-3 mt-6">
            <button
              @click="testConnection"
              class="btn btn-outline flex-1"
              :disabled="!apiUrl || testing"
            >
              <span v-if="testing" class="loading loading-spinner"></span>
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              Test Connection
            </button>

            <button
              @click="saveAndContinue"
              class="btn btn-primary flex-1"
              :disabled="!apiUrl || testing"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
              </svg>
              Continue
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

const router = useRouter()

const selectedOption = ref<'cloud' | 'custom' | 'local'>('cloud')
const customUrl = ref('')
const error = ref<string | null>(null)
const success = ref(false)
const testing = ref(false)

const apiUrl = computed(() => {
  switch (selectedOption.value) {
    case 'cloud':
      return import.meta.env.VITE_API_URL || 'https://api.evolveapp.com'
    case 'local':
      return 'http://localhost:8547'
    case 'custom':
      return customUrl.value
    default:
      return ''
  }
})

async function testConnection() {
  if (!apiUrl.value) return

  testing.value = true
  error.value = null
  success.value = false

  try {
    // Test API by trying to access login endpoint
    const response = await axios.post(`${apiUrl.value}/api/v1/auth/login`, {
      email: 'test@test.com',
      password: 'test'
    }, {
      timeout: 5000,
      validateStatus: (status) => status === 401 || status === 200 || status === 400
    })

    // If we get any response (even 401), the server is reachable
    success.value = true
  } catch (err: any) {
    if (err.code === 'ECONNREFUSED' || err.code === 'ETIMEDOUT') {
      error.value = 'Cannot connect to server. Please check the URL and try again.'
    } else {
      // Other errors mean the server responded, which is good enough
      success.value = true
    }
  } finally {
    testing.value = false
  }
}

function saveAndContinue() {
  if (!apiUrl.value) return

  // Save API URL to localStorage
  localStorage.setItem('api_url', apiUrl.value)
  localStorage.setItem('api_configured', 'true')

  // Update axios default
  axios.defaults.baseURL = apiUrl.value

  // Redirect to login
  router.push('/login')
}
</script>
