<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n({ useScope: 'global' })

defineProps<{
  open: boolean
  title: string
  isAdding: boolean
  instructions: string
  duration: number | null
  error: string
}>()

const emit = defineEmits<{
  'update:open': [value: boolean]
  'update:instructions': [value: string]
  'update:duration': [value: number | null]
  save: []
  saveAndClose: []
  close: []
}>()
</script>

<template>
  <UModal
    :open="open"
    @update:open="emit('update:open', $event)"
    :title="title"
    :ui="{ footer: 'justify-between' }"
    @close="emit('close')"
  >
    <template #body>
      <div class="space-y-4">
        <UFormField :label="t('recipes.fields.steps.instructions')" required>
          <UTextarea
            :model-value="instructions"
            @update:model-value="emit('update:instructions', $event as string)"
            :placeholder="t('recipes.fields.steps.instructionsPlaceholder')"
            :color="error ? 'error' : undefined"
            :highlight="!!error"
            :rows="4"
            class="w-full"
            autofocus
          />
          <p v-if="error" class="mt-1.5 text-sm text-error">{{ error }}</p>
        </UFormField>

        <UFormField :label="t('recipes.fields.steps.duration')">
          <UInputNumber
            :model-value="duration ?? undefined"
            @update:model-value="emit('update:duration', $event ?? null)"
            :placeholder="t('recipes.fields.steps.durationPlaceholder')"
            :min="0"
            class="w-full"
          />
          <p class="mt-1 text-xs text-dimmed">{{ t('recipes.fields.minutes') }}</p>
        </UFormField>
      </div>
    </template>

    <template #footer>
      <UButton
        :label="t('common.actions.close')"
        color="neutral"
        variant="ghost"
        @click="emit('close')"
      />
      <div class="flex gap-2">
        <UButton
          v-if="isAdding"
          :label="t('recipes.actions.saveAndClose')"
          color="neutral"
          variant="outline"
          @click="emit('saveAndClose')"
        />
        <UButton
          :label="isAdding ? t('recipes.actions.saveAndAddAnother') : t('common.actions.save')"
          :trailing-icon="isAdding ? 'i-lucide-plus' : 'i-lucide-check'"
          @click="emit('save')"
        />
      </div>
    </template>
  </UModal>
</template>
