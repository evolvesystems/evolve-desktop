# EIQ Desktop - Plugin System Architecture

**Version**: 1.0
**Date**: 2025-11-14
**Status**: Production-Ready Specification

---

## 🎯 Overview

The EIQ Desktop plugin system allows modules to be loaded dynamically, installed independently, and work together seamlessly. Each module is a self-contained plugin with its own UI, data, and API integration.

**Key Principles**:
- **Isolation**: Modules don't directly depend on each other
- **Communication**: Shared event bus and service registry
- **Lazy Loading**: Load modules only when needed
- **Hot Reload**: Development mode supports module reloading
- **Versioning**: Module compatibility checks

---

## 📁 Project Structure

```
evolveapp/
├── src/                           # Vue 3 Frontend
│   ├── main.ts                    # App entry point
│   ├── App.vue                    # Root component
│   │
│   ├── core/                      # Core Framework
│   │   ├── plugin-system/
│   │   │   ├── ModuleRegistry.ts       # Module registration
│   │   │   ├── ModuleLoader.ts         # Dynamic loading
│   │   │   ├── ModuleInterface.ts      # Plugin contract
│   │   │   ├── EventBus.ts             # Inter-module events
│   │   │   └── ServiceRegistry.ts      # Shared services
│   │   ├── database/
│   │   │   ├── DatabaseManager.ts      # SQLite wrapper
│   │   │   └── migrations/
│   │   ├── sync/
│   │   │   ├── SyncEngine.ts           # Background sync
│   │   │   └── QueueManager.ts         # Offline queue
│   │   └── api/
│   │       ├── ApiClient.ts            # HTTP client
│   │       └── AuthManager.ts          # JWT handling
│   │
│   ├── modules/                   # Module Plugins
│   │   ├── email/
│   │   │   ├── index.ts                # Plugin entry
│   │   │   ├── module.ts               # Module definition
│   │   │   ├── routes.ts               # Module routes
│   │   │   ├── components/             # Vue components
│   │   │   ├── stores/                 # Pinia stores
│   │   │   ├── services/               # Business logic
│   │   │   ├── database/               # Schema & migrations
│   │   │   └── api/                    # API endpoints
│   │   ├── crm/
│   │   │   └── (same structure)
│   │   ├── calendar/
│   │   │   └── (same structure)
│   │   └── helpdesk/
│   │       └── (same structure)
│   │
│   ├── router/
│   │   └── index.ts               # Vue Router with dynamic routes
│   ├── stores/
│   │   └── modules.ts             # Module state store
│   └── components/
│       ├── ModuleSwitcher.vue
│       └── ModuleContainer.vue
│
├── src-tauri/                     # Rust Backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/              # Tauri commands
│   │   │   ├── mod.rs
│   │   │   ├── database.rs        # Database commands
│   │   │   ├── sync.rs            # Sync commands
│   │   │   └── modules.rs         # Module management
│   │   ├── database/
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs      # SQLite connection pool
│   │   │   ├── migrations.rs      # Migration runner
│   │   │   └── schema.rs          # Table definitions
│   │   ├── sync/
│   │   │   ├── mod.rs
│   │   │   ├── engine.rs          # Sync orchestrator
│   │   │   └── queue.rs           # Offline queue processor
│   │   └── modules/
│   │       ├── mod.rs
│   │       ├── loader.rs          # Module metadata
│   │       └── registry.rs        # Installed modules
│   └── Cargo.toml
│
└── package.json
```

---

## 🔌 Plugin Interface (TypeScript)

### Core Module Interface

