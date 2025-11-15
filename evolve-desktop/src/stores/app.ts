import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { eventBus, EVENTS } from '@/core/event-bus'

export type Theme = 'light' | 'dark' | 'auto'

export interface Notification {
  id: string
  type: 'info' | 'success' | 'warning' | 'error'
  title: string
  message: string
  timestamp: Date
  read: boolean
  action?: {
    label: string
    callback: () => void
  }
}

export const useAppStore = defineStore('app', () => {
  // State
  const theme = ref<Theme>('auto')
  const sidebarOpen = ref(true)
  const currentModule = ref<string | null>(null)
  const notifications = ref<Notification[]>([])
  const loading = ref(false)
  const searchQuery = ref('')
  const searchOpen = ref(false)

  // Computed
  const unreadNotifications = computed(() =>
    notifications.value.filter((n) => !n.read).length
  )

  const effectiveTheme = computed(() => {
    if (theme.value === 'auto') {
      // Check system preference
      if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
        return 'dark'
      }
      return 'light'
    }
    return theme.value
  })

  // Actions
  function setTheme(newTheme: Theme) {
    theme.value = newTheme
    applyTheme()
    persistTheme()
    eventBus.emit(EVENTS.THEME_CHANGED, { theme: newTheme })
  }

  function applyTheme() {
    const htmlElement = document.documentElement

    if (effectiveTheme.value === 'dark') {
      htmlElement.setAttribute('data-theme', 'dark')
      htmlElement.classList.add('dark')
    } else {
      htmlElement.setAttribute('data-theme', 'light')
      htmlElement.classList.remove('dark')
    }
  }

  function persistTheme() {
    localStorage.setItem('theme', theme.value)
  }

  function restoreTheme() {
    const storedTheme = localStorage.getItem('theme') as Theme
    if (storedTheme) {
      theme.value = storedTheme
    }
    applyTheme()
  }

  function toggleSidebar() {
    sidebarOpen.value = !sidebarOpen.value
    eventBus.emit(EVENTS.SIDEBAR_TOGGLED, { open: sidebarOpen.value })
  }

  function setCurrentModule(moduleId: string | null) {
    currentModule.value = moduleId
  }

  function addNotification(
    notification: Omit<Notification, 'id' | 'timestamp' | 'read'>
  ) {
    const newNotification: Notification = {
      ...notification,
      id: `notif-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      timestamp: new Date(),
      read: false,
    }

    notifications.value.unshift(newNotification)

    // Limit to 100 notifications
    if (notifications.value.length > 100) {
      notifications.value = notifications.value.slice(0, 100)
    }

    eventBus.emit(EVENTS.NOTIFICATION_CREATED, newNotification)

    return newNotification.id
  }

  function markNotificationRead(id: string) {
    const notification = notifications.value.find((n) => n.id === id)
    if (notification) {
      notification.read = true
      eventBus.emit(EVENTS.NOTIFICATION_READ, { id })
    }
  }

  function markAllNotificationsRead() {
    notifications.value.forEach((n) => {
      n.read = true
    })
  }

  function dismissNotification(id: string) {
    const index = notifications.value.findIndex((n) => n.id === id)
    if (index !== -1) {
      notifications.value.splice(index, 1)
      eventBus.emit(EVENTS.NOTIFICATION_DISMISSED, { id })
    }
  }

  function clearNotifications() {
    notifications.value = []
  }

  function setLoading(value: boolean) {
    loading.value = value
  }

  function setSearchQuery(query: string) {
    searchQuery.value = query
  }

  function toggleSearch() {
    searchOpen.value = !searchOpen.value
  }

  function openSearch() {
    searchOpen.value = true
  }

  function closeSearch() {
    searchOpen.value = false
    searchQuery.value = ''
  }

  // Toast notifications (auto-dismiss after timeout)
  function showToast(
    type: 'info' | 'success' | 'warning' | 'error',
    message: string,
    duration: number = 3000
  ) {
    const id = addNotification({
      type,
      title: type.charAt(0).toUpperCase() + type.slice(1),
      message,
    })

    // Auto-dismiss
    setTimeout(() => {
      dismissNotification(id)
    }, duration)

    return id
  }

  return {
    // State
    theme,
    sidebarOpen,
    currentModule,
    notifications,
    loading,
    searchQuery,
    searchOpen,
    // Computed
    unreadNotifications,
    effectiveTheme,
    // Actions
    setTheme,
    applyTheme,
    restoreTheme,
    toggleSidebar,
    setCurrentModule,
    addNotification,
    markNotificationRead,
    markAllNotificationsRead,
    dismissNotification,
    clearNotifications,
    setLoading,
    setSearchQuery,
    toggleSearch,
    openSearch,
    closeSearch,
    showToast,
  }
})
