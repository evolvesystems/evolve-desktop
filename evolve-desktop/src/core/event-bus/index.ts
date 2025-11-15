/**
 * Global Event Bus for cross-module communication
 * Allows modules to communicate without direct dependencies
 */

type EventHandler = (...args: any[]) => void

class EventBus {
  private events: Map<string, Set<EventHandler>> = new Map()
  private onceEvents: Map<string, Set<EventHandler>> = new Map()

  /**
   * Subscribe to an event
   * @param event Event name
   * @param handler Event handler function
   * @returns Unsubscribe function
   */
  on(event: string, handler: EventHandler): () => void {
    if (!this.events.has(event)) {
      this.events.set(event, new Set())
    }
    this.events.get(event)!.add(handler)

    // Return unsubscribe function
    return () => this.off(event, handler)
  }

  /**
   * Subscribe to an event only once
   * @param event Event name
   * @param handler Event handler function
   */
  once(event: string, handler: EventHandler): void {
    if (!this.onceEvents.has(event)) {
      this.onceEvents.set(event, new Set())
    }
    this.onceEvents.get(event)!.add(handler)
  }

  /**
   * Unsubscribe from an event
   * @param event Event name
   * @param handler Event handler function
   */
  off(event: string, handler: EventHandler): void {
    this.events.get(event)?.delete(handler)
    this.onceEvents.get(event)?.delete(handler)
  }

  /**
   * Emit an event
   * @param event Event name
   * @param args Event arguments
   */
  emit(event: string, ...args: any[]): void {
    // Call regular handlers
    const handlers = this.events.get(event)
    if (handlers) {
      handlers.forEach(handler => {
        try {
          handler(...args)
        } catch (error) {
          console.error(`Error in event handler for '${event}':`, error)
        }
      })
    }

    // Call once handlers
    const onceHandlers = this.onceEvents.get(event)
    if (onceHandlers) {
      onceHandlers.forEach(handler => {
        try {
          handler(...args)
        } catch (error) {
          console.error(`Error in once event handler for '${event}':`, error)
        }
      })
      this.onceEvents.delete(event)
    }
  }

  /**
   * Clear all event listeners
   */
  clear(): void {
    this.events.clear()
    this.onceEvents.clear()
  }

  /**
   * Get all registered event names
   */
  getEventNames(): string[] {
    return [
      ...Array.from(this.events.keys()),
      ...Array.from(this.onceEvents.keys()),
    ]
  }

  /**
   * Get handler count for an event
   */
  listenerCount(event: string): number {
    const regularCount = this.events.get(event)?.size || 0
    const onceCount = this.onceEvents.get(event)?.size || 0
    return regularCount + onceCount
  }
}

// Export singleton instance
export const eventBus = new EventBus()

// Export class for testing
export { EventBus }

// Common event names
export const EVENTS = {
  // Module lifecycle
  MODULE_INSTALLED: 'module:installed',
  MODULE_UNINSTALLED: 'module:uninstalled',
  MODULE_ENABLED: 'module:enabled',
  MODULE_DISABLED: 'module:disabled',

  // Authentication
  USER_LOGGED_IN: 'user:logged-in',
  USER_LOGGED_OUT: 'user:logged-out',
  TOKEN_REFRESHED: 'user:token-refreshed',

  // Email events
  EMAIL_RECEIVED: 'email:received',
  EMAIL_SENT: 'email:sent',
  EMAIL_DELETED: 'email:deleted',
  EMAIL_FLAGGED: 'email:flagged',
  EMAIL_MOVED: 'email:moved',

  // Contact events
  CONTACT_CREATED: 'contact:created',
  CONTACT_UPDATED: 'contact:updated',
  CONTACT_DELETED: 'contact:deleted',
  CONTACT_MERGED: 'contact:merged',

  // Calendar events
  EVENT_CREATED: 'calendar:event-created',
  EVENT_UPDATED: 'calendar:event-updated',
  EVENT_DELETED: 'calendar:event-deleted',
  EVENT_REMINDER: 'calendar:reminder',

  // CRM events
  DEAL_CREATED: 'crm:deal-created',
  DEAL_UPDATED: 'crm:deal-updated',
  DEAL_STAGE_CHANGED: 'crm:deal-stage-changed',
  DEAL_WON: 'crm:deal-won',
  DEAL_LOST: 'crm:deal-lost',

  // Sync events
  SYNC_STARTED: 'sync:started',
  SYNC_COMPLETED: 'sync:completed',
  SYNC_FAILED: 'sync:failed',
  SYNC_CONFLICT: 'sync:conflict',

  // Notification events
  NOTIFICATION_CREATED: 'notification:created',
  NOTIFICATION_READ: 'notification:read',
  NOTIFICATION_DISMISSED: 'notification:dismissed',

  // Search events
  SEARCH_QUERY: 'search:query',
  SEARCH_RESULTS: 'search:results',

  // UI events
  THEME_CHANGED: 'ui:theme-changed',
  SIDEBAR_TOGGLED: 'ui:sidebar-toggled',
  MODAL_OPENED: 'ui:modal-opened',
  MODAL_CLOSED: 'ui:modal-closed',
} as const

export type EventName = typeof EVENTS[keyof typeof EVENTS]