```typescript
// src/core/plugin-system/ModuleInterface.ts

import { Component, RouteRecordRaw } from 'vue';
import { Store } from 'pinia';

/**
 * Module metadata
 */
export interface ModuleMetadata {
  id: string;                    // Unique identifier (e.g., 'email', 'crm')
  name: string;                  // Display name (e.g., 'Email Manager')
  version: string;               // Semantic version (e.g., '1.0.0')
  description: string;           // Short description
  icon: string;                  // Icon identifier (heroicons name)
  category: ModuleCategory;      // Category for organization
  dependencies?: string[];       // Module IDs this depends on
  minCoreVersion: string;        // Minimum core version required
  author?: string;
  license?: string;
}

export enum ModuleCategory {
  CORE = 'core',
  BUSINESS = 'business',
  PRODUCTIVITY = 'productivity',
  COMMUNICATION = 'communication',
  ANALYTICS = 'analytics',
  ADMIN = 'admin'
}

/**
 * Module lifecycle states
 */
export enum ModuleState {
  UNINSTALLED = 'uninstalled',
  INSTALLED = 'installed',
  ACTIVE = 'active',
  INACTIVE = 'inactive',
  ERROR = 'error'
}

/**
 * Database schema for a module
 */
export interface ModuleSchema {
  tables: TableDefinition[];
  indexes?: IndexDefinition[];
  migrations?: Migration[];
}

export interface TableDefinition {
  name: string;
  columns: ColumnDefinition[];
  primaryKey?: string[];
  foreignKeys?: ForeignKeyDefinition[];
}

export interface ColumnDefinition {
  name: string;
  type: 'TEXT' | 'INTEGER' | 'REAL' | 'BLOB' | 'DATETIME' | 'BOOLEAN';
  nullable?: boolean;
  unique?: boolean;
  default?: any;
}

export interface ForeignKeyDefinition {
  columns: string[];
  references: {
    table: string;
    columns: string[];
  };
  onDelete?: 'CASCADE' | 'SET NULL' | 'RESTRICT';
  onUpdate?: 'CASCADE' | 'SET NULL' | 'RESTRICT';
}

export interface IndexDefinition {
  name: string;
  table: string;
  columns: string[];
  unique?: boolean;
}

export interface Migration {
  version: number;
  up: string;      // SQL to apply migration
  down: string;    // SQL to rollback migration
}

/**
 * API endpoint configuration
 */
export interface ApiEndpoint {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
  path: string;
  handler: (params: any) => Promise<any>;
  requiresAuth?: boolean;
  rateLimit?: number;
}

/**
 * Module settings schema
 */
export interface ModuleSettings {
  [key: string]: any;
}

/**
 * Search result from module
 */
export interface SearchResult {
  id: string | number;
  type: string;
  title: string;
  subtitle?: string;
  description?: string;
  icon?: string;
  url?: string;
  relevance: number;  // 0-1 score
  metadata?: Record<string, any>;
}

/**
 * Notification from module
 */
export interface ModuleNotification {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  title: string;
  message: string;
  icon?: string;
  actions?: NotificationAction[];
  persistent?: boolean;
  timestamp: Date;
}

export interface NotificationAction {
  label: string;
  handler: () => void;
  primary?: boolean;
}

/**
 * Main module plugin interface
 */
export interface ModulePlugin {
  /**
   * Module metadata
   */
  metadata: ModuleMetadata;

  /**
   * Module lifecycle hooks
   */
  install(): Promise<void>;
  uninstall(): Promise<void>;
  activate(): Promise<void>;
  deactivate(): Promise<void>;

  /**
   * UI Components
   */
  getMainView(): Component;
  getSidebarWidget?(): Component;
  getQuickActions?(): QuickAction[];
  getTopBarActions?(): Component;

  /**
   * Routing
   */
  getRoutes(): RouteRecordRaw[];

  /**
   * State Management
   */
  getStore?(): Store;

  /**
   * Database
   */
  getSchema(): ModuleSchema;

  /**
   * API Integration
   */
  getApiEndpoints?(): ApiEndpoint[];

  /**
   * Settings
   */
  getSettingsView?(): Component;
  getDefaultSettings(): ModuleSettings;
  validateSettings?(settings: ModuleSettings): boolean;

  /**
   * Search Integration
   */
  search?(query: string): Promise<SearchResult[]>;

  /**
   * Notification Handling
   */
  handleNotification?(notification: ModuleNotification): void;

  /**
   * Export/Import
   */
  exportData?(): Promise<any>;
  importData?(data: any): Promise<void>;
}

/**
 * Quick action button
 */
export interface QuickAction {
  id: string;
  label: string;
  icon: string;
  handler: () => void;
  visible?: () => boolean;
  badge?: () => number | string;
}
```

---

## 📦 Module Registry

