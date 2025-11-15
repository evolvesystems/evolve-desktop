/**
 * Tauri API Service
 * Provides native desktop features (notifications, tray, dialogs)
 * Data operations go through axios to eiq-manager API
 */

import { invoke } from '@tauri-apps/api/core'
import { save, open } from '@tauri-apps/plugin-dialog'
import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification'

// Check if running in Tauri context
export const isTauri = () => {
  return '__TAURI__' in window
}

/**
 * Greet command (example)
 */
export async function greet(name: string): Promise<string> {
  if (!isTauri()) return `Hello, ${name}! (Browser mode)`

  try {
    return await invoke<string>('greet', { name })
  } catch (error) {
    console.error('Greet error:', error)
    throw error
  }
}

/**
 * Get application version
 */
export async function getAppVersion(): Promise<string> {
  if (!isTauri()) return '0.0.0-browser'

  try {
    return await invoke<string>('get_app_version')
  } catch (error) {
    console.error('Get version error:', error)
    return 'unknown'
  }
}

/**
 * Check API connection status
 */
export async function checkApiConnection(): Promise<boolean> {
  if (!isTauri()) return true // In browser, assume connected

  try {
    const result = await invoke<string>('sync_data', { moduleId: 'health-check' })
    const status = JSON.parse(result)
    return status.connected || false
  } catch (error) {
    console.error('API connection check failed:', error)
    return false
  }
}

/**
 * Show native notification
 */
export async function showNotification(title: string, body: string) {
  if (!isTauri()) {
    // Fallback to browser notification API
    if ('Notification' in window && Notification.permission === 'granted') {
      new Notification(title, { body })
    }
    return
  }

  try {
    let permissionGranted = await isPermissionGranted()

    if (!permissionGranted) {
      const permission = await requestPermission()
      permissionGranted = permission === 'granted'
    }

    if (permissionGranted) {
      sendNotification({ title, body })
    }
  } catch (error) {
    console.error('Notification error:', error)
  }
}

/**
 * Open file dialog
 */
export async function openFileDialog(options?: {
  multiple?: boolean
  directory?: boolean
  filters?: Array<{ name: string; extensions: string[] }>
}): Promise<string | string[] | null> {
  if (!isTauri()) {
    console.warn('File dialog not available in browser mode')
    return null
  }

  try {
    const result = await open({
      multiple: options?.multiple || false,
      directory: options?.directory || false,
      filters: options?.filters
    })

    return result
  } catch (error) {
    console.error('Open dialog error:', error)
    return null
  }
}

/**
 * Save file dialog
 */
export async function saveFileDialog(options?: {
  defaultPath?: string
  filters?: Array<{ name: string; extensions: string[] }>
}): Promise<string | null> {
  if (!isTauri()) {
    console.warn('Save dialog not available in browser mode')
    return null
  }

  try {
    const result = await save({
      defaultPath: options?.defaultPath,
      filters: options?.filters
    })

    return result
  } catch (error) {
    console.error('Save dialog error:', error)
    return null
  }
}

/**
 * Export attachment/file
 */
export async function exportFile(
  content: Blob | string,
  defaultFileName: string
): Promise<boolean> {
  if (!isTauri()) {
    // Browser fallback - trigger download
    const url = typeof content === 'string'
      ? `data:text/plain;charset=utf-8,${encodeURIComponent(content)}`
      : URL.createObjectURL(content)

    const a = document.createElement('a')
    a.href = url
    a.download = defaultFileName
    a.click()

    if (typeof content !== 'string') {
      URL.revokeObjectURL(url)
    }

    return true
  }

  try {
    const filePath = await saveFileDialog({
      defaultPath: defaultFileName
    })

    if (!filePath) return false

    // TODO: Write file using Tauri FS plugin
    console.log('Would save file to:', filePath)
    return true
  } catch (error) {
    console.error('Export file error:', error)
    return false
  }
}

export default {
  isTauri,
  greet,
  getAppVersion,
  checkApiConnection,
  showNotification,
  openFileDialog,
  saveFileDialog,
  exportFile
}
