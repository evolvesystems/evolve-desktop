/**
 * Settings Service for EvolveApp Desktop
 *
 * Manages user settings stored in the remote database.
 * Provides methods to get, set, and sync user preferences.
 */

import axios from 'axios'

export interface Setting {
  key: string
  value: any
  category?: string
  scope?: 'user' | 'tenant' | 'global'
}

export interface SettingsMap {
  [key: string]: any
}

class SettingsService {
  private baseUrl = '/api/v1/desktop/settings'
  private cache: SettingsMap = {}
  private cacheTimestamp: number = 0
  private cacheDuration = 5 * 60 * 1000 // 5 minutes

  /**
   * Get all settings for the current user
   */
  async getAll(category?: string, forceRefresh = false): Promise<SettingsMap> {
    const now = Date.now()

    // Return cached settings if still valid
    if (!forceRefresh && this.cache && (now - this.cacheTimestamp) < this.cacheDuration) {
      return this.cache
    }

    try {
      const params = category ? { category } : {}
      const response = await axios.get(this.baseUrl, { params })

      this.cache = response.data.settings || {}
      this.cacheTimestamp = now

      return this.cache
    } catch (error) {
      console.error('[SettingsService] Failed to fetch settings:', error)
      // Return cached data on error if available
      return this.cache || {}
    }
  }

  /**
   * Get a specific setting by key
   */
  async get(key: string, defaultValue: any = null): Promise<any> {
    try {
      const response = await axios.get(`${this.baseUrl}/${key}`)
      return response.data.value
    } catch (error: any) {
      if (error.response?.status === 404) {
        return defaultValue
      }
      console.error(`[SettingsService] Failed to fetch setting '${key}':`, error)
      return defaultValue
    }
  }

  /**
   * Set a single setting
   */
  async set(key: string, value: any, category?: string, scope: 'user' | 'tenant' | 'global' = 'user'): Promise<boolean> {
    try {
      await axios.post(`${this.baseUrl}/${key}`, {
        value,
        category,
        scope
      })

      // Update cache
      this.cache[key] = value

      return true
    } catch (error) {
      console.error(`[SettingsService] Failed to set setting '${key}':`, error)
      return false
    }
  }

  /**
   * Bulk set multiple settings
   */
  async setBulk(settings: SettingsMap, category?: string, scope: 'user' | 'tenant' | 'global' = 'user'): Promise<boolean> {
    try {
      await axios.post(this.baseUrl, {
        settings,
        category,
        scope
      })

      // Update cache
      Object.assign(this.cache, settings)

      return true
    } catch (error) {
      console.error('[SettingsService] Failed to bulk set settings:', error)
      return false
    }
  }

  /**
   * Delete a setting
   */
  async delete(key: string): Promise<boolean> {
    try {
      await axios.delete(`${this.baseUrl}/${key}`)

      // Remove from cache
      delete this.cache[key]

      return true
    } catch (error) {
      console.error(`[SettingsService] Failed to delete setting '${key}':`, error)
      return false
    }
  }

  /**
   * Reset all settings
   */
  async resetAll(): Promise<boolean> {
    try {
      await axios.delete(this.baseUrl)

      // Clear cache
      this.cache = {}
      this.cacheTimestamp = 0

      return true
    } catch (error) {
      console.error('[SettingsService] Failed to reset settings:', error)
      return false
    }
  }

  /**
   * Clear local cache (force refresh on next get)
   */
  clearCache(): void {
    this.cache = {}
    this.cacheTimestamp = 0
  }

  /**
   * Get cached settings without making API call
   */
  getCached(): SettingsMap {
    return { ...this.cache }
  }

  /**
   * Initialize settings by loading from API
   */
  async initialize(): Promise<void> {
    console.log('[SettingsService] Initializing settings...')
    try {
      await this.getAll(undefined, true)
      console.log('[SettingsService] Settings initialized successfully')
    } catch (error) {
      console.error('[SettingsService] Failed to initialize settings:', error)
    }
  }
}

// Export singleton instance
export const settingsService = new SettingsService()
export default settingsService
