import type { Router } from 'vue-router'
import type { ModulePlugin } from '../types/module'
import { moduleRegistry } from './module-registry'

/**
 * Module Loader - Handles dynamic loading and initialization of modules
 */
export class ModuleLoader {
  private router: Router | null = null
  private loadedModules: Set<string> = new Set()

  /**
   * Set the Vue Router instance
   * @param router Vue Router instance
   */
  setRouter(router: Router): void {
    this.router = router
  }

  /**
   * Load a module dynamically
   * @param moduleId Module ID
   * @param modulePath Path to module entry file
   */
  async loadModule(moduleId: string, modulePath: string): Promise<void> {
    if (this.loadedModules.has(moduleId)) {
      console.warn(`Module '${moduleId}' is already loaded`)
      return
    }

    try {
      // Dynamically import module
      const moduleExport = await import(/* @vite-ignore */ modulePath)
      const plugin: ModulePlugin = moduleExport.default || moduleExport

      // Validate plugin structure
      this.validatePlugin(plugin)

      // Register module
      await moduleRegistry.register(plugin)

      // Initialize module
      await moduleRegistry.initialize(moduleId)

      // Register routes
      this.registerModuleRoutes(plugin)

      this.loadedModules.add(moduleId)

      console.log(`Module '${moduleId}' loaded successfully from ${modulePath}`)
    } catch (error) {
      console.error(`Failed to load module '${moduleId}':`, error)
      throw error
    }
  }

  /**
   * Load multiple modules
   * @param modules Array of module configurations
   */
  async loadModules(
    modules: Array<{ id: string; path: string }>
  ): Promise<void> {
    const errors: Array<{ id: string; error: Error }> = []

    for (const { id, path } of modules) {
      try {
        await this.loadModule(id, path)
      } catch (error) {
        errors.push({ id, error: error as Error })
      }
    }

    if (errors.length > 0) {
      console.error('Some modules failed to load:', errors)
    }
  }

  /**
   * Register a module that's already imported
   * @param plugin Module plugin instance
   */
  async registerModule(plugin: ModulePlugin): Promise<void> {
    const moduleId = plugin.metadata.id

    if (this.loadedModules.has(moduleId)) {
      console.warn(`Module '${moduleId}' is already loaded`)
      return
    }

    // Validate plugin structure
    this.validatePlugin(plugin)

    // Register module
    await moduleRegistry.register(plugin)

    // Initialize module
    await moduleRegistry.initialize(moduleId)

    // Register routes
    this.registerModuleRoutes(plugin)

    this.loadedModules.add(moduleId)

    console.log(`Module '${moduleId}' registered successfully`)
  }

  /**
   * Validate plugin structure
   * @param plugin Module plugin to validate
   */
  private validatePlugin(plugin: ModulePlugin): void {
    if (!plugin.metadata) {
      throw new Error('Module plugin must have metadata')
    }

    const required = ['id', 'name', 'version', 'description', 'icon', 'category']
    for (const field of required) {
      if (!(field in plugin.metadata)) {
        throw new Error(`Module metadata must have '${field}' field`)
      }
    }

    if (typeof plugin.install !== 'function') {
      throw new Error('Module plugin must have install() method')
    }

    if (typeof plugin.getMainView !== 'function') {
      throw new Error('Module plugin must have getMainView() method')
    }

    if (typeof plugin.getRoutes !== 'function') {
      throw new Error('Module plugin must have getRoutes() method')
    }

    if (typeof plugin.getSchema !== 'function') {
      throw new Error('Module plugin must have getSchema() method')
    }
  }

  /**
   * Register module routes with Vue Router
   * @param plugin Module plugin
   */
  private registerModuleRoutes(plugin: ModulePlugin): void {
    if (!this.router) {
      console.warn('Router not set. Call setRouter() first.')
      return
    }

    const routes = plugin.getRoutes()

    for (const route of routes) {
      // Add module context to route meta
      const routeWithMeta = {
        ...route,
        meta: {
          ...route.meta,
          moduleId: plugin.metadata.id,
          moduleName: plugin.metadata.name,
        },
      }

      this.router.addRoute(routeWithMeta)
    }

    console.log(
      `Registered ${routes.length} route(s) for module '${plugin.metadata.id}'`
    )
  }

  /**
   * Unload a module
   * @param moduleId Module ID
   */
  async unloadModule(moduleId: string): Promise<void> {
    if (!this.loadedModules.has(moduleId)) {
      console.warn(`Module '${moduleId}' is not loaded`)
      return
    }

    // Unregister from registry
    await moduleRegistry.unregister(moduleId)

    // Remove routes (Vue Router doesn't have removeRoute in all versions)
    // This would require manual route management or router recreation

    this.loadedModules.delete(moduleId)

    console.log(`Module '${moduleId}' unloaded`)
  }

  /**
   * Get loaded module IDs
   */
  getLoadedModules(): string[] {
    return Array.from(this.loadedModules)
  }

  /**
   * Check if a module is loaded
   * @param moduleId Module ID
   */
  isModuleLoaded(moduleId: string): boolean {
    return this.loadedModules.has(moduleId)
  }
}

// Export singleton instance
export const moduleLoader = new ModuleLoader()
