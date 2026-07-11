<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n({ useScope: 'global' })

defineProps<{
  ingredients: Array<{
    name: string
    quantity: number | null
    unit: string | null
    notes: string | null
    sortOrder: number
  }>
  disabled?: boolean
}>()

const emit = defineEmits<{
  add: []
  edit: [index: number]
  remove: [index: number]
}>()
</script>

<template>
  <div class="space-y-4 p-3 sm:p-4">
    <div class="flex items-center justify-between">
      <h2 class="text-xs font-semibold uppercase tracking-wide text-muted">
        {{ t('recipes.sections.ingredients') }}
      </h2>
      <UButton
        v-if="!disabled"
        icon="i-lucide-plus"
        size="sm"
        color="neutral"
        variant="outline"
        :aria-label="t('recipes.actions.addIngredient')"
        @click="emit('add')"
      />
    </div>

    <div
      v-if="ingredients.length === 0"
      class="flex flex-col items-center justify-center gap-2 py-8 text-dimmed"
    >
      <UIcon name="i-lucide-shopping-cart" class="size-6" />
      <p class="text-sm">{{ t('recipes.fields.ingredients.empty') }}</p>
    </div>

    <div v-else class="space-y-2">
      <div
        v-for="(ingredient, index) in ingredients"
        :key="index"
        class="group flex items-center gap-3 rounded-lg border border-default px-3 py-2.5 transition-colors hover:border-accented"
      >
        <p class="flex-1 truncate text-sm font-medium text-highlighted">
          {{ ingredient.name }}
        </p>
        <span v-if="ingredient.quantity || ingredient.unit" class="shrink-0 text-xs text-dimmed">
          {{ [ingredient.quantity, ingredient.unit].filter(Boolean).join(' ') }}
        </span>
        <span
          v-if="ingredient.notes"
          class="hidden max-w-32 shrink-0 truncate text-xs italic text-dimmed sm:inline"
        >
          {{ ingredient.notes }}
        </span>
        <div v-if="!disabled" class="flex shrink-0 items-center gap-0.5">
          <UButton
            icon="i-lucide-pencil"
            size="sm"
            color="neutral"
            variant="ghost"
            :aria-label="t('recipes.actions.editIngredient')"
            @click="emit('edit', index)"
          />
          <UButton
            icon="i-lucide-trash-2"
            size="sm"
            color="error"
            variant="ghost"
            :disabled="ingredients.length === 1"
            :aria-label="t('recipes.actions.removeIngredient')"
            @click="emit('remove', index)"
          />
        </div>
      </div>
    </div>
  </div>
</template>
