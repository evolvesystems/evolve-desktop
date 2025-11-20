/**
 * Email Service for EvolveApp Desktop
 *
 * Provides methods to interact with the email API
 */

import axios from 'axios'

export interface EmailAddress {
  email: string
  name: string | null
}

export interface EmailMessage {
  id: number
  message_id: string
  subject: string
  from: EmailAddress
  to: EmailAddress[]
  cc?: EmailAddress[]
  bcc?: EmailAddress[]
  date: string
  is_read: boolean
  is_flagged: boolean
  is_draft: boolean
  has_attachments: boolean
  body_html?: string
  body_text?: string
  folder?: EmailFolder
  labels?: string[]
  attachments?: EmailAttachment[]
  size: number
  created_at: string
  updated_at: string

  // Computed/compatibility fields for UI
  fromAddress?: string
  fromName?: string
  toAddresses?: string[]
  receivedDate?: string
  isRead?: boolean
  isFlagged?: boolean
  hasAttachments?: boolean
  bodyHtml?: string
  body?: string
  snippet?: string
}

export interface EmailFolder {
  id: number
  name: string
  type: 'inbox' | 'sent' | 'drafts' | 'trash' | 'spam' | 'custom'
  unreadCount: number
  totalCount: number
  icon?: string
}

export interface EmailAttachment {
  id: number
  filename: string
  mimeType: string
  size: number
  downloadUrl: string
}

export interface EmailListParams {
  page?: number
  limit?: number
  folderId?: number
  isRead?: boolean
  isFlagged?: boolean
  hasAttachments?: boolean
  search?: string
  from?: string
  to?: string
  subject?: string
  dateFrom?: string
  dateTo?: string
}

export interface EmailListResponse {
  emails: EmailMessage[]
  pagination: {
    page: number
    limit: number
    total: number
    has_more: boolean
  }
}

// For backwards compatibility
export interface EmailListResponseCompat {
  data: EmailMessage[]
  total: number
  page: number
  limit: number
  pages: number
}

class EmailService {
  private baseUrl = '/api/v1/emails'

  /**
   * Get list of emails
   */
  async getEmails(params: EmailListParams = {}): Promise<EmailListResponseCompat> {
    try {
      const response = await axios.get(this.baseUrl, { params })
      const apiResponse: EmailListResponse = response.data

      // Transform emails to add compatibility fields for UI
      const emails = apiResponse.emails.map(email => this.transformEmailForUI(email))

      // Return in compatibility format
      return {
        data: emails,
        total: apiResponse.pagination.total,
        page: apiResponse.pagination.page,
        limit: apiResponse.pagination.limit,
        pages: Math.ceil(apiResponse.pagination.total / apiResponse.pagination.limit)
      }
    } catch (error) {
      console.error('[EmailService] Failed to fetch emails:', error)
      throw error
    }
  }

  /**
   * Transform API email format to UI-friendly format
   */
  private transformEmailForUI(email: EmailMessage): EmailMessage {
    return {
      ...email,
      // Compatibility fields
      fromAddress: email.from.email,
      fromName: email.from.name || email.from.email,
      toAddresses: email.to.map(addr => addr.email),
      receivedDate: email.date,
      isRead: email.is_read,
      isFlagged: email.is_flagged,
      hasAttachments: email.has_attachments,
      bodyHtml: email.body_html,
      body: email.body_text,
      snippet: email.body_text ? email.body_text.substring(0, 150) : ''
    }
  }

  /**
   * Get single email by ID
   */
  async getEmail(id: number): Promise<EmailMessage> {
    try {
      const response = await axios.get(`${this.baseUrl}/${id}`)
      return response.data
    } catch (error) {
      console.error(`[EmailService] Failed to fetch email ${id}:`, error)
      throw error
    }
  }

