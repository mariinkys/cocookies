<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { FormField } from '@primevue/forms'
import Card from 'primevue/card'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Message from 'primevue/message'

const { t } = useI18n({ useScope: 'global' })
const model = defineModel<{ title: string; description: string | null; imageUrl: string | null }>({
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
          {{ t('recipes.sections.basicInfo') }}
        </h2>
        <FormField v-slot="$field" name="title" class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('recipes.fields.title.label') }} <span class="text-red-500">*</span>
          </label>
          <InputText
            v-model="model.title"
            :placeholder="t('recipes.fields.title.placeholder')"
            :invalid="$field?.invalid"
            fluid
          />
          <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">{{
            $field.error?.message
          }}</Message>
        </FormField>
        <FormField v-slot="$field" name="description" class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">{{
            t('recipes.fields.description.label')
          }}</label>
          <Textarea
            v-model="model.description"
            :placeholder="t('recipes.fields.description.placeholder')"
            :invalid="$field?.invalid"
            rows="3"
            fluid
          />
          <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">{{
            $field.error?.message
          }}</Message>
        </FormField>
        <FormField v-slot="$field" name="imageUrl" class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">{{
            t('recipes.fields.imageUrl.label')
          }}</label>
          <InputText
            v-model="model.imageUrl"
            :placeholder="t('recipes.fields.imageUrl.placeholder')"
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