```typescript
// src/core/plugin-system/ModuleRegistry.ts

import { ModulePlugin, ModuleMetadata, ModuleState } from './ModuleInterface';
import { EventBus } from './EventBus';

/**
 * Module registration entry
 */
interface ModuleEntry {
  plugin: ModulePlugin;
  state: ModuleState;
  installedAt?: Date;
  activatedAt?: Date;
  error?: Error;
}

/**
 * Central registry for all modules
 */
export class ModuleRegistry {
  private static instance: ModuleRegistry;
  private modules = new Map<string, ModuleEntry>();
  private eventBus: EventBus;

  private constructor() {
    this.eventBus = EventBus.getInstance();
  }

  static getInstance(): ModuleRegistry {
    if (!ModuleRegistry.instance) {
      ModuleRegistry.instance = new ModuleRegistry();
    }
    return ModuleRegistry.instance;
  }

  /**
   * Register a module
   */
  register(plugin: ModulePlugin): void {
    const id = plugin.metadata.id;

    if (this.modules.has(id)) {
      throw new Error(`Module ${id} is already registered`);
    }

    // Validate dependencies
    this.validateDependencies(plugin);

    this.modules.set(id, {
      plugin,
      state: ModuleState.INSTALLED,
      installedAt: new Date()
    });

    this.eventBus.emit('module:registered', { id, metadata: plugin.metadata });
  }

  /**
   * Unregister a module
   */
  unregister(moduleId: string): void {
    if (!this.modules.has(moduleId)) {
      throw new Error(`Module ${moduleId} is not registered`);
    }

    // Check if other modules depend on this one
    this.checkDependents(moduleId);

    this.modules.delete(moduleId);
    this.eventBus.emit('module:unregistered', { id: moduleId });
  }

  /**
   * Get a registered module
   */
  get(moduleId: string): ModulePlugin | undefined {
    return this.modules.get(moduleId)?.plugin;
  }

  /**
   * Get all registered modules
   */
  getAll(): ModulePlugin[] {
    return Array.from(this.modules.values()).map(entry => entry.plugin);
  }

  /**
   * Get modules by category
   */
  getByCategory(category: string): ModulePlugin[] {
    return this.getAll().filter(plugin => plugin.metadata.category === category);
  }

  /**
   * Get module state
   */
  getState(moduleId: string): ModuleState {
    return this.modules.get(moduleId)?.state || ModuleState.UNINSTALLED;
  }

  /**
   * Set module state
   */
  setState(moduleId: string, state: ModuleState): void {
    const entry = this.modules.get(moduleId);
    if (entry) {
      entry.state = state;
      if (state === ModuleState.ACTIVE) {
        entry.activatedAt = new Date();
      }
      this.eventBus.emit('module:state-changed', { id: moduleId, state });
    }
  }

  /**
   * Check if module is installed
   */
  isInstalled(moduleId: string): boolean {
    const state = this.getState(moduleId);
    return state !== ModuleState.UNINSTALLED;
  }

  /**
   * Check if module is active
   */
  isActive(moduleId: string): boolean {
    return this.getState(moduleId) === ModuleState.ACTIVE;
  }

  /**
   * Validate module dependencies
   */
  private validateDependencies(plugin: ModulePlugin): void {
    const deps = plugin.metadata.dependencies || [];
    for (const depId of deps) {
      if (!this.isInstalled(depId)) {
        throw new Error(
          `Module ${plugin.metadata.id} depends on ${depId} which is not installed`
        );
      }
    }
  }

  /**
   * Check if any modules depend on this one
   */
  private checkDependents(moduleId: string): void {
    for (const plugin of this.getAll()) {
      const deps = plugin.metadata.dependencies || [];
      if (deps.includes(moduleId)) {
        throw new Error(
          `Cannot unregister ${moduleId}: ${plugin.metadata.id} depends on it`
        );
      }
    }
  }
}
```

---

## 🔄 Module Loader

