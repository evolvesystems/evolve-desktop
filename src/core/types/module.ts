import type { Component } from 'vue'
import type { RouteRecordRaw } from 'vue-router'

/**
 * Module metadata defining basic information about a plugin
 */
export interface ModuleMetadata {
  id: string
  name: string
  version: string
  description: string
  icon: string
  category: 'communication' | 'productivity' | 'business' | 'tools' | 'other'
  author?: string
  requiredModules?: string[]
}

/**
 * Database schema definition for a module
 */
export interface ModuleSchema {
  version: string
  tables: TableDefinition[]
}

export interface TableDefinition {
  name: string
  columns: ColumnDefinition[]
  indexes?: IndexDefinition[]
}

export interface ColumnDefinition {
  name: string
  type: 'INTEGER' | 'TEXT' | 'REAL' | 'BLOB' | 'BOOLEAN' | 'DATETIME'
  primaryKey?: boolean
  autoIncrement?: boolean
  notNull?: boolean
  unique?: boolean
  defaultValue?: any
  references?: {
    table: string
    column: string
    onDelete?: 'CASCADE' | 'SET NULL' | 'RESTRICT'
  }
}

export interface IndexDefinition {
  name: string
  columns: string[]
  unique?: boolean
}

/**
 * Search result from module search functionality
 */
export interface SearchResult {
  moduleId: string
  type: string
  title: string
  subtitle?: string
  data: any
  score?: number
}

/**
 * Module settings configuration
 */
export interface ModuleSettings {
  enabled: boolean
  config: Record<string, any>
}

/**
 * Main plugin interface that all modules must implement
 */
export interface ModulePlugin {
  /** Module metadata */
  metadata: ModuleMetadata

  /** Initialize the module */
  install(): Promise<void>

  /** Uninstall/cleanup the module */
  uninstall?(): Promise<void>

  /** Get the main view component */
  getMainView(): Component

  /** Get module routes */
  getRoutes(): RouteRecordRaw[]

  /** Get database schema */
  getSchema(): ModuleSchema

  /** Search within module data */
  search?(query: string): Promise<SearchResult[]>

  /** Get settings component */
  getSettingsView?(): Component

  /** Handle module settings update */
  onSettingsChanged?(settings: ModuleSettings): Promise<void>
}

/**
 * Module installation status
 */
export interface ModuleInstallation {
  moduleId: string
  version: string
  installedAt: Date
  enabled: boolean
  settings: ModuleSettings
}

/**
 * Event emitted by modules
 */
export interface ModuleEvent<T = any> {
  moduleId: string
  eventType: string
  timestamp: Date
  data: T
}
