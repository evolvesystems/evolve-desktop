<template>
  <!-- EXACT CLONE OF EIQ CHAT INTERFACE -->
  <div class="flex h-full bg-base-100">

    <!-- LEFT SIDEBAR - Channel List (w-72 = 288px exactly like EIQ) -->
    <div class="w-72 bg-base-200 border-r border-base-300 flex flex-col">

      <!-- Sidebar Header -->
      <div class="p-3 border-b border-base-300">
        <div class="flex items-center justify-between mb-2">
          <div class="flex items-center gap-2">
            <span class="text-base font-bold">EvolveChat</span>
          </div>

          <!-- Status Dropdown -->
          <div class="dropdown dropdown-end">
            <label tabindex="0" class="btn btn-ghost btn-xs gap-1">
              <span :class="['w-2 h-2 rounded-full', statusColor]"></span>
              <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
              </svg>
            </label>
            <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-white rounded-box w-40">
              <li><a @click="setStatus('online')">🟢 Online</a></li>
              <li><a @click="setStatus('away')">🟡 Away</a></li>
              <li><a @click="setStatus('dnd')">🔴 Do Not Disturb</a></li>
              <li><a @click="setStatus('offline')">⚪ Offline</a></li>
            </ul>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="flex gap-1">
          <button class="btn btn-primary btn-sm flex-1" title="Create new channel">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"></path>
            </svg>
            Channel
          </button>
          <button class="btn btn-secondary btn-sm flex-1" title="Start direct message">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
            </svg>
            DM
          </button>
        </div>
        <button class="btn btn-ghost btn-xs w-full mt-1" title="View starred messages">
          <svg class="w-4 h-4 text-warning" fill="currentColor" viewBox="0 0 24 24">
            <path d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"></path>
          </svg>
          Starred Messages
        </button>
      </div>

      <!-- Channel List -->
      <div class="flex-1 overflow-y-auto">

        <!-- Loading State -->
        <div v-if="chatStore.isLoading" class="p-4 text-center">
          <span class="loading loading-spinner loading-md"></span>
          <p class="text-sm text-base-content/60 mt-2">Loading channels...</p>
        </div>

        <!-- Error State -->
        <div v-else-if="chatStore.error" class="p-4">
          <div class="alert alert-error">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>{{ chatStore.error }}</span>
          </div>
        </div>

        <!-- Favorites -->
        <div v-if="chatStore.favoriteChannels.length > 0" class="px-2 py-1">
          <div class="text-xs font-semibold text-base-content/60 uppercase tracking-wider mb-1 px-2">
            ⭐ Favorites
          </div>
          <div class="space-y-0.5">
            <a v-for="channelView in chatStore.favoriteChannels"
               :key="channelView.channel.id"
               @click="selectChannel(channelView.channel)"
               :class="['flex items-center gap-2 px-2 py-1 rounded hover:bg-base-300 transition-colors cursor-pointer text-sm',
                        selectedChannel?.id === channelView.channel.id ? 'bg-base-300' : '']">
              <div class="avatar placeholder">
                <div :class="['w-6 h-6 flex items-center justify-center', getChannelAvatarClass(channelView.channel)]">
                  <span class="text-xs font-semibold">{{ getChannelIcon(channelView.channel) }}</span>
                </div>
              </div>
              <span class="flex-1 truncate text-sm">{{ channelView.channel.name }}</span>
              <span v-if="channelView.unreadCount > 0" class="badge badge-primary badge-sm">{{ channelView.unreadCount }}</span>
            </a>
          </div>
        </div>

        <!-- Public Channels -->
        <div v-if="chatStore.publicChannels.length > 0" class="px-2 py-1">
          <div class="text-xs font-semibold text-base-content/60 uppercase tracking-wider mb-1 px-2">
            Channels
          </div>
          <div class="space-y-0.5">
            <a v-for="channelView in chatStore.publicChannels"
               :key="channelView.channel.id"
               @click="selectChannel(channelView.channel)"
               :class="['flex items-center gap-2 px-2 py-1 rounded hover:bg-base-300 transition-colors cursor-pointer text-sm',
                        selectedChannel?.id === channelView.channel.id ? 'bg-base-300' : '']">
              <div class="avatar placeholder">
                <div :class="['w-6 h-6 flex items-center justify-center', getChannelAvatarClass(channelView.channel)]">
                  <span class="text-xs font-semibold">{{ getChannelIcon(channelView.channel) }}</span>
                </div>
              </div>
              <span class="flex-1 truncate text-sm">{{ channelView.channel.name }}</span>
              <span v-if="channelView.unreadCount > 0" class="badge badge-primary badge-sm">{{ channelView.unreadCount }}</span>
            </a>
          </div>
        </div>

        <!-- Private Channels -->
        <div v-if="chatStore.privateChannels.length > 0" class="px-2 py-1">
          <div class="text-xs font-semibold text-base-content/60 uppercase tracking-wider mb-1 px-2">
            Private
          </div>
          <div class="space-y-0.5">
            <a v-for="channelView in chatStore.privateChannels"
               :key="channelView.channel.id"
               @click="selectChannel(channelView.channel)"
               :class="['flex items-center gap-2 px-2 py-1 rounded hover:bg-base-300 transition-colors cursor-pointer text-sm',
                        selectedChannel?.id === channelView.channel.id ? 'bg-base-300' : '']">
              <div class="avatar placeholder">
                <div :class="['w-6 h-6 flex items-center justify-center', getChannelAvatarClass(channelView.channel)]">
                  <span class="text-xs font-semibold">{{ getChannelIcon(channelView.channel) }}</span>
                </div>
              </div>
              <span class="flex-1 truncate text-sm">{{ channelView.channel.name }}</span>
              <span v-if="channelView.unreadCount > 0" class="badge badge-primary badge-sm">{{ channelView.unreadCount }}</span>
            </a>
          </div>
        </div>

        <!-- Direct Messages -->
        <div v-if="chatStore.directMessages.length > 0" class="px-2 py-1">
          <div class="text-xs font-semibold text-base-content/60 uppercase tracking-wider mb-1 px-2">
            Direct Messages
          </div>
          <div class="space-y-0.5">
            <a v-for="channelView in chatStore.directMessages"
               :key="channelView.channel.id"
               @click="selectChannel(channelView.channel)"
               :class="['flex items-center gap-2 px-2 py-1 rounded hover:bg-base-300 transition-colors cursor-pointer text-sm',
                        selectedChannel?.id === channelView.channel.id ? 'bg-base-300' : '']">
              <div class="avatar placeholder">
                <div class="bg-neutral text-neutral-content rounded-full w-6 h-6 flex items-center justify-center">
                  <span class="text-xs font-semibold">{{ getChannelIcon(channelView.channel) }}</span>
                </div>
              </div>
              <span class="flex-1 truncate text-sm">{{ channelView.channel.name }}</span>
              <span v-if="channelView.unreadCount > 0" class="badge badge-primary badge-sm">{{ channelView.unreadCount }}</span>
            </a>
          </div>
        </div>

        <!-- Users List -->
        <div v-if="chatStore.chatUsers.length > 0" class="px-2 py-1">
          <div class="text-xs font-semibold text-base-content/60 uppercase tracking-wider mb-1 px-2">
            Users ({{ chatStore.chatUsers.length }})
          </div>
          <div class="space-y-0.5">
            <a v-for="user in chatStore.chatUsers"
               :key="user.id"
               @click="startDM(user.id)"
               class="flex items-center gap-2 px-2 py-1 rounded hover:bg-base-300 transition-colors cursor-pointer text-sm">
              <div class="avatar placeholder">
                <div class="bg-neutral text-neutral-content rounded-full w-6 h-6 flex items-center justify-center relative">
                  <span class="text-xs font-semibold">{{ chatStore.getInitials(user.fullName) }}</span>
                  <!-- Status indicator -->
                  <span :class="['absolute bottom-0 right-0 w-2 h-2 rounded-full border border-base-200', getUserStatusColor(user.status)]"></span>
                </div>
              </div>
              <span class="flex-1 truncate text-sm">{{ user.fullName }}</span>
            </a>
          </div>
        </div>

      </div>
    </div>

    <!-- MAIN CONTENT - Welcome Screen -->
    <div v-if="!selectedChannel" class="flex-1 flex items-center justify-center bg-base-100">
      <div class="text-center max-w-md px-4">
        <svg class="w-32 h-32 mx-auto text-base-content/20 mb-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
        </svg>
        <h2 class="text-3xl font-bold text-base-content/60 mb-3">Welcome to EvolveChat</h2>
        <p class="text-lg text-base-content/50 mb-8">
          Select a channel or direct message from the sidebar to start chatting
        </p>

        <div class="stats shadow bg-base-200">
          <div class="stat place-items-center">
            <div class="stat-title">Channels</div>
            <div class="stat-value text-primary">{{ chatStore.publicChannels.length + chatStore.privateChannels.length }}</div>
          </div>
          <div class="stat place-items-center">
            <div class="stat-title">Direct Messages</div>
            <div class="stat-value text-secondary">{{ chatStore.directMessages.length }}</div>
          </div>
          <div class="stat place-items-center">
            <div class="stat-title">Users Online</div>
            <div class="stat-value text-accent">{{ chatStore.chatUsers.length }}</div>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="flex-1 flex flex-col">
      <!-- Channel Header -->
      <div class="px-4 py-2 border-b border-base-300 bg-base-100">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <!-- Channel Type Icon -->
            <span v-if="selectedChannel.type === 'public'" class="text-2xl font-bold">#</span>
            <svg v-else-if="selectedChannel.type === 'private'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
            </svg>

            <div>
              <div class="flex items-center gap-2">
                <h1 class="text-lg font-bold">{{ selectedChannel.name }}</h1>
                <!-- Favorite button -->
                <button @click="toggleFavorite" class="btn btn-ghost btn-xs">
                  <svg class="w-4 h-4" :class="isFavorite ? 'text-warning' : 'text-base-content'" :fill="isFavorite ? 'currentColor' : 'none'" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"></path>
                  </svg>
                </button>
              </div>
              <p v-if="selectedChannel.topic" class="text-sm text-base-content/60">{{ selectedChannel.topic }}</p>
            </div>
          </div>

          <!-- Channel Actions Menu -->
          <div class="dropdown dropdown-end">
            <label tabindex="0" class="btn btn-ghost btn-sm">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"></path>
              </svg>
            </label>
            <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-white rounded-box w-52">
              <li><a>View Members</a></li>
              <li><a>Edit Channel</a></li>
              <li><a>Archive Channel</a></li>
            </ul>
          </div>
        </div>

        <!-- Pinned Messages -->
        <div v-if="chatStore.pinnedMessages.length > 0" class="mt-2 p-2 bg-warning/10 border border-warning/20 rounded-lg">
          <div class="text-xs font-semibold text-warning mb-1">📌 PINNED MESSAGES</div>
          <div v-for="pinned in chatStore.pinnedMessages" :key="pinned.id" class="text-sm">
            <span class="font-semibold">{{ pinned.user.fullName }}:</span>
            {{ pinned.content.slice(0, 100) }}{{ pinned.content.length > 100 ? '...' : '' }}
          </div>
        </div>
      </div>

      <!-- Messages Area - WhatsApp/Telegram style -->
      <div ref="messagesContainer" class="flex-1 overflow-y-auto scrollbar-hide px-4 pt-2">
        <!-- Loading State -->
        <div v-if="chatStore.isLoading" class="text-center py-12">
          <span class="loading loading-spinner loading-lg"></span>
        </div>

        <!-- Messages List -->
        <div v-else-if="chatStore.messages.length > 0" class="space-y-1">
          <div v-for="message in chatStore.messages"
               :key="message.id"
               :class="['group transition-colors relative py-0.5', isCurrentUserMessage(message) ? 'flex justify-end' : '']">

            <!-- WhatsApp-style: Left for others, right for current user -->
            <div :class="['flex gap-2 items-end', isCurrentUserMessage(message) ? 'flex-row-reverse' : '']">

              <!-- Avatar -->
              <div class="w-8 flex-shrink-0">
                <div class="avatar">
                  <div class="w-8 h-8 rounded-full">
                    <div v-if="message.user.profileImage" class="w-full h-full">
                      <img :src="message.user.profileImage" :alt="message.user.fullName" />
                    </div>
                    <div v-else :class="[isCurrentUserMessage(message) ? 'bg-primary text-primary-content' : 'bg-neutral text-neutral-content', 'rounded-full w-8 h-8 flex items-center justify-center']">
                      <span class="text-xs font-medium">{{ chatStore.getInitials(message.user.fullName) }}</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Message Bubble Container -->
              <div :class="['flex flex-col max-w-3xl', isCurrentUserMessage(message) ? 'items-end' : '']">
                <!-- Username (only for others) -->
                <div v-if="!isCurrentUserMessage(message)" class="text-xs text-base-content/60 mb-1 px-1">
                  {{ message.user.fullName }}
                </div>

                <!-- Message Content Bubble -->
                <div :class="['rounded-2xl px-3 py-2', isCurrentUserMessage(message) ? 'bg-primary text-primary-content' : 'bg-base-200 text-base-content']">
                  <div v-if="message.is_deleted" class="italic opacity-60 text-sm">
                    This message was deleted
                  </div>
                  <div v-else class="text-sm leading-relaxed whitespace-pre-wrap">
                    {{ message.content }}
                  </div>
                </div>

                <!-- Timestamp -->
                <div class="flex items-center gap-2 mt-0.5 px-1">
                  <div class="text-xs text-base-content/50 flex items-center gap-1">
                    <span>{{ formatTime(message.created_at) }}</span>
                    <span v-if="message.is_edited">(edited)</span>
                    <span v-if="message.is_pinned">📌</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Empty State -->
        <div v-else class="text-center py-12">
          <svg class="w-16 h-16 mx-auto text-base-content/20 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
          </svg>
          <p class="text-base-content/60">No messages yet. Start the conversation!</p>
        </div>
      </div>

      <!-- Message Input -->
      <div class="p-4 border-t border-base-300 bg-base-100">
        <form @submit.prevent="handleSendMessage" class="flex gap-3 items-end">
          <textarea
            v-model="messageInput"
            placeholder="Type a message..."
            class="textarea textarea-bordered flex-1 resize-none"
            rows="1"
            @keydown.enter.exact="handleEnterKey"
          ></textarea>
          <button type="submit" class="btn btn-primary" :disabled="chatStore.isSending || !messageInput.trim()">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
            </svg>
            Send
          </button>
        </form>

        <!-- Message Actions -->
        <div class="flex gap-2 mt-2">
          <button type="button" class="btn btn-ghost btn-sm" title="Add emoji">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
          </button>
          <button type="button" class="btn btn-ghost btn-sm" title="Attach file">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"></path>
            </svg>
          </button>
          <button type="button" class="btn btn-ghost btn-sm" title="Mention user">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"></path>
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from 'vue'
import { useChatStore } from '@/stores/chat'
import { useAuthStore } from '@/stores/auth'
import type { ChatChannel } from '@/services/chatService'

