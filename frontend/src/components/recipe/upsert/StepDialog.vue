<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import Dialog from 'primevue/dialog'
import Textarea from 'primevue/textarea'
import InputNumber from 'primevue/inputnumber'
import Button from 'primevue/button'
import Message from 'primevue/message'

const { t } = useI18n({ useScope: 'global' })

defineProps<{
  visible: boolean
  title: string
  isAdding: boolean
  instructions: string
  duration: number | null
  error: string
}>()

const emit = defineEmits<{
  'update:visible': [value: boolean]
  'update:instructions': [value: string]
  'update:duration': [value: number | null]
  save: []
  saveAndClose: []
  close: []
}>()
</script>

<template>
  <Dialog
    :visible="visible"
    @update:visible="emit('update:visible', $event)"
    :header="title"
    :style="{ width: '95vw', maxWidth: '520px' }"
    :modal="true"
    :closable="true"
    :draggable="false"
  >
    <div class="space-y-4 pt-1">
      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('recipes.fields.steps.instructions') }}
          <span class="text-red-500">*</span>
        </label>
        <Textarea
          :modelValue="instructions"
          @update:modelValue="emit('update:instructions', $event)"
          :placeholder="t('recipes.fields.steps.instructionsPlaceholder')"
          :invalid="!!error"
          rows="4"
          fluid
          autofocus
        />
        <Message v-if="error" severity="error" size="small" variant="simple">
          {{ error }}
        </Message>
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('recipes.fields.steps.duration') }}
        </label>
        <InputNumber
          :modelValue="duration"
          @update:modelValue="emit('update:duration', $event)"
          :placeholder="t('recipes.fields.steps.durationPlaceholder')"
          :min="0"
          :suffix="` ${t('recipes.fields.minutes')}`"
          fluid
        />
      </div>
    </div>

    <template #footer>
      <div class="flex justify-between items-center w-full">
        <Button
          :label="t('common.actions.close')"
          severity="secondary"
          text
          @click="emit('close')"
        />
        <div class="flex gap-2">
          <Button
            v-if="isAdding"
            :label="t('recipes.actions.saveAndClose')"
            severity="secondary"
            outlined
            @click="emit('saveAndClose')"
          />
          <Button
            :label="isAdding ? t('recipes.actions.saveAndAddAnother') : t('common.actions.save')"
            :icon="isAdding ? 'pi pi-plus' : 'pi pi-check'"
            icon-pos="right"
            @click="emit('save')"
          />
        </div>
      </div>
    </template>
  </Dialog>
</template>
