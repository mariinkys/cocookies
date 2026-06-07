<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import Button from 'primevue/button'
import Message from 'primevue/message'

const { t } = useI18n({ useScope: 'global' })

defineProps<{
  ingredients: Array<{
    name: string
    quantity: number | null
    unit: string | null
    notes: string | null
    sortOrder: number
  }>
  formError?: string
}>()

const emit = defineEmits<{
  add: []
  edit: [index: number]
  remove: [index: number]
}>()
</script>

<template>
  <div class="p-3 sm:p-4 space-y-4">
    <div class="flex items-center justify-between">
      <h2
        class="text-xs font-semibold text-surface-500 dark:text-surface-400 uppercase tracking-wide"
      >
        {{ t('recipes.sections.ingredients') }}
      </h2>
      <Button
        icon="pi pi-plus"
        size="small"
        severity="secondary"
        outlined
        rounded
        :aria-label="t('recipes.actions.addIngredient')"
        v-tooltip.left="t('recipes.actions.addIngredient')"
        @click="emit('add')"
      />
    </div>

    <Message v-if="formError" severity="error" size="small" variant="simple">{{
      formError
    }}</Message>

    <div
      v-if="ingredients.length === 0"
      class="flex flex-col items-center justify-center py-8 gap-2 text-surface-400 dark:text-surface-500"
    >
      <i class="pi pi-shopping-cart text-2xl" />
      <p class="text-sm">
        {{
          t('recipes.fields.ingredients.empty', 'No ingredients yet. Add your first ingredient.')
        }}
      </p>
    </div>

    <div v-else class="space-y-2">
      <div
        v-for="(ingredient, index) in ingredients"
        :key="index"
        class="flex items-center gap-3 rounded-lg border border-surface-200 dark:border-surface-700 px-3 py-2.5 group hover:border-surface-300 dark:hover:border-surface-600 transition-colors"
      >
        <p class="flex-1 text-sm text-surface-700 dark:text-surface-300 truncate font-medium">
          {{ ingredient.name }}
        </p>
        <span
          v-if="ingredient.quantity || ingredient.unit"
          class="text-xs text-surface-400 dark:text-surface-500 shrink-0"
        >
          {{ [ingredient.quantity, ingredient.unit].filter(Boolean).join(' ') }}
        </span>
        <span
          v-if="ingredient.notes"
          class="hidden sm:inline text-xs text-surface-400 dark:text-surface-500 shrink-0 italic truncate max-w-32"
        >
          {{ ingredient.notes }}
        </span>
        <div class="flex items-center gap-0.5 shrink-0">
          <Button
            icon="pi pi-pencil"
            size="small"
            text
            rounded
            severity="secondary"
            :aria-label="t('recipes.actions.editIngredient')"
            @click="emit('edit', index)"
          />
          <Button
            icon="pi pi-trash"
            size="small"
            text
            rounded
            severity="danger"
            :disabled="ingredients.length === 1"
            :aria-label="t('recipes.actions.removeIngredient')"
            @click="emit('remove', index)"
          />
        </div>
      </div>
    </div>
  </div>
</template>
