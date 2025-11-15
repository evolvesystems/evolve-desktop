/**
 * Email Module Plugin
 *
 * Provides email client functionality with folder management,
 * email composition, and full IMAP/SMTP integration
 */

import type { Component } from 'vue'
import type { RouteRecordRaw } from 'vue-router'
import type { ModulePlugin, ModuleMetadata, ModuleSchema } from '@/core/types/module'

// Lazy load components
const EmailView = () => import('./views/EmailView.vue')

// Module metadata
const metadata: ModuleMetadata = {
  id: 'email',
  name: 'Email',
  version: '1.0.0',
  description: 'Full-featured email client with IMAP/SMTP support',
  icon: 'mail',
  category: 'communication'
}

// Module routes
const routes: RouteRecordRaw[] = [
  {
    path: '/email',
    name: 'email',
    component: EmailView,
    meta: {
      title: 'Email',
      icon: 'mail',
      requiresAuth: true
    }
  }
]

// Module schema (empty for now since we're using API)
const schema: ModuleSchema = {
  version: '1.0.0',
  tables: []
}

// Module plugin implementation
export const emailModule: ModulePlugin = {
  metadata,

  async install() {
    console.log('Email module installed')
  },

  async uninstall() {
    console.log('Email module uninstalled')
  },

  getMainView(): Component {
    return EmailView as any
  },

  getRoutes(): RouteRecordRaw[] {
    return routes
  },

  getSchema(): ModuleSchema {
    return schema
  },

  async search(query: string) {
    // TODO: Implement email search
    return []
  }
}

export default emailModule