const chatStore = useChatStore()
const authStore = useAuthStore()

const selectedChannel = ref<ChatChannel | null>(null)
const messageInput = ref('')
const messagesContainer = ref<HTMLElement | null>(null)

const statusColor = computed(() => {
  switch (chatStore.userStatus) {
    case 'online': return 'bg-success'
    case 'away': return 'bg-warning'
    case 'dnd': return 'bg-error'
    case 'offline': return 'bg-base-300'
    default: return 'bg-base-300'
  }
})

const isFavorite = computed(() => {
  if (!selectedChannel.value) return false
  return chatStore.favoriteChannels.some(cv => cv.channel.id === selectedChannel.value?.id)
})

// Initialize chat on mount
onMounted(async () => {
  console.log('[ChatView] Initializing chat view...')
  try {
    await chatStore.initialize()
  } catch (error) {
    console.error('[ChatView] Failed to initialize:', error)
  }
})

// Auto-scroll to bottom when messages change
watch(() => chatStore.messages.length, () => {
  nextTick(() => {
    scrollToBottom()
  })
})

function scrollToBottom() {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
}

async function selectChannel(channel: ChatChannel) {
  selectedChannel.value = channel
  await chatStore.selectChannel(channel)
  scrollToBottom()
}

function getChannelAvatarClass(channel: ChatChannel): string {
  if (channel.type === 'dm' || channel.type === 'group_dm') {
    return 'bg-neutral text-neutral-content rounded-full'
  }

  const defaultColor = channel.type === 'private' ? 'secondary' : 'primary'
  const color = channel.color || defaultColor
  return `bg-${color} text-${color}-content rounded`
}

