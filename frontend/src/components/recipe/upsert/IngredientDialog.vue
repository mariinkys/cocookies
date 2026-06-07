<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import InputNumber from 'primevue/inputnumber'
import Button from 'primevue/button'
import Message from 'primevue/message'

const { t } = useI18n({ useScope: 'global' })

defineProps<{
  visible: boolean
  title: string
  isAdding: boolean
  name: string
  quantity: number | null
  unit: string | null
  notes: string | null
  error: string
}>()

const emit = defineEmits<{
  'update:visible': [value: boolean]
  'update:name': [value: string]
  'update:quantity': [value: number | null]
  'update:unit': [value: string | null]
  'update:notes': [value: string | null]
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
          {{ t('recipes.fields.ingredients.name') }}
          <span class="text-red-500">*</span>
        </label>
        <InputText
          :modelValue="name"
          @update:modelValue="emit('update:name', $event as string)"
          :placeholder="t('recipes.fields.ingredients.namePlaceholder')"
          :invalid="!!error"
          fluid
          autofocus
        />
        <Message v-if="error" severity="error" size="small" variant="simple">{{ error }}</Message>
      </div>

      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('recipes.fields.ingredients.quantity') }}
          </label>
          <InputNumber
            :modelValue="quantity"
            @update:modelValue="emit('update:quantity', $event)"
            :placeholder="t('recipes.fields.ingredients.quantityPlaceholder')"
            :min="0"
            :max-fraction-digits="3"
            fluid
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('recipes.fields.ingredients.unit') }}
          </label>
          <InputText
            :modelValue="unit ?? ''"
            @update:modelValue="emit('update:unit', $event || null)"
            :placeholder="t('recipes.fields.ingredients.unitPlaceholder')"
            fluid
          />
        </div>
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('recipes.fields.ingredients.notes') }}
        </label>
        <InputText
          :modelValue="notes ?? ''"
          @update:modelValue="emit('update:notes', $event || null)"
          :placeholder="t('recipes.fields.ingredients.notesPlaceholder')"
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
