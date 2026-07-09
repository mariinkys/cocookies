<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { RecipeRequest } from '@/types/recipe.types'

const { t } = useI18n({ useScope: 'global' })

defineProps<{ disabled?: boolean }>()
const model = defineModel<RecipeRequest>({ required: true })
const emit = defineEmits<{ enable: []; disable: [] }>()

const nutritionFieldKeys = [
  'calories',
  'proteinG',
  'carbsG',
  'sugarG',
  'fatG',
  'saturatedFatG',
  'fiberG',
  'sodiumMg',
] as const
</script>

<template>
  <UCard>
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-xs font-semibold uppercase tracking-wide text-muted">
          {{ t('recipes.fields.nutrition.title') }}
        </h2>
        <UButton
          v-if="model.nutrition && !disabled"
          icon="i-lucide-trash-2"
          color="error"
          variant="ghost"
          size="sm"
          :aria-label="t('recipes.fields.nutrition.remove')"
          @click="emit('disable')"
        />
        <UButton
          v-else-if="!disabled"
          :label="t('recipes.fields.nutrition.add')"
          icon="i-lucide-plus"
          color="neutral"
          variant="outline"
          size="sm"
          @click="emit('enable')"
        />
      </div>

      <div v-if="model.nutrition" class="space-y-3">
        <div class="flex gap-2">
          <UFormField
            :label="t('recipes.fields.nutrition.servingSizeValue')"
            class="flex-1"
            required
          >
            <UInputNumber
              :model-value="model.nutrition.servingSizeValue || undefined"
              @update:model-value="model.nutrition!.servingSizeValue = $event ?? 0"
              :disabled="disabled"
              :min="0"
              class="w-full"
            />
          </UFormField>
          <UFormField :label="t('recipes.fields.nutrition.servingSizeUnit')" class="w-24" required>
            <UInput
              :model-value="model.nutrition.servingSizeUnit"
              @update:model-value="model.nutrition!.servingSizeUnit = ($event as string) ?? ''"
              :disabled="disabled"
              placeholder="g"
              class="w-full"
            />
          </UFormField>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <UFormField
            v-for="key in nutritionFieldKeys"
            :key="key"
            :label="t(`recipes.fields.nutrition.${key}`)"
          >
            <UInputNumber
              :model-value="model.nutrition[key] ?? undefined"
              @update:model-value="model.nutrition![key] = $event ?? null"
              :disabled="disabled"
              :min="0"
              class="w-full"
            />
          </UFormField>
        </div>
      </div>

      <p v-else class="text-sm text-dimmed">{{ t('recipes.fields.nutrition.empty') }}</p>
    </div>
  </UCard>
</template>
