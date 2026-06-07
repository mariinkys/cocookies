<script setup lang="ts">
import NavBar from './components/NavBar.vue'
import Toast from 'primevue/toast'
</script>

<template>
  <div
    class="flex flex-col h-screen bg-surface-50 dark:bg-surface-950 text-surface-900 dark:text-surface-0 transition-colors duration-200"
  >
    <NavBar />
    <main class="flex-1 overflow-y-auto">
      <RouterView />
    </main>
    <Toast position="bottom-right" />
  </div>
</template>
