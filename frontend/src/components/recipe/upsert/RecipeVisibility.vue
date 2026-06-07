<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import Card from 'primevue/card'
import Checkbox from 'primevue/checkbox'

const { t } = useI18n({ useScope: 'global' })
const model = defineModel<{ shared: boolean }>({ required: true })
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #content>
      <div class="p-3 sm:p-4 space-y-4">
        <h2
          class="text-xs font-semibold text-surface-500 dark:text-surface-400 uppercase tracking-wide"
        >
          {{ t('recipes.sections.visibility') }}
        </h2>
        <div class="flex items-start gap-3">
          <Checkbox v-model="model.shared" input-id="shared" :binary="true" class="mt-0.5" />
          <div>
            <label
              for="shared"
              class="text-sm font-medium text-surface-700 dark:text-surface-300 cursor-pointer"
            >
              {{ t('recipes.fields.shared.label') }}
            </label>
            <p class="text-xs text-surface-400 dark:text-surface-500 mt-0.5">
              {{ t('recipes.fields.shared.help') }}
            </p>
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>