```typescript
// src/core/plugin-system/ModuleLoader.ts

import { ModulePlugin, ModuleState } from './ModuleInterface';
import { ModuleRegistry } from './ModuleRegistry';
import { DatabaseManager } from '../database/DatabaseManager';
import { EventBus } from './EventBus';
import { invoke } from '@tauri-apps/api/tauri';

/**
 * Dynamic module loader
 */
export class ModuleLoader {
  private static instance: ModuleLoader;
  private registry: ModuleRegistry;
  private eventBus: EventBus;
  private database: DatabaseManager;

  private constructor() {
    this.registry = ModuleRegistry.getInstance();
    this.eventBus = EventBus.getInstance();
    this.database = DatabaseManager.getInstance();
  }

  static getInstance(): ModuleLoader {
    if (!ModuleLoader.instance) {
      ModuleLoader.instance = new ModuleLoader();
    }
    return ModuleLoader.instance;
  }

  /**
   * Install a module
   */
  async install(plugin: ModulePlugin): Promise<void> {
    const id = plugin.metadata.id;

    try {
      // Register module
      this.registry.register(plugin);

      // Install database schema
      await this.installSchema(plugin);

      // Run install hook
      await plugin.install();

      // Save to database
      await invoke('save_installed_module', {
        moduleId: id,
        metadata: plugin.metadata,
        settings: plugin.getDefaultSettings()
      });

      this.registry.setState(id, ModuleState.INSTALLED);
      this.eventBus.emit('module:installed', { id });

    } catch (error) {
      console.error(`Failed to install module ${id}:`, error);
      this.registry.setState(id, ModuleState.ERROR);
      throw error;
    }
  }

  /**
   * Uninstall a module
   */
  async uninstall(moduleId: string): Promise<void> {
    const plugin = this.registry.get(moduleId);
    if (!plugin) {
      throw new Error(`Module ${moduleId} not found`);
    }

    try {
      // Deactivate if active
      if (this.registry.isActive(moduleId)) {
        await this.deactivate(moduleId);
      }

      // Run uninstall hook
      await plugin.uninstall();

      // Remove database schema
      await this.uninstallSchema(plugin);

      // Remove from database
      await invoke('remove_installed_module', { moduleId });

      // Unregister
      this.registry.unregister(moduleId);

      this.eventBus.emit('module:uninstalled', { id: moduleId });

    } catch (error) {
      console.error(`Failed to uninstall module ${moduleId}:`, error);
      throw error;
    }
  }

  /**
   * Activate a module
   */
  async activate(moduleId: string): Promise<void> {
    const plugin = this.registry.get(moduleId);
    if (!plugin) {
      throw new Error(`Module ${moduleId} not found`);
    }

    try {
      // Run activate hook
      await plugin.activate();

      // Update state
      this.registry.setState(moduleId, ModuleState.ACTIVE);

      // Start sync for this module
      this.eventBus.emit('sync:start', { moduleId });

      this.eventBus.emit('module:activated', { id: moduleId });

    } catch (error) {
      console.error(`Failed to activate module ${moduleId}:`, error);
      this.registry.setState(moduleId, ModuleState.ERROR);
      throw error;
    }
  }

  /**
   * Deactivate a module
   */
  async deactivate(moduleId: string): Promise<void> {
    const plugin = this.registry.get(moduleId);
    if (!plugin) {
      throw new Error(`Module ${moduleId} not found`);
    }

    try {
      // Run deactivate hook
      await plugin.deactivate();

      // Update state
      this.registry.setState(moduleId, ModuleState.INACTIVE);

      // Stop sync for this module
      this.eventBus.emit('sync:stop', { moduleId });

      this.eventBus.emit('module:deactivated', { id: moduleId });

    } catch (error) {
      console.error(`Failed to deactivate module ${moduleId}:`, error);
      throw error;
    }
  }

  /**
   * Load all installed modules
   */
  async loadInstalledModules(): Promise<void> {
    try {
      const installed: any[] = await invoke('get_installed_modules');

      for (const record of installed) {
        // Dynamically import module
        const moduleImport = await this.importModule(record.module_id);
        const plugin = moduleImport.default as ModulePlugin;

        // Register
        this.registry.register(plugin);

        // Activate if was active
        if (record.is_active) {
          await this.activate(record.module_id);
        }
      }
    } catch (error) {
      console.error('Failed to load installed modules:', error);
      throw error;
    }
  }

  /**
   * Dynamically import a module
   */
  private async importModule(moduleId: string): Promise<any> {
    // Dynamic import based on module ID
    switch (moduleId) {
      case 'email':
        return import('../../modules/email');
      case 'crm':
        return import('../../modules/crm');
      case 'calendar':
        return import('../../modules/calendar');
      case 'helpdesk':
        return import('../../modules/helpdesk');
      default:
        throw new Error(`Unknown module: ${moduleId}`);
    }
  }

  /**
   * Install database schema for module
   */
  private async installSchema(plugin: ModulePlugin): Promise<void> {
    const schema = plugin.getSchema();
    await this.database.applySchema(plugin.metadata.id, schema);
  }

  /**
   * Uninstall database schema for module
   */
  private async uninstallSchema(plugin: ModulePlugin): Promise<void> {
    const schema = plugin.getSchema();
    await this.database.removeSchema(plugin.metadata.id, schema);
  }
}
```