  /**
   * Mark email as read/unread
   */
  async markAsRead(id: number, isRead: boolean = true): Promise<boolean> {
    try {
      await axios.patch(`${this.baseUrl}/${id}/read`, { isRead })
      return true
    } catch (error) {
      console.error(`[EmailService] Failed to mark email ${id} as read:`, error)
      return false
    }
  }

  /**
   * Flag/unflag email
   */
  async setFlag(id: number, isFlagged: boolean): Promise<boolean> {
    try {
      await axios.patch(`${this.baseUrl}/${id}/flag`, { isFlagged })
      return true
    } catch (error) {
      console.error(`[EmailService] Failed to flag email ${id}:`, error)
      return false
    }
  }

  /**
   * Move email to folder
   */
  async moveToFolder(id: number, folderId: number): Promise<boolean> {
    try {
      await axios.patch(`${this.baseUrl}/${id}/move`, { folderId })
      return true
    } catch (error) {
      console.error(`[EmailService] Failed to move email ${id}:`, error)
      return false
    }
  }

  /**
   * Delete email
   */
  async deleteEmail(id: number): Promise<boolean> {
    try {
      await axios.delete(`${this.baseUrl}/${id}`)
      return true
    } catch (error) {
      console.error(`[EmailService] Failed to delete email ${id}:`, error)
      return false
    }
  }

  /**
   * Send email
   */
  async sendEmail(data: {
    to: string[]
    cc?: string[]
    bcc?: string[]
    subject: string
    body: string
    bodyHtml?: string
    attachments?: File[]
  }): Promise<boolean> {
    try {
      const formData = new FormData()
      formData.append('to', JSON.stringify(data.to))
      if (data.cc) formData.append('cc', JSON.stringify(data.cc))
      if (data.bcc) formData.append('bcc', JSON.stringify(data.bcc))
      formData.append('subject', data.subject)
      formData.append('body', data.body)
      if (data.bodyHtml) formData.append('bodyHtml', data.bodyHtml)

      if (data.attachments) {
        data.attachments.forEach((file, index) => {
          formData.append(`attachments[${index}]`, file)
        })
      }

      await axios.post(`${this.baseUrl}/send`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data'
        }
      })
      return true
    } catch (error) {
      console.error('[EmailService] Failed to send email:', error)
      return false
    }
  }

  /**
   * Get folders
   */
  async getFolders(): Promise<EmailFolder[]> {
    try {
      const response = await axios.get(`${this.baseUrl}/folders`)
      return response.data
    } catch (error) {
      console.error('[EmailService] Failed to fetch folders:', error)
      return []
    }
  }

  /**
   * Get email accounts
   */
  async getAccounts(): Promise<any[]> {
    try {
      const response = await axios.get(`${this.baseUrl}/accounts`)
      return response.data
    } catch (error) {
      console.error('[EmailService] Failed to fetch accounts:', error)
      return []
    }
  }

  /**
   * Search emails
   */
  async searchEmails(query: string, params: EmailListParams = {}): Promise<EmailListResponse> {
    try {
      const response = await axios.get(this.baseUrl, {
        params: { ...params, search: query }
      })
      return response.data
    } catch (error) {
      console.error('[EmailService] Failed to search emails:', error)
      throw error
    }
  }

  /**
   * Bulk mark as read
   */
  async bulkMarkAsRead(ids: number[], isRead: boolean = true): Promise<boolean> {
    try {
      await axios.post(`${this.baseUrl}/bulk/read`, { ids, isRead })
      return true
    } catch (error) {
      console.error('[EmailService] Failed to bulk mark as read:', error)
      return false
    }
  }

  /**
   * Bulk delete
   */
  async bulkDelete(ids: number[]): Promise<boolean> {
    try {
      await axios.post(`${this.baseUrl}/bulk/delete`, { ids })
      return true
    } catch (error) {
      console.error('[EmailService] Failed to bulk delete:', error)
      return false
    }
  }
}

// Export singleton instance
export const emailService = new EmailService()
export default emailService
