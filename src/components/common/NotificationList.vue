<template>
  <div class="p-2">
    <div class="flex items-center justify-between px-2 py-1 mb-2">
      <h3 class="font-semibold">Notifications</h3>
      <button
        v-if="notifications.length > 0"
        @click="appStore.markAllNotificationsRead"
        class="btn btn-ghost btn-xs"
      >
        Mark all read
      </button>
    </div>

    <div v-if="notifications.length === 0" class="text-center py-8 text-base-content/50">
      <svg class="w-12 h-12 mx-auto mb-2 opacity-30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
      </svg>
      <p class="text-sm">No notifications</p>
    </div>

    <div v-else class="space-y-1 max-h-80 overflow-auto">
      <div
        v-for="notification in notifications"
        :key="notification.id"
        class="p-3 rounded-lg hover:bg-base-300 transition-colors cursor-pointer"
        :class="{
          'bg-base-300/50': !notification.read,
          'opacity-70': notification.read
        }"
        @click="handleNotificationClick(notification)"
      >
        <div class="flex items-start gap-2">
          <div class="flex-shrink-0 mt-1">
            <div
              class="w-2 h-2 rounded-full"
              :class="{
                'bg-info': notification.type === 'info',
                'bg-success': notification.type === 'success',
                'bg-warning': notification.type === 'warning',
                'bg-error': notification.type === 'error',
              }"
            ></div>
          </div>

          <div class="flex-1 min-w-0">
            <div class="flex items-start justify-between gap-2">
              <h4 class="font-medium text-sm">{{ notification.title }}</h4>
              <button
                @click.stop="appStore.dismissNotification(notification.id)"
                class="btn btn-ghost btn-xs btn-square flex-shrink-0"
              >
                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>

            <p class="text-xs text-base-content/70 mt-1">{{ notification.message }}</p>

            <div class="flex items-center justify-between mt-2">
              <span class="text-xs text-base-content/50">
                {{ formatTimestamp(notification.timestamp) }}
              </span>

              <button
                v-if="notification.action"
                @click.stop="handleAction(notification)"
                class="btn btn-primary btn-xs"
              >
                {{ notification.action.label }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="notifications.length > 0" class="mt-2 pt-2 border-t border-base-300">
      <button
        @click="appStore.clearNotifications"
        class="btn btn-ghost btn-sm btn-block"
      >
        Clear all
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAppStore, type Notification } from '@/stores/app'
import { formatDistanceToNow } from 'date-fns'

const appStore = useAppStore()

const notifications = computed(() => appStore.notifications)

function handleNotificationClick(notification: Notification) {
  if (!notification.read) {
    appStore.markNotificationRead(notification.id)
  }
}

function handleAction(notification: Notification) {
  if (notification.action) {
    notification.action.callback()
    appStore.dismissNotification(notification.id)
  }
}

function formatTimestamp(timestamp: Date) {
  return formatDistanceToNow(timestamp, { addSuffix: true })
}
</script>
