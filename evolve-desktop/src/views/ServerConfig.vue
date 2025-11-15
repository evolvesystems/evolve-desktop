<template>
  <div class="min-h-screen flex items-center justify-center bg-base-200 p-4">
    <div class="card w-full max-w-md bg-base-100 shadow-xl">
      <div class="card-body">
        <div class="text-center mb-6">
          <div class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-primary flex items-center justify-center">
            <span class="text-primary-content font-bold text-2xl">E</span>
          </div>
          <h1 class="text-2xl font-bold">EvolveApp Setup</h1>
          <p class="text-base-content/70">Connect to your EIQ Manager</p>
        </div>

        <div class="space-y-4">
          <!-- Preset Options -->
          <div class="form-control">
            <label class="label">
              <span class="label-text">Choose your setup</span>
            </label>
            <div class="space-y-2">
              <label class="label cursor-pointer border rounded-lg p-3 hover:bg-base-200">
                <span class="label-text">
                  <div class="font-semibold">EvolveApp Cloud</div>
                  <div class="text-xs text-base-content/70">Hosted by EvolveApp</div>
                </span>
                <input
                  type="radio"
                  name="server"
                  class="radio radio-primary"
                  value="cloud"
                  v-model="selectedOption"
                  @change="handlePresetChange"
                />
              </label>

              <label class="label cursor-pointer border rounded-lg p-3 hover:bg-base-200">
                <span class="label-text">
                  <div class="font-semibold">Self-Hosted</div>
                  <div class="text-xs text-base-content/70">Your own server</div>
                </span>
                <input
                  type="radio"
                  name="server"
                  class="radio radio-primary"
                  value="custom"
                  v-model="selectedOption"
                  @change="handlePresetChange"
                />
              </label>

              <label class="label cursor-pointer border rounded-lg p-3 hover:bg-base-200">
                <span class="label-text">
                  <div class="font-semibold">Local Development</div>
                  <div class="text-xs text-base-content/70">localhost:8547</div>
                </span>
                <input
                  type="radio"
                  name="server"
                  class="radio radio-primary"
                  value="local"
                  v-model="selectedOption"
                  @change="handlePresetChange"
                />
              </label>
            </div>
          </div>

          <!-- Custom URL Input -->
          <div v-if="selectedOption === 'custom'" class="form-control">
            <label class="label">
              <span class="label-text">Server URL</span>
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

          <!-- Test Connection -->
          <div v-if="apiUrl" class="alert alert-info">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <span class="text-sm">Will connect to: {{ apiUrl }}</span>
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
          <div class="flex gap-2 mt-6">
            <button
              @click="testConnection"
              class="btn btn-outline flex-1"
              :disabled="!apiUrl || testing"
            >
              <span v-if="testing" class="loading loading-spinner"></span>
              <span v-else>Test Connection</span>
            </button>

            <button
              @click="saveAndContinue"
              class="btn btn-primary flex-1"
              :disabled="!apiUrl || testing"
            >
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
      return 'https://api.evolveapp.com'
    case 'local':
      return 'http://localhost:8547'
    case 'custom':
      return customUrl.value
    default:
      return ''
  }
})

function handlePresetChange() {
  error.value = null
  success.value = false
}

async function testConnection() {
  if (!apiUrl.value) return

  testing.value = true
  error.value = null
  success.value = false

  try {
    // Test API health endpoint
    const response = await axios.get(`${apiUrl.value}/api/health`, {
      timeout: 5000
    })

    if (response.status === 200) {
      success.value = true
    }
  } catch (err: any) {
    error.value = err.message || 'Failed to connect to server'
    success.value = false
  } finally {
    testing.value = false
  }
}

function saveAndContinue() {
  if (!apiUrl.value) return

  // Save API URL to localStorage
  localStorage.setItem('api_url', apiUrl.value)

  // Update axios default
  axios.defaults.baseURL = apiUrl.value

  // Redirect to login
  router.push('/login')
}
</script>
