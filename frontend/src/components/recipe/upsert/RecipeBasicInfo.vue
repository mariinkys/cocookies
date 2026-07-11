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
        {{ t('recipes.sections.basicInfo') }}
      </h2>

      <UFormField :label="t('recipes.fields.title.label')" name="title" required>
        <UInput
          :model-value="model.title"
          @update:model-value="model.title = ($event as string) ?? ''"
          :placeholder="t('recipes.fields.title.placeholder')"
          :disabled="disabled"
          :maxlength="255"
          class="w-full"
        />
      </UFormField>

      <UFormField :label="t('recipes.fields.description.label')" name="description">
        <UTextarea
          :model-value="model.description ?? undefined"
          @update:model-value="model.description = $event || null"
          :placeholder="t('recipes.fields.description.placeholder')"
          :disabled="disabled"
          :rows="3"
          class="w-full"
        />
      </UFormField>

      <UFormField :label="t('recipes.fields.imageUrl.label')" name="imageUrl">
        <UInput
          :model-value="model.imageUrl ?? undefined"
          @update:model-value="model.imageUrl = ($event as string) || null"
          :placeholder="t('recipes.fields.imageUrl.placeholder')"
          :disabled="disabled"
          icon="i-lucide-image"
          class="w-full"
        />
      </UFormField>
    </div>
  </UCard>
</template>
