import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import axios from 'axios'
import { eventBus, EVENTS } from '@/core/event-bus'

export interface User {
  id: string
  email: string
  name: string
  avatar?: string
  role: string
}

export interface AuthTokens {
  accessToken: string
  refreshToken: string
  expiresAt: number
}

export const useAuthStore = defineStore('auth', () => {
  // State
  const user = ref<User | null>(null)
  const tokens = ref<AuthTokens | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const isAuthenticated = computed(() => !!user.value && !!tokens.value)
  const isTokenExpired = computed(() => {
    if (!tokens.value) return true
    return Date.now() >= tokens.value.expiresAt
  })

  // Actions
  async function login(email: string, password: string) {
    loading.value = true
    error.value = null

    try {
      const response = await axios.post('/api/v1/auth/login', {
        email,
        password,
      })

      const { user: userData, access_token, refresh_token, expires_in } = response.data

      // Store user data
      user.value = {
        id: userData.id,
        email: userData.email,
        name: userData.name,
        avatar: userData.avatar,
        role: userData.role,
      }

      // Store tokens
      tokens.value = {
        accessToken: access_token,
        refreshToken: refresh_token,
        expiresAt: Date.now() + expires_in * 1000,
      }

      // Set default authorization header
      setAuthHeader(access_token)

      // Persist to localStorage
      persistAuth()

      // Emit event
      eventBus.emit(EVENTS.USER_LOGGED_IN, { user: user.value })

      return user.value
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Login failed'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function logout() {
    console.log('[AuthStore] Logout started')
    try {
      // Call logout endpoint if authenticated
      if (tokens.value?.accessToken) {
        console.log('[AuthStore] Calling logout API endpoint...')
        await axios.post('/api/v1/auth/logout')
        console.log('[AuthStore] Logout API call successful')
      }
    } catch (err) {
      console.error('[AuthStore] Logout API error:', err)
    }

    // Clear state
    console.log('[AuthStore] Clearing auth state...')
    user.value = null
    tokens.value = null
    error.value = null

    // Clear localStorage
    clearAuth()
    console.log('[AuthStore] localStorage cleared')

    // Clear auth header
    delete axios.defaults.headers.common['Authorization']
    console.log('[AuthStore] Auth header cleared')

    // Emit event
    eventBus.emit(EVENTS.USER_LOGGED_OUT)
    console.log('[AuthStore] Logout complete')
  }

  async function refreshToken() {
    if (!tokens.value?.refreshToken) {
      throw new Error('No refresh token available')
    }

    try {
      const response = await axios.post('/api/v1/auth/refresh', {
        refresh_token: tokens.value.refreshToken,
      })

      const { access_token, refresh_token, expires_in } = response.data

      // Update tokens
      tokens.value = {
        accessToken: access_token,
        refreshToken: refresh_token,
        expiresAt: Date.now() + expires_in * 1000,
      }

      // Update auth header
      setAuthHeader(access_token)

      // Persist to localStorage
      persistAuth()

      // Emit event
      eventBus.emit(EVENTS.TOKEN_REFRESHED)

      return tokens.value
    } catch (err) {
      // Refresh failed, logout
      await logout()
      throw err
    }
  }

  function setAuthHeader(token: string) {
    axios.defaults.headers.common['Authorization'] = `Bearer ${token}`
  }

  function persistAuth() {
    if (user.value && tokens.value) {
      localStorage.setItem('user', JSON.stringify(user.value))
      localStorage.setItem('tokens', JSON.stringify(tokens.value))
    }
  }

  function clearAuth() {
    localStorage.removeItem('user')
    localStorage.removeItem('tokens')
  }

  function restoreAuth() {
    const storedUser = localStorage.getItem('user')
    const storedTokens = localStorage.getItem('tokens')

    if (storedUser && storedTokens) {
      try {
        user.value = JSON.parse(storedUser)
        tokens.value = JSON.parse(storedTokens)

        // Check if token is expired
        if (!isTokenExpired.value) {
          setAuthHeader(tokens.value!.accessToken)
        } else {
          // Try to refresh
          refreshToken().catch(() => {
            clearAuth()
          })
        }
      } catch (err) {
        console.error('Failed to restore auth:', err)
        clearAuth()
      }
    }
  }

  async function updateProfile(data: Partial<User>) {
    if (!user.value) return

    try {
      const response = await axios.patch('/api/v1/auth/profile', data)
      user.value = { ...user.value, ...response.data }
      persistAuth()
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Profile update failed'
      throw err
    }
  }

  return {
    // State
    user,
    tokens,
    loading,
    error,
    // Computed
    isAuthenticated,
    isTokenExpired,
    // Actions
    login,
    logout,
    refreshToken,
    restoreAuth,
    updateProfile,
  }
})
