import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from './router'
import App from './App.vue'
import './style.css'

// Import stores
import { useAuthStore } from './stores/auth'
import { useAppStore } from './stores/app'

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

// Restore auth state from localStorage
authStore.restoreAuth()

// Restore theme
appStore.restoreTheme()

// Set router for module loader
moduleLoader.setRouter(router)

// Setup axios interceptors for token refresh
axios.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config

    // If token expired and we haven't retried yet
    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true

      try {
        // Try to refresh token
        await authStore.refreshToken()

        // Retry original request
        return axios(originalRequest)
      } catch (refreshError) {
        // Refresh failed, logout
        await authStore.logout()
        router.push('/login')
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
    console.log('Loaded modules:', moduleLoader.getEnabledModules().map(m => m.name).join(', '))
  } catch (error) {
    console.error('Failed to initialize modules:', error)
  }
}

initializeModules()
