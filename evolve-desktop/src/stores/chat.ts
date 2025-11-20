import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { chatService, type ChatChannel, type ChatChannelView, type ChatMessage, type ChatUser, type ChatListResponse } from '@/services/chatService'
import { useAuthStore } from './auth'

export const useChatStore = defineStore('chat', () => {
  // State
  const publicChannels = ref<ChatChannelView[]>([])
  const privateChannels = ref<ChatChannelView[]>([])
  const directMessages = ref<ChatChannelView[]>([])
  const favoriteChannels = ref<ChatChannelView[]>([])
  const chatUsers = ref<ChatUser[]>([])
  const selectedChannel = ref<ChatChannel | null>(null)
  const messages = ref<ChatMessage[]>([])
  const pinnedMessages = ref<ChatMessage[]>([])
  const isLoading = ref(false)
  const isSending = ref(false)
  const error = ref<string | null>(null)
  const userStatus = ref<'online' | 'away' | 'dnd' | 'offline'>('online')

  // Computed
  const totalUnread = computed(() => {
    let total = 0
    total += publicChannels.value.reduce((sum, cv) => sum + cv.unreadCount, 0)
    total += privateChannels.value.reduce((sum, cv) => sum + cv.unreadCount, 0)
    total += directMessages.value.reduce((sum, cv) => sum + cv.unreadCount, 0)
    return total
  })

  const allChannels = computed(() => {
    const all: ChatChannelView[] = []
    all.push(...favoriteChannels.value)
    all.push(...publicChannels.value)
    all.push(...privateChannels.value)
    all.push(...directMessages.value)
    return all
  })

  // Actions

  /**
   * Initialize chat module - load all channels and users
   */
  async function initialize(): Promise<void> {
    console.log('[ChatStore] Initializing chat...')
    isLoading.value = true
    error.value = null

    try {
      const data = await chatService.getChatData()

      publicChannels.value = data.public_channels || []
      privateChannels.value = data.private_channels || []
      directMessages.value = data.direct_messages || []
      favoriteChannels.value = data.favorite_channels || []
      chatUsers.value = data.chat_users || []

      console.log('[ChatStore] Loaded chat data:', {
        public: publicChannels.value.length,
        private: privateChannels.value.length,
        dms: directMessages.value.length,
        favorites: favoriteChannels.value.length,
        users: chatUsers.value.length
      })
    } catch (err: any) {
      console.error('[ChatStore] Failed to initialize:', err)
      error.value = err.response?.data?.message || 'Failed to load chat'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Select a channel and load its messages
   */
  async function selectChannel(channel: ChatChannel): Promise<void> {
    console.log('[ChatStore] Selecting channel:', channel.id, channel.name)
    selectedChannel.value = channel
    await loadChannelMessages(channel.id)
  }

  /**
   * Load messages for a channel
   */
  async function loadChannelMessages(channelId: string): Promise<void> {
    isLoading.value = true
    error.value = null

    try {
      const data = await chatService.getChannelMessages(channelId)

      messages.value = data.messages || []
      pinnedMessages.value = data.pinned_messages || []
      selectedChannel.value = data.channel

      console.log('[ChatStore] Loaded', messages.value.length, 'messages for channel', channelId)
    } catch (err: any) {
      console.error('[ChatStore] Failed to load messages:', err)
      error.value = err.response?.data?.message || 'Failed to load messages'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Send a message to the selected channel
   */
  async function sendMessage(content: string, parentMessageId?: string): Promise<boolean> {
    if (!selectedChannel.value) {
      console.error('[ChatStore] No channel selected')
      return false
    }

    isSending.value = true
    error.value = null

    try {
      const message = await chatService.sendMessage(selectedChannel.value.id, {
        content,
        parent_message_id: parentMessageId
      })

      // Add message to list
      messages.value.push(message)

      // Scroll to bottom (handled by component)

      return true
    } catch (err: any) {
      console.error('[ChatStore] Failed to send message:', err)
      error.value = err.response?.data?.message || 'Failed to send message'
      return false
    } finally {
      isSending.value = false
    }
  }

  /**
   * Edit a message
   */
  async function editMessage(messageId: string, content: string): Promise<boolean> {
    try {
      const updatedMessage = await chatService.editMessage(messageId, content)

      // Update message in list
      const index = messages.value.findIndex(m => m.id === messageId)
      if (index !== -1) {
        messages.value[index] = updatedMessage
      }

      return true
    } catch (err: any) {
      console.error('[ChatStore] Failed to edit message:', err)
      error.value = err.response?.data?.message || 'Failed to edit message'
      return false
    }
  }

  /**
   * Delete a message
   */
  async function deleteMessage(messageId: string): Promise<boolean> {
    try {
      await chatService.deleteMessage(messageId)

      // Mark message as deleted in list
      const message = messages.value.find(m => m.id === messageId)
      if (message) {
        message.is_deleted = true
        message.content = ''
      }

      return true
    } catch (err: any) {
      console.error('[ChatStore] Failed to delete message:', err)
      error.value = err.response?.data?.message || 'Failed to delete message'
      return false
    }
  }

  /**
   * Add reaction to a message
   */
  async function addReaction(messageId: string, emoji: string): Promise<boolean> {
    try {
      await chatService.addReaction(messageId, emoji)
      return true
    } catch (err: any) {
      console.error('[ChatStore] Failed to add reaction:', err)
      return false
    }
  }

  /**
   * Toggle favorite on a channel
   */
  async function toggleFavorite(channelId: string): Promise<boolean> {
    try {
      await chatService.toggleFavorite(channelId)

      // Refresh channel list to update favorites
      await initialize()

      return true
    } catch (err: any) {
      console.error('[ChatStore] Failed to toggle favorite:', err)
      return false
    }
  }

  /**
   * Update user status
   */
  async function updateStatus(status: 'online' | 'away' | 'dnd' | 'offline'): Promise<void> {
    try {
      await chatService.updateStatus(status)
      userStatus.value = status
    } catch (err: any) {
      console.error('[ChatStore] Failed to update status:', err)
    }
  }

  /**
   * Start a direct message with a user
   */
  async function startDirectMessage(userId: number): Promise<ChatChannel | null> {
    try {
      const channel = await chatService.startDirectMessage(userId)

      // Refresh channel list
      await initialize()

      // Select the new DM channel
      await selectChannel(channel)

      return channel
    } catch (err: any) {
      console.error('[ChatStore] Failed to start DM:', err)
      error.value = err.response?.data?.message || 'Failed to start direct message'
      return null
    }
  }

  /**
   * Create a new channel
   */
  async function createChannel(name: string, type: 'public' | 'private', topic?: string): Promise<ChatChannel | null> {
    try {
      const channel = await chatService.createChannel({ name, type, topic })

      // Refresh channel list
      await initialize()

      // Select the new channel
      await selectChannel(channel)

      return channel
    } catch (err: any) {
      console.error('[ChatStore] Failed to create channel:', err)
      error.value = err.response?.data?.message || 'Failed to create channel'
      return null
    }
  }

  /**
   * Get display name for a channel
   */
  function getChannelDisplayName(channel: ChatChannel): string {
    const authStore = useAuthStore()
    const currentUser = authStore.user

    // For DM channels, show the other user's name
    if (channel.type === 'dm' || channel.type === 'group_dm') {
      // This is a simplified version - the real implementation would parse
      // the channel members and return the other user's name
      return channel.name
    }

    return channel.name
  }

  /**
   * Get initials for a name
   */
  function getInitials(name: string): string {
    return name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2)
  }

  return {
    // State
    publicChannels,
    privateChannels,
    directMessages,
    favoriteChannels,
    chatUsers,
    selectedChannel,
    messages,
    pinnedMessages,
    isLoading,
    isSending,
    error,
    userStatus,

    // Computed
    totalUnread,
    allChannels,

    // Actions
    initialize,
    selectChannel,
    loadChannelMessages,
    sendMessage,
    editMessage,
    deleteMessage,
    addReaction,
    toggleFavorite,
    updateStatus,
    startDirectMessage,
    createChannel,
    getChannelDisplayName,
    getInitials
  }
})
