<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { FormField } from '@primevue/forms'
import Card from 'primevue/card'
import Message from 'primevue/message'
import CategorySelect from '@/components/category/SelectorComponent.vue'
import DifficultySelect from '@/components/difficulty/SelectorComponent.vue'

const { t } = useI18n({ useScope: 'global' })
const model = defineModel<{ categoryId: string | null; difficultyId: string | null }>({
  required: true,
})
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #content>
      <div class="p-3 sm:p-4 space-y-4">
        <h2
          class="text-xs font-semibold text-surface-500 dark:text-surface-400 uppercase tracking-wide"
        >
          {{ t('recipes.sections.organisation') }}
        </h2>
        <FormField v-slot="$field" name="categoryId" class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">{{
            t('recipes.fields.category.label')
          }}</label>
          <CategorySelect v-model="model.categoryId" :invalid="$field?.invalid" />
          <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">{{
            $field.error?.message
          }}</Message>
        </FormField>
        <FormField v-slot="$field" name="difficultyId" class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">{{
            t('recipes.fields.difficulty.label')
          }}</label>
          <DifficultySelect v-model="model.difficultyId" :invalid="$field?.invalid" />
          <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">{{
            $field.error?.message
          }}</Message>
        </FormField>
      </div>
    </template>
  </Card>
</template>
