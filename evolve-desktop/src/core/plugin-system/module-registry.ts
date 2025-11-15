import type { ModulePlugin, ModuleInstallation } from '../types/module'
import { eventBus, EVENTS } from '../event-bus'

/**
 * Module Registry - Manages module registration and lifecycle
 */
class ModuleRegistry {
  private modules: Map<string, ModulePlugin> = new Map()
  private installations: Map<string, ModuleInstallation> = new Map()
  private initialized: Set<string> = new Set()

  /**
   * Register a module plugin
   * @param plugin Module plugin instance
   */
  async register(plugin: ModulePlugin): Promise<void> {
    const { id, version, requiredModules } = plugin.metadata

    // Check if already registered
    if (this.modules.has(id)) {
      throw new Error(`Module '${id}' is already registered`)
    }

    // Check required modules
    if (requiredModules) {
      for (const requiredId of requiredModules) {
        if (!this.modules.has(requiredId)) {
          throw new Error(
            `Module '${id}' requires '${requiredId}' which is not installed`
          )
        }
      }
    }

    // Register module
    this.modules.set(id, plugin)

    // Create installation record
    const installation: ModuleInstallation = {
      moduleId: id,
      version,
      installedAt: new Date(),
      enabled: true,
      settings: {
        enabled: true,
        config: {},
      },
    }
    this.installations.set(id, installation)

    // Emit event
    eventBus.emit(EVENTS.MODULE_INSTALLED, { moduleId: id, version })

    console.log(`Module '${id}' v${version} registered successfully`)
  }

  /**
   * Unregister a module
   * @param moduleId Module ID
   */
  async unregister(moduleId: string): Promise<void> {
    const plugin = this.modules.get(moduleId)
    if (!plugin) {
      throw new Error(`Module '${moduleId}' not found`)
    }

    // Check if other modules depend on this
    for (const [id, otherPlugin] of this.modules) {
      if (id !== moduleId && otherPlugin.metadata.requiredModules?.includes(moduleId)) {
        throw new Error(
          `Cannot unregister '${moduleId}' because '${id}' depends on it`
        )
      }
    }

    // Call uninstall if available
    if (plugin.uninstall) {
      await plugin.uninstall()
    }

    // Remove from registry
    this.modules.delete(moduleId)
    this.installations.delete(moduleId)
    this.initialized.delete(moduleId)

    // Emit event
    eventBus.emit(EVENTS.MODULE_UNINSTALLED, { moduleId })

    console.log(`Module '${moduleId}' unregistered successfully`)
  }

  /**
   * Initialize a module (call install() method)
   * @param moduleId Module ID
   */
  async initialize(moduleId: string): Promise<void> {
    if (this.initialized.has(moduleId)) {
      console.warn(`Module '${moduleId}' is already initialized`)
      return
    }

    const plugin = this.modules.get(moduleId)
    if (!plugin) {
      throw new Error(`Module '${moduleId}' not found`)
    }

    // Initialize required modules first
    if (plugin.metadata.requiredModules) {
      for (const requiredId of plugin.metadata.requiredModules) {
        if (!this.initialized.has(requiredId)) {
          await this.initialize(requiredId)
        }
      }
    }

    // Call install method
    await plugin.install()
    this.initialized.add(moduleId)

    console.log(`Module '${moduleId}' initialized successfully`)
  }

  /**
   * Get a registered module
   * @param moduleId Module ID
   */
  getModule(moduleId: string): ModulePlugin | undefined {
    return this.modules.get(moduleId)
  }

  /**
   * Get module installation info
   * @param moduleId Module ID
   */
  getInstallation(moduleId: string): ModuleInstallation | undefined {
    return this.installations.get(moduleId)
  }

  /**
   * Get all registered modules
   */
  getAllModules(): ModulePlugin[] {
    return Array.from(this.modules.values())
  }

  /**
   * Get all module installations
   */
  getAllInstallations(): ModuleInstallation[] {
    return Array.from(this.installations.values())
  }

  /**
   * Check if a module is registered
   * @param moduleId Module ID
   */
  isRegistered(moduleId: string): boolean {
    return this.modules.has(moduleId)
  }

  /**
   * Check if a module is initialized
   * @param moduleId Module ID
   */
  isInitialized(moduleId: string): boolean {
    return this.initialized.has(moduleId)
  }

  /**
   * Enable a module
   * @param moduleId Module ID
   */
  async enable(moduleId: string): Promise<void> {
    const installation = this.installations.get(moduleId)
    if (!installation) {
      throw new Error(`Module '${moduleId}' not found`)
    }

    if (installation.enabled) {
      return
    }

    installation.enabled = true
    installation.settings.enabled = true

    // Initialize if not already initialized
    if (!this.initialized.has(moduleId)) {
      await this.initialize(moduleId)
    }

    eventBus.emit(EVENTS.MODULE_ENABLED, { moduleId })
    console.log(`Module '${moduleId}' enabled`)
  }

  /**
   * Disable a module
   * @param moduleId Module ID
   */
  async disable(moduleId: string): Promise<void> {
    const installation = this.installations.get(moduleId)
    if (!installation) {
      throw new Error(`Module '${moduleId}' not found`)
    }

    // Check if other enabled modules depend on this
    for (const [id, otherInstallation] of this.installations) {
      if (id !== moduleId && otherInstallation.enabled) {
        const otherPlugin = this.modules.get(id)
        if (otherPlugin?.metadata.requiredModules?.includes(moduleId)) {
          throw new Error(
            `Cannot disable '${moduleId}' because enabled module '${id}' depends on it`
          )
        }
      }
    }

    installation.enabled = false
    installation.settings.enabled = false

    eventBus.emit(EVENTS.MODULE_DISABLED, { moduleId })
    console.log(`Module '${moduleId}' disabled`)
  }

  /**
   * Update module settings
   * @param moduleId Module ID
   * @param settings New settings
   */
  async updateSettings(moduleId: string, settings: Record<string, any>): Promise<void> {
    const installation = this.installations.get(moduleId)
    const plugin = this.modules.get(moduleId)

    if (!installation || !plugin) {
      throw new Error(`Module '${moduleId}' not found`)
    }

    installation.settings.config = {
      ...installation.settings.config,
      ...settings,
    }

    // Call plugin's settings changed handler if available
    if (plugin.onSettingsChanged) {
      await plugin.onSettingsChanged(installation.settings)
    }

    console.log(`Settings updated for module '${moduleId}'`)
  }

  /**
   * Search across all modules
   * @param query Search query
   */
  async search(query: string): Promise<any[]> {
    const results: any[] = []

    for (const [moduleId, plugin] of this.modules) {
      const installation = this.installations.get(moduleId)

      // Only search in enabled modules
      if (!installation?.enabled || !plugin.search) {
        continue
      }

      try {
        const moduleResults = await plugin.search(query)
        results.push(...moduleResults)
      } catch (error) {
        console.error(`Error searching in module '${moduleId}':`, error)
      }
    }

    // Sort by score if available
    results.sort((a, b) => (b.score || 0) - (a.score || 0))

    return results
  }
}

// Export singleton instance
export const moduleRegistry = new ModuleRegistry()

// Export class for testing
export { ModuleRegistry }
