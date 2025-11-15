/**
 * Service Registry - Dependency Injection container for shared services
 * Allows modules to register and access shared services
 */

type ServiceFactory<T = any> = () => T | Promise<T>
type ServiceInstance<T = any> = T

interface ServiceDefinition<T = any> {
  factory: ServiceFactory<T>
  instance?: ServiceInstance<T>
  singleton: boolean
}

class ServiceRegistry {
  private services: Map<string, ServiceDefinition> = new Map()

  /**
   * Register a service
   * @param name Service name
   * @param factory Factory function to create service instance
   * @param singleton Whether to create single instance (default: true)
   */
  register<T>(
    name: string,
    factory: ServiceFactory<T>,
    singleton: boolean = true
  ): void {
    if (this.services.has(name)) {
      console.warn(`Service '${name}' is already registered. Overwriting.`)
    }

    this.services.set(name, {
      factory,
      singleton,
    })
  }

  /**
   * Get a service instance
   * @param name Service name
   * @returns Service instance
   */
  async get<T>(name: string): Promise<T> {
    const definition = this.services.get(name)

    if (!definition) {
      throw new Error(`Service '${name}' not found in registry`)
    }

    // Return existing instance for singletons
    if (definition.singleton && definition.instance) {
      return definition.instance as T
    }

    // Create new instance
    const instance = await definition.factory()

    // Store singleton instance
    if (definition.singleton) {
      definition.instance = instance
    }

    return instance as T
  }

  /**
   * Get a service synchronously (only works if already instantiated or factory is sync)
   * @param name Service name
   * @returns Service instance or undefined
   */
  getSync<T>(name: string): T | undefined {
    const definition = this.services.get(name)

    if (!definition) {
      return undefined
    }

    // Return existing instance
    if (definition.instance) {
      return definition.instance as T
    }

    // Try to create synchronously
    try {
      const instance = definition.factory()
      if (instance instanceof Promise) {
        console.warn(`Service '${name}' factory is async. Use get() instead.`)
        return undefined
      }

      if (definition.singleton) {
        definition.instance = instance
      }

      return instance as T
    } catch (error) {
      console.error(`Error creating service '${name}':`, error)
      return undefined
    }
  }

  /**
   * Check if a service is registered
   * @param name Service name
   */
  has(name: string): boolean {
    return this.services.has(name)
  }

  /**
   * Unregister a service
   * @param name Service name
   */
  unregister(name: string): void {
    this.services.delete(name)
  }

  /**
   * Clear all services
   */
  clear(): void {
    this.services.clear()
  }

  /**
   * Get all registered service names
   */
  getServiceNames(): string[] {
    return Array.from(this.services.keys())
  }
}

// Export singleton instance
export const serviceRegistry = new ServiceRegistry()

// Export class for testing
export { ServiceRegistry }

// Common service names
export const SERVICES = {
  HTTP_CLIENT: 'http-client',
  AUTH_SERVICE: 'auth-service',
  DATABASE: 'database',
  SYNC_ENGINE: 'sync-engine',
  NOTIFICATION_SERVICE: 'notification-service',
  SEARCH_SERVICE: 'search-service',
  STORAGE_SERVICE: 'storage-service',
  ENCRYPTION_SERVICE: 'encryption-service',
  LOGGER: 'logger',
  ANALYTICS: 'analytics',
} as const

export type ServiceName = typeof SERVICES[keyof typeof SERVICES]
