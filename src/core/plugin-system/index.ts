/**
 * Plugin System - Core module management system
 */

export { moduleRegistry, ModuleRegistry } from './module-registry'
export { moduleLoader, ModuleLoader } from './module-loader'
export * from '../types/module'
export { eventBus, EventBus, EVENTS, type EventName } from '../event-bus'
export { serviceRegistry, ServiceRegistry, SERVICES, type ServiceName } from '../service-registry'
