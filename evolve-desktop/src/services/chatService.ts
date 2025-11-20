import axios from 'axios'

// Types matching the EIQ Manager API
export interface ChatUser {
  id: number
  fullName: string
  email: string
  status: 'online' | 'away' | 'dnd' | 'offline'
  profileImage?: string
}

export interface ChatChannel {
  id: string  // Changed from number to string (UUID)
  name: string
  type: 'public' | 'private' | 'dm' | 'group_dm'
  topic?: string
  color?: string
  icon?: string
  created_at: string
  updated_at: string
}

export interface ChatChannelView {
  channel: ChatChannel
  unreadCount: number
}

export interface ChatMessage {
  id: string  // Changed from number to string (UUID)
  content: string
  user: ChatUser
  channel: ChatChannel
  created_at: string
  updated_at?: string
  is_edited: boolean
  is_deleted: boolean
  is_pinned: boolean
  parent_message_id?: string  // Changed from number to string (UUID)
  reply_count?: number
}

export interface ChatListResponse {
  public_channels: ChatChannelView[]
  private_channels: ChatChannelView[]
  direct_messages: ChatChannelView[]
  favorite_channels: ChatChannelView[]
  chat_users: ChatUser[]
}

export interface ChannelMessagesResponse {
  messages: ChatMessage[]
  channel: ChatChannel
  pinned_messages: ChatMessage[]
}

export interface SendMessageRequest {
  content: string
  parent_message_id?: number
}

class ChatService {
  private baseUrl = '/api/v1/chat'

  /**
   * Get all channels and users for the sidebar
   */
  async getChatData(): Promise<ChatListResponse> {
    try {
      console.log('[ChatService] Fetching chat data...')
      const response = await axios.get(this.baseUrl)
      console.log('[ChatService] Chat data fetched:', response.data)
      return response.data
    } catch (error: any) {
      console.error('[ChatService] Failed to fetch chat data:', error)
      throw error
    }
  }

  /**
   * Get messages for a specific channel
   */
  async getChannelMessages(channelId: string): Promise<ChannelMessagesResponse> {
    try {
      console.log('[ChatService] Fetching messages for channel:', channelId)
      const response = await axios.get(`${this.baseUrl}/channels/${channelId}`)
      console.log('[ChatService] Messages fetched:', response.data.messages?.length, 'messages')
      return response.data
    } catch (error: any) {
      console.error('[ChatService] Failed to fetch channel messages:', error)
      throw error
    }
  }

  /**
   * Send a message to a channel
   */
  async sendMessage(channelId: string, data: SendMessageRequest): Promise<ChatMessage> {
    try {
      console.log('[ChatService] Sending message to channel:', channelId, data)
      const response = await axios.post(`${this.baseUrl}/channels/${channelId}/messages`, data)
      console.log('[ChatService] Message sent:', response.data)
      return response.data
    } catch (error: any) {
      console.error('[ChatService] Failed to send message:', error)
      throw error
    }
  }

  /**
   * Edit a message
   */
  async editMessage(messageId: string, content: string): Promise<ChatMessage> {
    try {
      const response = await axios.patch(`${this.baseUrl}/messages/${messageId}`, { content })
      return response.data
    } catch (error: any) {
      console.error('[ChatService] Failed to edit message:', error)
      throw error
    }
  }

  /**
   * Delete a message
   */
  async deleteMessage(messageId: string): Promise<void> {
    try {
      await axios.delete(`${this.baseUrl}/messages/${messageId}`)
    } catch (error: any) {
      console.error('[ChatService] Failed to delete message:', error)
      throw error
    }
  }

  /**
   * Add reaction to a message
   */
  async addReaction(messageId: string, emoji: string): Promise<void> {
    try {
      await axios.post(`${this.baseUrl}/messages/${messageId}/reactions`, { emoji })
    } catch (error: any) {
      console.error('[ChatService] Failed to add reaction:', error)
      throw error
    }
  }

  /**
   * Toggle favorite on a channel
   */
  async toggleFavorite(channelId: string): Promise<void> {
    try {
      await axios.post(`${this.baseUrl}/channels/${channelId}/favorite`)
    } catch (error: any) {
      console.error('[ChatService] Failed to toggle favorite:', error)
      throw error
    }
  }

  /**
   * Update user status
   */
  async updateStatus(status: 'online' | 'away' | 'dnd' | 'offline'): Promise<void> {
    try {
      await axios.post(`${this.baseUrl}/status`, { status })
    } catch (error: any) {
      console.error('[ChatService] Failed to update status:', error)
      throw error
    }
  }

  /**
   * Start a direct message with a user
   */
  async startDirectMessage(userId: number): Promise<ChatChannel> {
    try {
      const response = await axios.post(`${this.baseUrl}/dm/start`, { user_id: userId })
      return response.data
    } catch (error: any) {
      console.error('[ChatService] Failed to start DM:', error)
      throw error
    }
  }

  /**
   * Create a new channel
   */
  async createChannel(data: { name: string; type: 'public' | 'private'; topic?: string }): Promise<ChatChannel> {
    try {
      const response = await axios.post(`${this.baseUrl}/channels`, data)
      return response.data
    } catch (error: any) {
      console.error('[ChatService] Failed to create channel:', error)
      throw error
    }
  }
}

export const chatService = new ChatService()
