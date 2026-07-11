<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { RecipeRequest } from '@/types/recipe.types'
import CategorySelect from '@/components/category/CategorySelectorComponent.vue'
import DifficultySelect from '@/components/difficulty/DifficultySelectorComponent.vue'

const { t } = useI18n({ useScope: 'global' })

defineProps<{ disabled?: boolean }>()
const model = defineModel<RecipeRequest>({ required: true })
</script>

<template>
  <UCard>
    <div class="space-y-4">
      <h2 class="text-xs font-semibold uppercase tracking-wide text-muted">
        {{ t('recipes.sections.organisation') }}
      </h2>

      <UFormField :label="t('recipes.fields.category.label')" name="categoryId">
        <CategorySelect v-model="model.categoryId" :disabled="disabled" class="w-full" />
      </UFormField>

      <UFormField :label="t('recipes.fields.difficulty.label')" name="difficultyId">
        <DifficultySelect v-model="model.difficultyId" :disabled="disabled" class="w-full" />
      </UFormField>
    </div>
  </UCard>
</template>