function getChannelIcon(channel: ChatChannel): string {
  if (channel.icon) return channel.icon
  return channel.name.slice(0, 1).toUpperCase()
}

function getUserStatusColor(status: string): string {
  switch (status) {
    case 'online': return 'bg-success'
    case 'away': return 'bg-warning'
    case 'dnd': return 'bg-error'
    default: return 'bg-base-300'
  }
}

function isCurrentUserMessage(message: any): boolean {
  return message.user.id === authStore.user?.id
}

function formatTime(timestamp: string): string {
  const date = new Date(timestamp)
  return date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })
}

async function handleSendMessage() {
  if (!messageInput.value.trim() || chatStore.isSending) return

  const content = messageInput.value.trim()
  messageInput.value = ''

  const success = await chatStore.sendMessage(content)
  if (success) {
    scrollToBottom()
  } else {
    // Restore message on failure
    messageInput.value = content
  }
}

function handleEnterKey(event: KeyboardEvent) {
  if (!event.shiftKey) {
    event.preventDefault()
    handleSendMessage()
  }
}

async function startDM(userId: number) {
  await chatStore.startDirectMessage(userId)
}

async function setStatus(status: 'online' | 'away' | 'dnd' | 'offline') {
  await chatStore.updateStatus(status)
}

async function toggleFavorite() {
  if (!selectedChannel.value) return
  await chatStore.toggleFavorite(selectedChannel.value.id)
}
</script>

<style scoped>
/* Hide scrollbar */
.scrollbar-hide {
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
</style>