---

## 🔔 Event Bus

```typescript
// src/core/plugin-system/EventBus.ts

type EventHandler = (...args: any[]) => void;

/**
 * Global event bus for inter-module communication
 */
export class EventBus {
  private static instance: EventBus;
  private handlers = new Map<string, Set<EventHandler>>();

  private constructor() {}

  static getInstance(): EventBus {
    if (!EventBus.instance) {
      EventBus.instance = new EventBus();
    }
    return EventBus.instance;
  }

  /**
   * Subscribe to an event
   */
  on(event: string, handler: EventHandler): () => void {
    if (!this.handlers.has(event)) {
      this.handlers.set(event, new Set());
    }

    this.handlers.get(event)!.add(handler);

    // Return unsubscribe function
    return () => this.off(event, handler);
  }

  /**
   * Subscribe to event (fires once)
   */
  once(event: string, handler: EventHandler): void {
    const wrappedHandler = (...args: any[]) => {
      handler(...args);
      this.off(event, wrappedHandler);
    };
    this.on(event, wrappedHandler);
  }

  /**
   * Unsubscribe from event
   */
  off(event: string, handler: EventHandler): void {
    const handlers = this.handlers.get(event);
    if (handlers) {
      handlers.delete(handler);
    }
  }

  /**
   * Emit an event
   */
  emit(event: string, ...args: any[]): void {
    const handlers = this.handlers.get(event);
    if (handlers) {
      handlers.forEach(handler => {
        try {
          handler(...args);
        } catch (error) {
          console.error(`Error in event handler for ${event}:`, error);
        }
      });
    }
  }

  /**
   * Clear all handlers for an event
   */
  clear(event?: string): void {
    if (event) {
      this.handlers.delete(event);
    } else {
      this.handlers.clear();
    }
  }
}

/**
 * Standard events
 */
export const EVENTS = {
  // Module lifecycle
  MODULE_REGISTERED: 'module:registered',
  MODULE_UNREGISTERED: 'module:unregistered',
  MODULE_INSTALLED: 'module:installed',
  MODULE_UNINSTALLED: 'module:uninstalled',
  MODULE_ACTIVATED: 'module:activated',
  MODULE_DEACTIVATED: 'module:deactivated',
  MODULE_STATE_CHANGED: 'module:state-changed',

  // Sync events
  SYNC_START: 'sync:start',
  SYNC_STOP: 'sync:stop',
  SYNC_PROGRESS: 'sync:progress',
  SYNC_COMPLETE: 'sync:complete',
  SYNC_ERROR: 'sync:error',

  // Data events
  CONTACT_CREATED: 'contact:created',
  CONTACT_UPDATED: 'contact:updated',
  CONTACT_DELETED: 'contact:deleted',
  EMAIL_RECEIVED: 'email:received',
  TICKET_CREATED: 'ticket:created',

  // UI events
  NAVIGATE: 'ui:navigate',
  NOTIFICATION: 'ui:notification',
  SEARCH: 'ui:search',
} as const;
```

---

## 🔧 Service Registry

```typescript
// src/core/plugin-system/ServiceRegistry.ts

/**
 * Shared service registry
 */
export class ServiceRegistry {
  private static instance: ServiceRegistry;
  private services = new Map<string, any>();

  private constructor() {}

  static getInstance(): ServiceRegistry {
    if (!ServiceRegistry.instance) {
      ServiceRegistry.instance = new ServiceRegistry();
    }
    return ServiceRegistry.instance;
  }

  /**
   * Register a service
   */
  register<T>(name: string, service: T): void {
    if (this.services.has(name)) {
      throw new Error(`Service ${name} is already registered`);
    }
    this.services.set(name, service);
  }

  /**
   * Get a service
   */
  get<T>(name: string): T {
    const service = this.services.get(name);
    if (!service) {
      throw new Error(`Service ${name} not found`);
    }
    return service as T;
  }

  /**
   * Check if service exists
   */
  has(name: string): boolean {
    return this.services.has(name);
  }

  /**
   * Unregister a service
   */
  unregister(name: string): void {
    this.services.delete(name);
  }
}

/**
 * Standard services
 */
export const SERVICES = {
  DATABASE: 'database',
  API_CLIENT: 'api-client',
  AUTH: 'auth',
  SYNC: 'sync',
  NOTIFICATIONS: 'notifications',
  SEARCH: 'search',
} as const;
```

