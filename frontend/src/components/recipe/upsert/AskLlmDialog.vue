<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Message from 'primevue/message'
import SelectButton from 'primevue/selectbutton'
import { llmService } from '@/services/llm.service'
import type { LlmRecipeResult } from '@/types/llm.types'
import { useToast } from 'primevue/usetoast'

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()

defineProps<{ visible: boolean }>()
const emit = defineEmits<{
  'update:visible': [value: boolean]
  import: [recipe: LlmRecipeResult]
}>()

type Mode = 'file' | 'url'

const mode = ref<Mode>('file')
const modeOptions = [
  { label: t('recipes.import.modes.file'), value: 'file' },
  { label: t('recipes.import.modes.url'), value: 'url' },
]

const file = ref<File | null>(null)
const url = ref('')
const loading = ref(false)
const error = ref('')

const fileInputRef = ref<HTMLInputElement | null>(null)

const canSubmit = computed(() => {
  if (loading.value) return false
  if (mode.value === 'url') return url.value.trim().length > 0
  return file.value !== null
})

async function onModeChange() {
  if (mode.value === 'url') {
    await nextTick() // let SelectButton finish rendering its new state...
    mode.value = 'file'
    toast.add({
      severity: 'info',
      summary: t('common.feedback.notAvailable'),
      detail: t('recipes.messages.notAvailable'),
      life: 3000,
    })
  }

  // file.value = null
  // url.value = ''
  // error.value = ''
}

function onFileChange(event: Event) {
  const input = event.target as HTMLInputElement
  file.value = input.files?.[0] ?? null
  error.value = ''
}

function clearFile() {
  file.value = null
  if (fileInputRef.value) fileInputRef.value.value = ''
}

async function submit() {
  error.value = ''
  loading.value = true
  try {
    const prompt = t('recipes.import.prompt')
    let result

    if (mode.value === 'file') {
      result = await llmService.analyzeFile(file.value!, prompt)
    } else {
      result = await llmService.analyzeUrl(url.value.trim(), prompt)
    }

    console.log('LLM result:', JSON.stringify(result, null, 2))

    emit('import', result.recipe)
    emit('update:visible', false)
    reset()
  } catch (e) {
    console.error('LLM import error:', e)
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
  if (fileInputRef.value) fileInputRef.value.value = ''
}

function close() {
  emit('update:visible', false)
  reset()
}
</script>

<template>
  <Dialog
    :visible="visible"
    @update:visible="emit('update:visible', $event)"
    :header="t('recipes.import.title')"
    :style="{ width: '95vw', maxWidth: '480px' }"
    :modal="true"
    :closable="!loading"
    :draggable="false"
    @hide="reset"
  >
    <div class="space-y-4 pt-1">
      <SelectButton
        v-model="mode"
        :options="modeOptions"
        option-label="label"
        option-value="value"
        :allow-empty="false"
        fluid
        @change="onModeChange"
      />

      <div v-if="mode === 'file'" class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('recipes.import.fileLabel') }}
        </label>

        <!-- Hidden native input -->
        <input
          ref="fileInputRef"
          type="file"
          accept="image/*,.pdf"
          class="sr-only"
          @change="onFileChange"
        />

        <div
          v-if="!file"
          class="flex flex-col items-center justify-center gap-2 rounded-lg border-2 border-dashed border-surface-300 dark:border-surface-600 px-4 py-8 cursor-pointer hover:border-primary-400 dark:hover:border-primary-500 transition-colors"
          @click="fileInputRef?.click()"
        >
          <i class="pi pi-upload text-2xl text-surface-400 dark:text-surface-500" />
          <p class="text-sm text-surface-500 dark:text-surface-400 text-center">
            {{ t('recipes.import.filePlaceholder') }}
          </p>
          <p class="text-xs text-surface-400 dark:text-surface-500">
            {{ t('recipes.import.fileHint') }}
          </p>
        </div>

        <div
          v-else
          class="flex items-center gap-3 rounded-lg border border-surface-200 dark:border-surface-700 px-3 py-2.5"
        >
          <i
            :class="[
              'text-lg shrink-0',
              file.type === 'application/pdf'
                ? 'pi pi-file-pdf text-red-500'
                : 'pi pi-image text-primary-500',
            ]"
          ></i>
          <span class="flex-1 text-sm text-surface-700 dark:text-surface-300 truncate">
            {{ file.name }}
          </span>
          <Button
            icon="pi pi-times"
            size="small"
            text
            rounded
            severity="secondary"
            :aria-label="t('common.actions.remove')"
            @click="clearFile"
          />
        </div>
      </div>

      <div v-else class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('recipes.import.urlLabel') }}
        </label>
        <InputText
          v-model="url"
          :placeholder="t('recipes.import.urlPlaceholder')"
          :disabled="loading"
          fluid
        />
      </div>

      <Message v-if="error" severity="error" size="small" variant="simple">
        {{ error }}
      </Message>

      <Message severity="info" size="small" variant="simple">
        {{ t('recipes.import.disclaimer') }}
      </Message>
    </div>

    <template #footer>
      <div class="flex justify-end gap-2 w-full">
        <Button
          :label="t('common.actions.cancel')"
          severity="secondary"
          outlined
          :disabled="loading"
          @click="close"
        />
        <Button
          :label="t('recipes.import.submit')"
          icon="pi pi-sparkles"
          icon-pos="right"
          :loading="loading"
          :disabled="!canSubmit"
          @click="submit"
        />
      </div>
    </template>
  </Dialog>
</template>
