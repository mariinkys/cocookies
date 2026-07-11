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
  <div class="flex items-center gap-1.5 select-none">
    <UBadge color="neutral" variant="subtle" title="Frontend Version">
      <UIcon name="i-lucide-monitor" class="w-3 h-3" />
      v{{ frontendVersion }}
    </UBadge>

    <span class="text-dimmed">/</span>

    <UBadge
      color="neutral"
      variant="subtle"
      :title="error ? 'Could not reach server' : 'Server Version'"
    >
      <UIcon name="i-lucide-server" class="w-3 h-3" />
      <template v-if="serverVersion">v{{ serverVersion }}</template>
      <template v-else-if="error">—</template>
      <UIcon v-else name="i-lucide-loader-circle" class="w-3 h-3 animate-spin" />
    </UBadge>
  </div>
</template>