---

## 🦀 Rust Backend (Module Management)

```rust
// src-tauri/src/commands/modules.rs

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::database::Connection;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstalledModule {
    pub id: i64,
    pub module_id: String,
    pub module_name: String,
    pub version: String,
    pub is_active: bool,
    pub settings: serde_json::Value,
    pub installed_at: String,
}

/// Save installed module to database
#[tauri::command]
pub async fn save_installed_module(
    module_id: String,
    metadata: serde_json::Value,
    settings: serde_json::Value,
    db: State<'_, Connection>,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO installed_modules (module_id, module_name, version, is_active, settings, installed_at)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))",
        rusqlite::params![
            module_id,
            metadata["name"].as_str().unwrap_or(""),
            metadata["version"].as_str().unwrap_or("1.0.0"),
            false,
            settings.to_string(),
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Get all installed modules
#[tauri::command]
pub async fn get_installed_modules(
    db: State<'_, Connection>,
) -> Result<Vec<InstalledModule>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, module_id, module_name, version, is_active, settings, installed_at FROM installed_modules")
        .map_err(|e| e.to_string())?;

    let modules = stmt
        .query_map([], |row| {
            Ok(InstalledModule {
                id: row.get(0)?,
                module_id: row.get(1)?,
                module_name: row.get(2)?,
                version: row.get(3)?,
                is_active: row.get(4)?,
                settings: serde_json::from_str(row.get::<_, String>(5)?.as_str())
                    .unwrap_or(serde_json::Value::Null),
                installed_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(modules)
}

/// Remove installed module
#[tauri::command]
pub async fn remove_installed_module(
    module_id: String,
    db: State<'_, Connection>,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM installed_modules WHERE module_id = ?1",
        [module_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Update module active status
#[tauri::command]
pub async fn set_module_active(
    module_id: String,
    is_active: bool,
    db: State<'_, Connection>,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE installed_modules SET is_active = ?1 WHERE module_id = ?2",
        rusqlite::params![is_active, module_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
```

---

## 📝 Example Module Implementation

```typescript
// src/modules/email/index.ts

import type { ModulePlugin } from '@/core/plugin-system/ModuleInterface';
import { EmailModule } from './module';

const emailPlugin: ModulePlugin = new EmailModule();

export default emailPlugin;
```

