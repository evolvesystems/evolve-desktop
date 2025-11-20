import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    redirect: () => {
      // Check if API is configured
      const apiConfigured = localStorage.getItem('api_configured')
      if (!apiConfigured) {
        return '/welcome'
      }
      return '/dashboard'
    },
  },
  {
    path: '/welcome',
    name: 'welcome',
    component: () => import('@/views/Welcome.vue'),
    meta: {
      requiresAuth: false,
      layout: 'auth',
    },
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('@/views/Login.vue'),
    meta: {
      requiresAuth: false,
      layout: 'auth',
    },
  },
  {
    path: '/dashboard',
    name: 'dashboard',
    component: () => import('@/views/Dashboard.vue'),
    meta: {
      requiresAuth: true,
      title: 'dashboard',
    },
  },
  {
    path: '/email',
    name: 'email',
    component: () => import('@/modules/email/views/EmailView.vue'),
    meta: {
      requiresAuth: true,
      title: 'email',
    },
  },
  {
    path: '/chat',
    name: 'chat',
    component: () => import('@/views/ChatView.vue'),
    meta: {
      requiresAuth: true,
      title: 'chat',
    },
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('@/views/Settings.vue'),
    meta: {
      requiresAuth: true,
      title: 'settings',
    },
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'not-found',
    component: () => import('@/views/NotFound.vue'),
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

// Navigation guards
router.beforeEach((to, from, next) => {
  const authStore = useAuthStore()
  const apiConfigured = localStorage.getItem('api_configured')

  // If API not configured and not going to welcome page, redirect to welcome
  if (!apiConfigured && to.name !== 'welcome') {
    next({ name: 'welcome' })
    return
  }

  // Check if route requires authentication
  if (to.meta.requiresAuth !== false && !authStore.isAuthenticated) {
    // Redirect to login
    next({
      name: 'login',
      query: { redirect: to.fullPath },
    })
    return
  }

  // Redirect to dashboard if already authenticated and trying to access login/welcome
  if ((to.name === 'login' || to.name === 'welcome') && authStore.isAuthenticated) {
    next({ name: 'dashboard' })
    return
  }

  next()
})

export default router
