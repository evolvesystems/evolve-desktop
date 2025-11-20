/**
 * Email Store - Manages email state and operations
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import emailService, { type EmailMessage, type EmailFolder, type EmailListParams } from '@/services/emailService'

export const useEmailStore = defineStore('email', () => {
  // State
  const emails = ref<EmailMessage[]>([])
  const selectedEmail = ref<EmailMessage | null>(null)
  const folders = ref<EmailFolder[]>([])
  const currentFolder = ref<EmailFolder | null>(null)
  const isLoading = ref(false)
  const isLoadingMessage = ref(false)
  const isSending = ref(false)

  // Pagination
  const currentPage = ref(1)
  const totalPages = ref(1)
  const totalEmails = ref(0)
  const pageSize = ref(50)

  // Filters
  const searchQuery = ref('')
  const showUnreadOnly = ref(false)
  const showFlaggedOnly = ref(false)

  // Computed
  const unreadCount = computed(() => {
    return emails.value.filter(e => !e.isRead).length
  })

  const flaggedCount = computed(() => {
    return emails.value.filter(e => e.isFlagged).length
  })

  const inboxFolder = computed(() => {
    return folders.value.find(f => f.type === 'inbox')
  })

  const sentFolder = computed(() => {
    return folders.value.find(f => f.type === 'sent')
  })

  const draftsFolder = computed(() => {
    return folders.value.find(f => f.type === 'drafts')
  })

  const trashFolder = computed(() => {
    return folders.value.find(f => f.type === 'trash')
  })

  // Actions

  /**
   * Initialize email module - load folders and inbox
   */
  async function initialize(): Promise<void> {
    console.log('[EmailStore] Initializing...')
    await loadFolders()

    if (inboxFolder.value) {
      await selectFolder(inboxFolder.value)
    }
  }

  /**
   * Load folders
   */
  async function loadFolders(): Promise<void> {
    try {
      folders.value = await emailService.getFolders()
      console.log('[EmailStore] Loaded', folders.value.length, 'folders')
    } catch (error) {
      console.error('[EmailStore] Failed to load folders:', error)
    }
  }

  /**
   * Load emails for current folder
   */
  async function loadEmails(params: EmailListParams = {}): Promise<void> {
    isLoading.value = true

    try {
      const queryParams: EmailListParams = {
        page: currentPage.value,
        limit: pageSize.value,
        ...params
      }

      if (currentFolder.value) {
        queryParams.folderId = currentFolder.value.id
      }

      if (searchQuery.value) {
        queryParams.search = searchQuery.value
      }

      if (showUnreadOnly.value) {
        queryParams.isRead = false
      }

      if (showFlaggedOnly.value) {
        queryParams.isFlagged = true
      }

      const response = await emailService.getEmails(queryParams)

      emails.value = response.data
      totalEmails.value = response.total
      totalPages.value = response.pages
      currentPage.value = response.page

      console.log('[EmailStore] Loaded', emails.value.length, 'emails')
    } catch (error) {
      console.error('[EmailStore] Failed to load emails:', error)
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Select a folder
   */
  async function selectFolder(folder: EmailFolder): Promise<void> {
    currentFolder.value = folder
    currentPage.value = 1
    searchQuery.value = ''
    showUnreadOnly.value = false
    showFlaggedOnly.value = false
    await loadEmails()
  }

  /**
   * Select an email to view
   */
  async function selectEmail(email: EmailMessage): Promise<void> {
    isLoadingMessage.value = true
    selectedEmail.value = email

    try {
      // Fetch full email details
      const fullEmail = await emailService.getEmail(email.id)
      selectedEmail.value = fullEmail

      // Mark as read if not already
      if (!fullEmail.isRead) {
        await markAsRead(fullEmail.id, true)
      }
    } catch (error) {
      console.error('[EmailStore] Failed to load email:', error)
    } finally {
      isLoadingMessage.value = false
    }
  }

  /**
   * Mark email as read/unread
   */
  async function markAsRead(id: number, isRead: boolean): Promise<void> {
    const success = await emailService.markAsRead(id, isRead)
    if (success) {
      // Update in list
      const email = emails.value.find(e => e.id === id)
      if (email) {
        email.isRead = isRead
      }
      // Update selected
      if (selectedEmail.value?.id === id) {
        selectedEmail.value.isRead = isRead
      }
    }
  }

  /**
   * Toggle flag on email
   */
  async function toggleFlag(id: number): Promise<void> {
    const email = emails.value.find(e => e.id === id)
    if (!email) return

    const newFlagState = !email.isFlagged
    const success = await emailService.setFlag(id, newFlagState)

    if (success) {
      email.isFlagged = newFlagState
      if (selectedEmail.value?.id === id) {
        selectedEmail.value.isFlagged = newFlagState
      }
    }
  }

  /**
   * Delete email
   */
  async function deleteEmail(id: number): Promise<void> {
    const success = await emailService.deleteEmail(id)
    if (success) {
      emails.value = emails.value.filter(e => e.id !== id)
      if (selectedEmail.value?.id === id) {
        selectedEmail.value = null
      }
      totalEmails.value--
    }
  }

  /**
   * Move email to folder
   */
  async function moveToFolder(id: number, folderId: number): Promise<void> {
    const success = await emailService.moveToFolder(id, folderId)
    if (success) {
      await loadEmails()
    }
  }

  /**
   * Send email
   */
  async function sendEmail(data: {
    to: string[]
    cc?: string[]
    bcc?: string[]
    subject: string
    body: string
    bodyHtml?: string
    attachments?: File[]
  }): Promise<boolean> {
    isSending.value = true
    try {
      const success = await emailService.sendEmail(data)
      if (success) {
        console.log('[EmailStore] Email sent successfully')
      }
      return success
    } catch (error) {
      console.error('[EmailStore] Failed to send email:', error)
      return false
    } finally {
      isSending.value = false
    }
  }

  /**
   * Search emails
   */
  async function search(query: string): Promise<void> {
    searchQuery.value = query
    currentPage.value = 1
    await loadEmails()
  }

  /**
   * Clear search
   */
  async function clearSearch(): Promise<void> {
    searchQuery.value = ''
    await loadEmails()
  }

  /**
   * Go to next page
   */
  async function nextPage(): Promise<void> {
    if (currentPage.value < totalPages.value) {
      currentPage.value++
      await loadEmails()
    }
  }

  /**
   * Go to previous page
   */
  async function previousPage(): Promise<void> {
    if (currentPage.value > 1) {
      currentPage.value--
      await loadEmails()
    }
  }

  /**
   * Refresh current view
   */
  async function refresh(): Promise<void> {
    await loadEmails()
  }

  return {
    // State
    emails,
    selectedEmail,
    folders,
    currentFolder,
    isLoading,
    isLoadingMessage,
    isSending,
    currentPage,
    totalPages,
    totalEmails,
    pageSize,
    searchQuery,
    showUnreadOnly,
    showFlaggedOnly,

    // Computed
    unreadCount,
    flaggedCount,
    inboxFolder,
    sentFolder,
    draftsFolder,
    trashFolder,

    // Actions
    initialize,
    loadFolders,
    loadEmails,
    selectFolder,
    selectEmail,
    markAsRead,
    toggleFlag,
    deleteEmail,
    moveToFolder,
    sendEmail,
    search,
    clearSearch,
    nextPage,
    previousPage,
    refresh
  }
})
