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
        {{ t('recipes.sections.visibility') }}
      </h2>

      <USwitch
        v-model="model.shared"
        :disabled="disabled"
        :label="t('recipes.fields.shared.label')"
        :description="t('recipes.fields.shared.help')"
      />
    </div>
  </UCard>
</template>