```typescript
// src/modules/email/module.ts

import {
  ModulePlugin,
  ModuleMetadata,
  ModuleCategory,
  ModuleSchema,
  ModuleSettings,
  QuickAction,
} from '@/core/plugin-system/ModuleInterface';
import { RouteRecordRaw } from 'vue-router';
import { defineStore } from 'pinia';

// Components (lazy loaded)
const EmailView = () => import('./components/EmailView.vue');
const EmailComposer = () => import('./components/EmailComposer.vue');
const EmailSettings = () => import('./components/EmailSettings.vue');

export class EmailModule implements ModulePlugin {
  metadata: ModuleMetadata = {
    id: 'email',
    name: 'Email Manager',
    version: '1.0.0',
    description: 'Full-featured email client with offline support',
    icon: 'envelope',
    category: ModuleCategory.CORE,
    minCoreVersion: '1.0.0',
    author: 'EIQ Desktop',
    license: 'Proprietary',
  };

  async install(): Promise<void> {
    console.log('Installing Email module...');
    // One-time setup tasks
  }

  async uninstall(): Promise<void> {
    console.log('Uninstalling Email module...');
    // Cleanup tasks
  }

  async activate(): Promise<void> {
    console.log('Activating Email module...');
    // Start background tasks, listeners, etc.
  }

  async deactivate(): Promise<void> {
    console.log('Deactivating Email module...');
    // Stop background tasks
  }

  getMainView() {
    return EmailView;
  }

  getQuickActions(): QuickAction[] {
    return [
      {
        id: 'compose',
        label: 'Compose Email',
        icon: 'pencil',
        handler: () => {
          // Open composer
        },
      },
    ];
  }

  getRoutes(): RouteRecordRaw[] {
    return [
      {
        path: '/email',
        name: 'email',
        component: EmailView,
        children: [
          {
            path: 'compose',
            name: 'email-compose',
            component: EmailComposer,
          },
        ],
      },
    ];
  }

  getSchema(): ModuleSchema {
    return {
      tables: [
        {
          name: 'email_accounts',
          columns: [
            { name: 'id', type: 'INTEGER', nullable: false },
            { name: 'account_id', type: 'INTEGER', nullable: false },
            { name: 'email_address', type: 'TEXT', nullable: false },
            { name: 'provider', type: 'TEXT', default: 'smartermail' },
            { name: 'is_active', type: 'BOOLEAN', default: true },
            { name: 'created_at', type: 'DATETIME', default: 'CURRENT_TIMESTAMP' },
          ],
          primaryKey: ['id'],
          foreignKeys: [
            {
              columns: ['account_id'],
              references: { table: 'accounts', columns: ['id'] },
              onDelete: 'CASCADE',
            },
          ],
        },
        {
          name: 'email_messages',
          columns: [
            { name: 'id', type: 'INTEGER', nullable: false },
            { name: 'folder_id', type: 'INTEGER', nullable: false },
            { name: 'subject', type: 'TEXT' },
            { name: 'body_html', type: 'TEXT' },
            { name: 'from_address', type: 'TEXT' },
            { name: 'received_date', type: 'DATETIME' },
            { name: 'is_read', type: 'BOOLEAN', default: false },
          ],
          primaryKey: ['id'],
        },
      ],
      indexes: [
        {
          name: 'idx_email_messages_folder',
          table: 'email_messages',
          columns: ['folder_id', 'received_date'],
        },
      ],
    };
  }

  getSettingsView() {
    return EmailSettings;
  }

  getDefaultSettings(): ModuleSettings {
    return {
      syncInterval: 5,
      downloadAttachments: true,
      showNotifications: true,
    };
  }

  async search(query: string) {
    // Search implementation
    return [];
  }
}
```

---

## 🚀 Bootstrap Process

```typescript
// src/main.ts

import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { createRouter, createWebHistory } from 'vue-router';
import App from './App.vue';

import { ModuleLoader } from './core/plugin-system/ModuleLoader';
import { ServiceRegistry } from './core/plugin-system/ServiceRegistry';
import { DatabaseManager } from './core/database/DatabaseManager';
import { ApiClient } from './core/api/ApiClient';
import { SyncEngine } from './core/sync/SyncEngine';

async function bootstrap() {
  const app = createApp(App);
  const pinia = createPinia();
  app.use(pinia);

  // Initialize core services
  const database = DatabaseManager.getInstance();
  const apiClient = new ApiClient();
  const syncEngine = SyncEngine.getInstance();

  // Register services
  const services = ServiceRegistry.getInstance();
  services.register('database', database);
  services.register('api-client', apiClient);
  services.register('sync', syncEngine);

  // Initialize database
  await database.initialize();

  // Load installed modules
  const loader = ModuleLoader.getInstance();
  await loader.loadInstalledModules();

  // Setup router (with dynamic routes from modules)
  const router = setupRouter();
  app.use(router);

  // Mount app
  app.mount('#app');
}

function setupRouter() {
  const registry = ModuleRegistry.getInstance();
  const routes = [];

  // Add routes from all active modules
  for (const plugin of registry.getAll()) {
    if (registry.isActive(plugin.metadata.id)) {
      routes.push(...plugin.getRoutes());
    }
  }

  return createRouter({
    history: createWebHistory(),
    routes,
  });
}

bootstrap();
```

---

## ✅ Module Development Checklist

When creating a new module:

- [ ] Create module directory under `src/modules/{module-name}/`
- [ ] Implement `ModulePlugin` interface
- [ ] Define database schema
- [ ] Create Vue components
- [ ] Define routes
- [ ] Implement API integration
- [ ] Add to dynamic import in `ModuleLoader`
- [ ] Write unit tests
- [ ] Document module API
- [ ] Create settings UI
- [ ] Implement search if applicable
- [ ] Add migration scripts

---

**Next**: See implementation guides for Email, CRM, and Calendar modules.
