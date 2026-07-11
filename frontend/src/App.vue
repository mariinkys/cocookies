<script setup lang="ts">
import { onMounted } from 'vue'
import { useThemeConfig } from '@/composables/useThemeConfig'
import { useColorMode } from '@/composables/useColorMode'

const { load } = useThemeConfig()

// oxlint-disable-next-line no-empty-pattern
const {} = useColorMode() // triggers onMounted for dark mode restore

onMounted(() => load())
</script>

<template>
  <UApp>
    <div
      class="flex flex-col h-screen bg-surface-50 dark:bg-surface-950 text-surface-900 dark:text-surface-0 transition-colors duration-200"
    >
      <NavBar />
      <main class="flex-1 overflow-y-auto">
        <RouterView />
      </main>
    </div>
  </UApp>
</template>
