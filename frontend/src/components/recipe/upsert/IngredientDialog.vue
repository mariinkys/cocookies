<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n({ useScope: 'global' })

defineProps<{
  open: boolean
  title: string
  isAdding: boolean
  name: string
  quantity: number | null
  unit: string | null
  notes: string | null
  error: string
}>()

const emit = defineEmits<{
  'update:open': [value: boolean]
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
  <UModal
    :open="open"
    @update:open="emit('update:open', $event)"
    :title="title"
    :ui="{ footer: 'justify-between' }"
    @close="emit('close')"
  >
    <template #body>
      <div class="space-y-4">
        <UFormField :label="t('recipes.fields.ingredients.name')" required>
          <UInput
            :model-value="name"
            @update:model-value="emit('update:name', $event as string)"
            :placeholder="t('recipes.fields.ingredients.namePlaceholder')"
            :color="error ? 'error' : undefined"
            :highlight="!!error"
            :maxlength="150"
            class="w-full"
            autofocus
          />
          <p v-if="error" class="mt-1.5 text-sm text-error">{{ error }}</p>
        </UFormField>

        <div class="grid grid-cols-2 gap-3">
          <UFormField :label="t('recipes.fields.ingredients.quantity')">
            <UInputNumber
              :model-value="quantity ?? undefined"
              @update:model-value="emit('update:quantity', $event ?? null)"
              :placeholder="t('recipes.fields.ingredients.quantityPlaceholder')"
              :min="0"
              :step="0.1"
              class="w-full"
            />
          </UFormField>
          <UFormField :label="t('recipes.fields.ingredients.unit')">
            <UInput
              :model-value="unit ?? ''"
              @update:model-value="emit('update:unit', ($event as string) || null)"
              :placeholder="t('recipes.fields.ingredients.unitPlaceholder')"
              :maxlength="50"
              class="w-full"
            />
          </UFormField>
        </div>

        <UFormField :label="t('recipes.fields.ingredients.notes')">
          <UInput
            :model-value="notes ?? ''"
            @update:model-value="emit('update:notes', ($event as string) || null)"
            :placeholder="t('recipes.fields.ingredients.notesPlaceholder')"
            class="w-full"
          />
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
