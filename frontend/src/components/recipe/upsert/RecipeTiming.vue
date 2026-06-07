<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { FormField } from '@primevue/forms'
import Card from 'primevue/card'
import InputNumber from 'primevue/inputnumber'
import Message from 'primevue/message'

const { t } = useI18n({ useScope: 'global' })
const model = defineModel<{ prepTime: number | null; cookTime: number | null; servings: number }>({
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
          {{ t('recipes.sections.timing') }}
        </h2>
        <div class="grid grid-cols-2 gap-3 sm:grid-cols-1 sm:gap-4">
          <FormField v-slot="$field" name="prepTime" class="flex flex-col gap-1.5">
            <label class="text-sm font-medium text-surface-700 dark:text-surface-300">{{
              t('recipes.fields.prepTime.label')
            }}</label>
            <InputNumber
              v-model="model.prepTime"
              :placeholder="t('recipes.fields.prepTime.placeholder')"
              :suffix="` ${t('recipes.fields.minutes')}`"
              :invalid="$field?.invalid"
              showClear
              fluid
            />
            <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">{{
              $field.error?.message
            }}</Message>
          </FormField>
          <FormField v-slot="$field" name="cookTime" class="flex flex-col gap-1.5">
            <label class="text-sm font-medium text-surface-700 dark:text-surface-300">{{
              t('recipes.fields.cookTime.label')
            }}</label>
            <InputNumber
              v-model="model.cookTime"
              :placeholder="t('recipes.fields.cookTime.placeholder')"
              :suffix="` ${t('recipes.fields.minutes')}`"
              :invalid="$field?.invalid"
              showClear
              fluid
            />
            <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">{{
              $field.error?.message
            }}</Message>
          </FormField>
        </div>
        <FormField v-slot="$field" name="servings" class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('recipes.fields.servings.label') }} <span class="text-red-500">*</span>
          </label>
          <InputNumber
            v-model="model.servings"
            :placeholder="t('recipes.fields.servings.placeholder')"
            :min="1"
            :invalid="$field?.invalid"
            fluid
          />
          <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">{{
            $field.error?.message
          }}</Message>
        </FormField>
      </div>
    </template>
  </Card>
</template>
