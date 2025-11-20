import axios from 'axios'

export interface DesktopModule {
  key: string
  name: string
  description: string
  icon: string
  category: string
  isComingSoon: boolean
  displayOrder: number
}

class ModulesService {
  private cache: DesktopModule[] | null = null
  private cacheTimestamp: number = 0
  private readonly CACHE_DURATION = 5 * 60 * 1000 // 5 minutes

  /**
   * Get all enabled modules from API
   */
  async getAvailableModules(forceRefresh = false): Promise<DesktopModule[]> {
    const now = Date.now()

    // Return cached data if available and not expired
    if (!forceRefresh && this.cache && (now - this.cacheTimestamp < this.CACHE_DURATION)) {
      return this.cache
    }

    try {
      const response = await axios.get<{ success: boolean; modules: DesktopModule[] }>(
        '/api/v1/desktop/modules'
      )

      if (response.data.success) {
        this.cache = response.data.modules
        this.cacheTimestamp = now
        return this.cache
      }

      throw new Error('Failed to fetch modules')
    } catch (error: any) {
      console.error('[ModulesService] Error fetching available modules:', error.message)

      // Return cached data if available, even if expired
      if (this.cache) {
        console.warn('[ModulesService] Using stale cache due to API error')
        return this.cache
      }

      // Return default modules as fallback
      return this.getDefaultModules()
    }
  }

  /**
   * Get modules grouped by category
   */
  async getModulesByCategory(forceRefresh = false): Promise<Record<string, DesktopModule[]>> {
    const modules = await this.getAvailableModules(forceRefresh)

    const grouped: Record<string, DesktopModule[]> = {}

    for (const module of modules) {
      if (!grouped[module.category]) {
        grouped[module.category] = []
      }
      grouped[module.category].push(module)
    }

    return grouped
  }

  /**
   * Check if a specific module is enabled
   */
  async isModuleEnabled(moduleKey: string): Promise<boolean> {
    const modules = await this.getAvailableModules()
    return modules.some(m => m.key === moduleKey)
  }

  /**
   * Clear cache (call this after settings change)
   */
  clearCache(): void {
    this.cache = null
    this.cacheTimestamp = 0
  }

  /**
   * Default modules fallback (if API fails)
   */
  private getDefaultModules(): DesktopModule[] {
    return [
      {
        key: 'email',
        name: 'Email',
        description: 'Full-featured email client with IMAP/SMTP support',
        icon: 'mail',
        category: 'communication',
        isComingSoon: false,
        displayOrder: 1,
      },
      {
        key: 'chat',
        name: 'Team Chat',
        description: 'Real-time messaging and collaboration',
        icon: 'message-square',
        category: 'communication',
        isComingSoon: false,
        displayOrder: 2,
      },
      {
        key: 'settings',
        name: 'Settings',
        description: 'Configure your application preferences',
        icon: 'settings',
        category: 'tools',
        isComingSoon: false,
        displayOrder: 99,
      },
    ]
  }
}

export const modulesService = new ModulesService()
