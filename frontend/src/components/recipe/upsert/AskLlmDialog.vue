<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { llmService } from '@/services/llm.service'
import type { LlmRecipeResult } from '@/types/llm.types'

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()

defineProps<{ open: boolean }>()
const emit = defineEmits<{
  'update:open': [value: boolean]
  import: [recipe: LlmRecipeResult]
}>()

type Mode = 'file' | 'url'

const mode = ref<Mode>('file')
const file = ref<File | null>(null)
const url = ref('')
const loading = ref(false)
const error = ref('')

const canSubmit = computed(() => {
  if (loading.value) return false
  return mode.value === 'url' ? url.value.trim().length > 0 : file.value !== null
})

// URL import isn't wired up on the backend yet — keep the option visible so
// people know it's coming, but bounce back to file mode with an explanation.
function selectUrlMode() {
  toast.add({
    color: 'info',
    title: t('common.feedback.notAvailable'),
    description: t('recipes.messages.notAvailable'),
  })
}

async function submit() {
  error.value = ''
  loading.value = true
  try {
    const prompt = t('recipes.import.prompt')
    const result =
      mode.value === 'file'
        ? await llmService.analyzeFile(file.value!, prompt)
        : await llmService.analyzeUrl(url.value.trim(), prompt)

    emit('import', result.recipe)
    emit('update:open', false)
    reset()
  } catch {
    error.value = t('recipes.import.error')
  } finally {
    loading.value = false
  }
}

function reset() {
  file.value = null
  url.value = ''
  error.value = ''
  mode.value = 'file'
}

function close() {
  emit('update:open', false)
  reset()
}
</script>

<template>
  <UModal
    :open="open"
    @update:open="emit('update:open', $event)"
    :title="t('recipes.import.title')"
    :dismissible="!loading"
    :ui="{ footer: 'justify-end' }"
    @close="close"
  >
    <template #body>
      <div class="space-y-4">
        <UFieldGroup class="w-full">
          <UButton
            :label="t('recipes.import.modes.file')"
            icon="i-lucide-file-up"
            class="flex-1 justify-center"
            :color="mode === 'file' ? 'primary' : 'neutral'"
            :variant="mode === 'file' ? 'solid' : 'outline'"
            @click="
              () => {
                mode = 'url'
              }
            "
          />
          <UButton
            :label="t('recipes.import.modes.url')"
            icon="i-lucide-link"
            class="flex-1 justify-center"
            color="neutral"
            variant="outline"
            @click="selectUrlMode"
          />
        </UFieldGroup>

        <div v-if="mode === 'file'" class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-muted">
            {{ t('recipes.import.fileLabel') }}
          </label>
          <UFileUpload
            v-model="file"
            accept="image/*,.pdf"
            icon="i-lucide-upload"
            :label="t('recipes.import.filePlaceholder')"
            :description="t('recipes.import.fileHint')"
            class="min-h-40 w-full"
          />
        </div>

        <div v-else class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-muted">
            {{ t('recipes.import.urlLabel') }}
          </label>
          <UInput
            :model-value="url"
            @update:model-value="url = ($event as string) ?? ''"
            :placeholder="t('recipes.import.urlPlaceholder')"
            :disabled="loading"
            class="w-full"
          />
        </div>

        <p v-if="error" class="text-sm text-error">{{ error }}</p>

        <UAlert
          color="neutral"
          variant="subtle"
          icon="i-lucide-info"
          :description="t('recipes.import.disclaimer')"
        />
      </div>
    </template>

    <template #footer>
      <UButton
        :label="t('common.actions.cancel')"
        color="neutral"
        variant="outline"
        :disabled="loading"
        @click="close"
      />
      <UButton
        :label="t('recipes.import.submit')"
        trailing-icon="i-lucide-sparkles"
        :loading="loading"
        :disabled="!canSubmit"
        @click="submit"
      />
    </template>
  </UModal>
</template>
