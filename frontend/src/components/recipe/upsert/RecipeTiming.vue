<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { RecipeRequest } from '@/types/recipe.types'

const { t } = useI18n({ useScope: 'global' })

defineProps<{ disabled?: boolean }>()
const model = defineModel<RecipeRequest>({ required: true })
</script>

<template>
  <UCard>
    <div class="space-y-4">
      <h2 class="text-xs font-semibold uppercase tracking-wide text-muted">
        {{ t('recipes.sections.timing') }}
      </h2>

      <div class="grid grid-cols-2 gap-3 sm:grid-cols-1 sm:gap-4">
        <UFormField :label="t('recipes.fields.prepTime.label')" name="prepTime">
          <UInputNumber
            :model-value="model.prepTime ?? undefined"
            @update:model-value="model.prepTime = $event ?? null"
            :placeholder="t('recipes.fields.prepTime.placeholder')"
            :disabled="disabled"
            :min="0"
            class="w-full"
          />
          <p class="mt-1 text-xs text-dimmed">{{ t('recipes.fields.minutes') }}</p>
        </UFormField>

        <UFormField :label="t('recipes.fields.cookTime.label')" name="cookTime">
          <UInputNumber
            :model-value="model.cookTime ?? undefined"
            @update:model-value="model.cookTime = $event ?? null"
            :placeholder="t('recipes.fields.cookTime.placeholder')"
            :disabled="disabled"
            :min="0"
            class="w-full"
          />
          <p class="mt-1 text-xs text-dimmed">{{ t('recipes.fields.minutes') }}</p>
        </UFormField>
      </div>

      <UFormField :label="t('recipes.fields.servings.label')" name="servings" required>
        <UInputNumber
          :model-value="model.servings"
          @update:model-value="model.servings = $event ?? 1"
          :placeholder="t('recipes.fields.servings.placeholder')"
          :disabled="disabled"
          :min="1"
          class="w-full"
        />
      </UFormField>
    </div>
  </UCard>
</template>
