import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from './router'
import App from './App.vue'
import './style.css'

// Import stores
import { useAuthStore } from './stores/auth'
import { useAppStore } from './stores/app'
import { useSettingsStore } from './stores/settings'

// Import plugin system
import { moduleLoader } from './core/plugin-system'
import axios from 'axios'

// Import modules
import emailModule from './modules/email'

// Configure axios defaults
// Use saved API URL from wizard, fallback to env variable or default
const savedApiUrl = localStorage.getItem('api_url')
axios.defaults.baseURL = savedApiUrl || import.meta.env.VITE_API_URL || 'http://localhost:8547'
axios.defaults.headers.common['Content-Type'] = 'application/json'
axios.defaults.headers.common['Accept'] = 'application/json'

// Create Vue app
const app = createApp(App)

// Create Pinia
const pinia = createPinia()

// Use plugins
app.use(pinia)
app.use(router)

// Initialize stores
const authStore = useAuthStore()
const appStore = useAppStore()
const settingsStore = useSettingsStore()

// Restore auth state from localStorage
authStore.restoreAuth()

// Restore theme
appStore.restoreTheme()

// Initialize settings if authenticated
if (authStore.isAuthenticated) {
  settingsStore.initialize().catch(error => {
    console.error('Failed to initialize settings:', error)
  })
}

// Set router for module loader
moduleLoader.setRouter(router)

// Setup axios request interceptor to always use correct baseURL
axios.interceptors.request.use(
  (config) => {
    const apiUrl = localStorage.getItem('api_url')
    if (apiUrl) {
      // Set baseURL
      config.baseURL = apiUrl

      // Also update the global default
      if (axios.defaults.baseURL !== apiUrl) {
        axios.defaults.baseURL = apiUrl
        console.log('[axios interceptor] Updated baseURL to:', apiUrl)
      }
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// Track if refresh is in progress to prevent concurrent refresh attempts
let isRefreshing = false
let failedQueue: any[] = []

const processQueue = (error: any, token: string | null = null) => {
  failedQueue.forEach(prom => {
    if (error) {
      prom.reject(error)
    } else {
      prom.resolve(token)
    }
  })

  failedQueue = []
}

// Setup axios interceptors for token refresh
axios.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config

    // Skip interceptor for auth endpoints (login, refresh, logout)
    if (originalRequest.url?.includes('/api/v1/auth/')) {
      return Promise.reject(error)
    }

    // If token expired and we haven't retried yet
    if (error.response?.status === 401 && !originalRequest._retry) {
      if (isRefreshing) {
        // If refresh is already in progress, queue this request
        return new Promise((resolve, reject) => {
          failedQueue.push({ resolve, reject })
        }).then(token => {
          originalRequest.headers['Authorization'] = 'Bearer ' + token
          return axios(originalRequest)
        }).catch(err => {
          return Promise.reject(err)
        })
      }

      originalRequest._retry = true
      isRefreshing = true

      try {
        // Try to refresh token
        await authStore.refreshToken()
        isRefreshing = false
        processQueue(null, authStore.tokens?.accessToken || null)

        // Retry original request with new token
        if (authStore.tokens?.accessToken) {
          originalRequest.headers['Authorization'] = 'Bearer ' + authStore.tokens.accessToken
        }
        return axios(originalRequest)
      } catch (refreshError) {
        // Refresh failed, force logout
        isRefreshing = false
        processQueue(refreshError, null)

        console.error('[main.ts] Token refresh failed, forcing logout')
        await authStore.logout()

        // Force redirect to login
        if (router.currentRoute.value.path !== '/login') {
          window.location.href = '/login'
        }

        return Promise.reject(refreshError)
      }
    }

    return Promise.reject(error)
  }
)

// Mount app
app.mount('#app')

// Initialize modules (load built-in modules)
async function initializeModules() {
  try {
    // Register built-in modules
    await moduleLoader.registerModule(emailModule)

    console.log('Module system initialized')
    console.log('Loaded modules:', moduleLoader.getLoadedModules().join(', '))
  } catch (error) {
    console.error('Failed to initialize modules:', error)
  }
}

initializeModules()
