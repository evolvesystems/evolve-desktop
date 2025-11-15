/**
 * Email Module Types
 */

export interface Email {
  id: number
  message_id: string
  subject: string
  from: EmailAddress
  to: EmailAddress[]
  cc?: EmailAddress[]
  bcc?: EmailAddress[]
  reply_to?: EmailAddress[]
  body_text?: string
  body_html?: string
  date: string
  is_read: boolean
  is_flagged: boolean
  is_draft: boolean
  has_attachments: boolean
  folder: EmailFolder
  labels: string[]
  attachments?: EmailAttachment[]
  in_reply_to?: string
  references?: string[]
  size: number
  created_at: string
  updated_at: string
}

export interface EmailAddress {
  email: string
  name?: string
}

export interface EmailFolder {
  id: number
  name: string
  type: FolderType
  parent_id?: number
  unread_count: number
  total_count: number
  icon?: string
}

export type FolderType =
  | 'inbox'
  | 'sent'
  | 'drafts'
  | 'trash'
  | 'spam'
  | 'archive'
  | 'custom'

export interface EmailAttachment {
  id: number
  filename: string
  content_type: string
  size: number
  url: string
  cid?: string // Content-ID for inline images
}

export interface EmailComposer {
  to: EmailAddress[]
  cc: EmailAddress[]
  bcc: EmailAddress[]
  subject: string
  body: string
  attachments: File[]
  reply_to?: Email
  forward?: Email
  is_draft: boolean
}

export interface EmailFilter {
  folder_id?: number
  is_read?: boolean
  is_flagged?: boolean
  has_attachments?: boolean
  search?: string
  from?: string
  to?: string
  subject?: string
  date_from?: string
  date_to?: string
  labels?: string[]
}

export interface EmailPagination {
  page: number
  limit: number
  total: number
  has_more: boolean
}

export interface EmailListResponse {
  emails: Email[]
  pagination: EmailPagination
}

export interface EmailAccount {
  id: number
  email: string
  name: string
  provider: string
  imap_host?: string
  smtp_host?: string
  is_active: boolean
}

export interface EmailThread {
  id: number
  subject: string
  participants: EmailAddress[]
  message_count: number
  last_message_date: string
  emails: Email[]
  is_read: boolean
}

export interface EmailLabel {
  id: number
  name: string
  color: string
  icon?: string
}

export interface SendEmailRequest {
  to: string[]
  cc?: string[]
  bcc?: string[]
  subject: string
  body_html?: string
  body_text?: string
  reply_to_id?: number
  forward_id?: number
  attachments?: number[]
  is_draft?: boolean
}

export interface EmailQuota {
  used: number
  total: number
  percentage: number
}
