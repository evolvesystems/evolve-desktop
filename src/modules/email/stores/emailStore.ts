import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import axios from 'axios'
import type {
  Email,
  EmailFolder,
  EmailFilter,
  EmailListResponse,
  SendEmailRequest,
  EmailAccount,
  EmailLabel,
  EmailQuota
} from '../types/email'

export const useEmailStore = defineStore('email', () => {
  // State
  const emails = ref<Email[]>([])
  const selectedEmail = ref<Email | null>(null)
  const folders = ref<EmailFolder[]>([])
  const selectedFolder = ref<EmailFolder | null>(null)
  const accounts = ref<EmailAccount[]>([])
  const labels = ref<EmailLabel[]>([])
  const quota = ref<EmailQuota | null>(null)
  const loading = ref(false)
  const composerOpen = ref(false)
  const filter = ref<EmailFilter>({})
  const pagination = ref({
    page: 1,
    limit: 50,
    total: 0,
    has_more: false
  })

  // Computed
  const unreadCount = computed(() =>
    emails.value.filter(e => !e.is_read).length
  )

  const flaggedEmails = computed(() =>
    emails.value.filter(e => e.is_flagged)
  )

  const draftEmails = computed(() =>
    emails.value.filter(e => e.is_draft)
  )

  // Actions
  async function fetchFolders() {
    try {
      const response = await axios.get('/api/v1/emails/folders')
      folders.value = response.data

      // Set inbox as default if no folder selected
      if (!selectedFolder.value && folders.value.length > 0) {
        selectedFolder.value = folders.value.find(f => f.type === 'inbox') || folders.value[0]
      }
    } catch (error) {
      console.error('Failed to fetch folders:', error)
      throw error
    }
  }

  async function fetchEmails(folderId?: number) {
    loading.value = true

    try {
      const params: any = {
        page: pagination.value.page,
        limit: pagination.value.limit,
        folder_id: folderId || selectedFolder.value?.id,
        ...filter.value
      }

      const response = await axios.get<EmailListResponse>('/api/v1/emails', { params })

      if (pagination.value.page === 1) {
        emails.value = response.data.emails
      } else {
        emails.value.push(...response.data.emails)
      }

      pagination.value = response.data.pagination
    } catch (error) {
      console.error('Failed to fetch emails:', error)
      throw error
    } finally {
      loading.value = false
    }
  }

  async function fetchEmail(id: number) {
    loading.value = true

    try {
      const response = await axios.get<Email>(`/api/v1/emails/${id}`)
      selectedEmail.value = response.data

      // Mark as read
      if (!response.data.is_read) {
        await markAsRead(id)
      }

      return response.data
    } catch (error) {
      console.error('Failed to fetch email:', error)
      throw error
    } finally {
      loading.value = false
    }
  }

  async function sendEmail(data: SendEmailRequest) {
    loading.value = true

    try {
      const response = await axios.post<Email>('/api/v1/emails', data)

      // Add to sent folder
      const sentFolder = folders.value.find(f => f.type === 'sent')
      if (sentFolder && selectedFolder.value?.id === sentFolder.id) {
        emails.value.unshift(response.data)
      }

      return response.data
    } catch (error) {
      console.error('Failed to send email:', error)
      throw error
    } finally {
      loading.value = false
    }
  }

  async function markAsRead(id: number, read: boolean = true) {
    try {
      await axios.patch(`/api/v1/emails/${id}`, { is_read: read })

      // Update local state
      const email = emails.value.find(e => e.id === id)
      if (email) {
        email.is_read = read
      }

      if (selectedEmail.value?.id === id) {
        selectedEmail.value.is_read = read
      }

      // Update folder unread count
      if (selectedFolder.value) {
        selectedFolder.value.unread_count += read ? -1 : 1
      }
    } catch (error) {
      console.error('Failed to mark email:', error)
      throw error
    }
  }

  async function markAsFlagged(id: number, flagged: boolean = true) {
    try {
      await axios.patch(`/api/v1/emails/${id}`, { is_flagged: flagged })

      // Update local state
      const email = emails.value.find(e => e.id === id)
      if (email) {
        email.is_flagged = flagged
      }

      if (selectedEmail.value?.id === id) {
        selectedEmail.value.is_flagged = flagged
      }
    } catch (error) {
      console.error('Failed to flag email:', error)
      throw error
    }
  }

  async function deleteEmail(id: number) {
    try {
      await axios.delete(`/api/v1/emails/${id}`)

      // Remove from local state
      emails.value = emails.value.filter(e => e.id !== id)

      if (selectedEmail.value?.id === id) {
        selectedEmail.value = null
      }

      // Update folder count
      if (selectedFolder.value) {
        selectedFolder.value.total_count -= 1
      }
    } catch (error) {
      console.error('Failed to delete email:', error)
      throw error
    }
  }

  async function moveToFolder(emailId: number, folderId: number) {
    try {
      await axios.patch(`/api/v1/emails/${emailId}/move`, { folder_id: folderId })

      // Remove from current view if different folder
      if (selectedFolder.value?.id !== folderId) {
        emails.value = emails.value.filter(e => e.id !== emailId)
      }

      // Update counts
      if (selectedFolder.value) {
        selectedFolder.value.total_count -= 1
      }

      const targetFolder = folders.value.find(f => f.id === folderId)
      if (targetFolder) {
        targetFolder.total_count += 1
      }
    } catch (error) {
      console.error('Failed to move email:', error)
      throw error
    }
  }

  async function searchEmails(query: string) {
    filter.value = { ...filter.value, search: query }
    pagination.value.page = 1
    await fetchEmails()
  }

  async function applyFilter(newFilter: EmailFilter) {
    filter.value = newFilter
    pagination.value.page = 1
    await fetchEmails()
  }

  function clearFilter() {
    filter.value = {}
    pagination.value.page = 1
  }

  function selectEmail(email: Email | null) {
    selectedEmail.value = email
  }

  function selectFolder(folder: EmailFolder) {
    selectedFolder.value = folder
    pagination.value.page = 1
    emails.value = []
    fetchEmails(folder.id)
  }

  function openComposer() {
    composerOpen.value = true
  }

  function closeComposer() {
    composerOpen.value = false
  }

  async function loadMore() {
    if (pagination.value.has_more && !loading.value) {
      pagination.value.page += 1
      await fetchEmails()
    }
  }

  async function refresh() {
    pagination.value.page = 1
    emails.value = []
    await Promise.all([
      fetchFolders(),
      fetchEmails()
    ])
  }

  async function fetchAccounts() {
    try {
      const response = await axios.get('/api/v1/emails/accounts')
      accounts.value = response.data
    } catch (error) {
      console.error('Failed to fetch accounts:', error)
    }
  }

  async function fetchLabels() {
    try {
      const response = await axios.get('/api/v1/emails/labels')
      labels.value = response.data
    } catch (error) {
      console.error('Failed to fetch labels:', error)
    }
  }

  async function fetchQuota() {
    try {
      const response = await axios.get('/api/v1/emails/quota')
      quota.value = response.data
    } catch (error) {
      console.error('Failed to fetch quota:', error)
    }
  }

  return {
    // State
    emails,
    selectedEmail,
    folders,
    selectedFolder,
    accounts,
    labels,
    quota,
    loading,
    composerOpen,
    filter,
    pagination,
    // Computed
    unreadCount,
    flaggedEmails,
    draftEmails,
    // Actions
    fetchFolders,
    fetchEmails,
    fetchEmail,
    sendEmail,
    markAsRead,
    markAsFlagged,
    deleteEmail,
    moveToFolder,
    searchEmails,
    applyFilter,
    clearFilter,
    selectEmail,
    selectFolder,
    openComposer,
    closeComposer,
    loadMore,
    refresh,
    fetchAccounts,
    fetchLabels,
    fetchQuota
  }
})
