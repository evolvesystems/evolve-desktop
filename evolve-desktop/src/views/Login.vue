<template>
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary/10 via-base-200 to-secondary/10 p-4">
    <!-- Background Pattern -->
    <div class="absolute inset-0 opacity-5">
      <div class="absolute inset-0" style="background-image: radial-gradient(circle, currentColor 1px, transparent 1px); background-size: 32px 32px;"></div>
    </div>

    <div class="relative w-full max-w-md">
      <!-- Main Login Card -->
      <div class="card bg-base-100 shadow-2xl border border-base-300">
        <div class="card-body p-8">
          <!-- Logo & Branding -->
          <div class="text-center mb-8">
            <div class="inline-flex items-center justify-center w-20 h-20 mb-4 rounded-2xl bg-gradient-to-br from-primary to-secondary shadow-lg">
              <svg class="w-10 h-10 text-primary-content" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
              </svg>
            </div>
            <h1 class="text-3xl font-bold bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
              EvolveApp
            </h1>
            <p class="text-base-content/60 mt-2">Sign in to your account</p>
            <p class="text-base-content/40 text-xs mt-1">Version {{ appVersion }}</p>
          </div>

          <!-- Login Form -->
          <form @submit.prevent="handleLogin" class="space-y-5">
            <!-- Email Field -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Email Address</span>
              </label>
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg class="w-5 h-5 text-base-content/40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207" />
                  </svg>
                </div>
                <input
                  v-model="email"
                  type="email"
                  placeholder="you@example.com"
                  class="input input-bordered w-full pl-10 focus:input-primary transition-all bg-base-100"
                  required
                  :disabled="loading"
                />
              </div>
            </div>

            <!-- Password Field -->
            <div class="form-control">
              <label class="label">
                <span class="label-text font-semibold">Password</span>
              </label>
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg class="w-5 h-5 text-base-content/40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                  </svg>
                </div>
                <input
                  v-model="password"
                  type="password"
                  placeholder="Enter your password"
                  class="input input-bordered w-full pl-10 focus:input-primary transition-all bg-base-100"
                  required
                  :disabled="loading"
                />
              </div>
              <label class="label">
                <span class="label-text-alt"></span>
                <a href="#" class="label-text-alt link link-hover link-primary">Forgot password?</a>
              </label>
            </div>

            <!-- Error Alert -->
            <div v-if="error" class="alert alert-error shadow-lg">
              <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span class="text-sm">{{ error }}</span>
            </div>

            <!-- Submit Button -->
            <div class="form-control mt-6">
              <button
                type="submit"
                class="btn btn-primary btn-lg w-full shadow-lg hover:shadow-xl transition-all gap-2"
                :disabled="loading"
              >
                <span v-if="loading" class="loading loading-spinner"></span>
                <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1" />
                </svg>
                <span>{{ loading ? 'Signing in...' : 'Sign In' }}</span>
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()

const email = ref('')
const password = ref('')
const loading = ref(false)
const error = ref<string | null>(null)
const appVersion = ref('')

onMounted(async () => {
  // Get app version from Tauri backend
  try {
    appVersion.value = await invoke<string>('get_app_version')
  } catch (e) {
    appVersion.value = '1.0.10'
  }
})

async function handleLogin() {
  loading.value = true
  error.value = null

  try {
    await authStore.login(email.value, password.value)

    // Redirect to original destination or dashboard
    const redirect = route.query.redirect as string
    router.push(redirect || '/dashboard')
  } catch (err: any) {
    error.value = err.response?.data?.message || 'Login failed. Please check your credentials.'
  } finally {
    loading.value = false
  }
}
</script>
