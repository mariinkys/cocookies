<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { systemService } from '@/services/system.service'

const frontendVersion = __APP_VERSION__
const serverVersion = ref<string | null>(null)
const error = ref(false)

onMounted(async () => {
  try {
    serverVersion.value = await systemService.getServerVersion()
  } catch {
    error.value = true
  }
})
</script>

<template>
  <div class="flex items-center gap-1.5 text-xs text-surface-400 dark:text-surface-500 select-none">
    <span
      class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-surface-100 dark:bg-surface-800"
      title="Frontend Version"
    >
      <i class="pi pi-desktop" style="font-size: 0.65rem"></i>
      v{{ frontendVersion }}
    </span>

    <span class="text-surface-300 dark:text-surface-600">/</span>

    <span
      class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-surface-100 dark:bg-surface-800"
      :title="error ? 'Could not Reach Server' : 'Server Version'"
    >
      <i class="pi pi-server" style="font-size: 0.65rem"></i>
      <template v-if="serverVersion">v{{ serverVersion }}</template>
      <template v-else-if="error">—</template>
      <i v-else class="pi pi-spinner pi-spin" style="font-size: 0.65rem"></i>
    </span>
  </div>
</template>
