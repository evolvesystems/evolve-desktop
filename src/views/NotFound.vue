<template>
  <div class="min-h-screen flex items-center justify-center bg-base-200 p-4">
    <div class="text-center">
      <h1 class="text-9xl font-bold text-base-content/20">404</h1>
      <h2 class="text-3xl font-bold mt-4">Page Not Found</h2>
      <p class="text-base-content/70 mt-2">The page you're looking for doesn't exist.</p>
      <button @click="() => router.push('/dashboard')" class="btn btn-primary mt-6">
        Go to Dashboard
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()
</script>
