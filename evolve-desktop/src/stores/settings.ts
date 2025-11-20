/**
 * Settings Store - Manages user settings with database persistence
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import settingsService, { type SettingsMap } from '@/services/settingsService'

export const useSettingsStore = defineStore('settings', () => {
  // State
  const settings = ref<SettingsMap>({})
  const isLoaded = ref(false)
  const isLoading = ref(false)

  // Computed
  const theme = computed(() => settings.value.theme || 'light')
  const language = computed(() => settings.value.language || 'en')
  const notificationsEnabled = computed(() => settings.value.notifications_enabled !== false)

  // Actions

  /**
   * Initialize settings by loading from API
   */
  async function initialize(): Promise<void> {
    if (isLoaded.value) return

    isLoading.value = true
    try {
      const data = await settingsService.getAll(undefined, true)
      settings.value = data
      isLoaded.value = true
      console.log('[SettingsStore] Settings loaded:', Object.keys(data).length, 'settings')
    } catch (error) {
      console.error('[SettingsStore] Failed to load settings:', error)
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Get a setting by key
   */
  function get(key: string, defaultValue: any = null): any {
    return settings.value[key] ?? defaultValue
  }

  /**
   * Set a single setting
   */
  async function set(key: string, value: any, category?: string): Promise<boolean> {
    const success = await settingsService.set(key, value, category)
    if (success) {
      settings.value[key] = value
    }
    return success
  }

  /**
   * Set multiple settings at once
   */
  async function setBulk(newSettings: SettingsMap, category?: string): Promise<boolean> {
    const success = await settingsService.setBulk(newSettings, category)
    if (success) {
      Object.assign(settings.value, newSettings)
    }
    return success
  }

  /**
   * Delete a setting
   */
  async function remove(key: string): Promise<boolean> {
    const success = await settingsService.delete(key)
    if (success) {
      delete settings.value[key]
    }
    return success
  }

  /**
   * Reset all settings
   */
  async function resetAll(): Promise<boolean> {
    const success = await settingsService.resetAll()
    if (success) {
      settings.value = {}
    }
    return success
  }

  /**
   * Refresh settings from server
   */
  async function refresh(): Promise<void> {
    isLoading.value = true
    try {
      const data = await settingsService.getAll(undefined, true)
      settings.value = data
    } catch (error) {
      console.error('[SettingsStore] Failed to refresh settings:', error)
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Set theme
   */
  async function setTheme(newTheme: 'light' | 'dark'): Promise<boolean> {
    return set('theme', newTheme, 'ui')
  }

  /**
   * Set language
   */
  async function setLanguage(lang: string): Promise<boolean> {
    return set('language', lang, 'ui')
  }

  /**
   * Toggle notifications
   */
  async function toggleNotifications(): Promise<boolean> {
    const newValue = !notificationsEnabled.value
    return set('notifications_enabled', newValue, 'notifications')
  }

  return {
    // State
    settings,
    isLoaded,
    isLoading,

    // Computed
    theme,
    language,
    notificationsEnabled,

    // Actions
    initialize,
    get,
    set,
    setBulk,
    remove,
    resetAll,
    refresh,
    setTheme,
    setLanguage,
    toggleNotifications
  }
})
