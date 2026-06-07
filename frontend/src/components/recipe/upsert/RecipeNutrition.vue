<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import Card from 'primevue/card'
import Button from 'primevue/button'
import InputNumber from 'primevue/inputnumber'
import InputText from 'primevue/inputtext'
import type { RecipeRequest } from '@/types/recipe.types'

const model = defineModel<RecipeRequest>({ required: true })

const emit = defineEmits<{
  enable: []
  disable: []
}>()

const { t } = useI18n({ useScope: 'global' })
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #content>
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-base font-semibold text-surface-900 dark:text-surface-0">
          {{ t('recipes.fields.nutrition.title') }}
        </h2>
        <Button
          v-if="model.nutrition"
          icon="pi pi-trash"
          severity="danger"
          text
          rounded
          size="small"
          :aria-label="t('recipes.fields.nutrition.remove')"
          @click="emit('disable')"
        />
        <Button
          v-else
          icon="pi pi-plus"
          severity="secondary"
          outlined
          size="small"
          :label="t('recipes.fields.nutrition.add')"
          @click="emit('enable')"
        />
      </div>

      <div v-if="model.nutrition" class="space-y-3">
        <div class="flex gap-2">
          <div class="flex flex-col gap-1 flex-1">
            <label class="text-sm text-surface-600 dark:text-surface-400">
              {{ t('recipes.fields.nutrition.servingSizeValue') }}
            </label>
            <InputNumber
              v-model="model.nutrition.servingSizeValue"
              :min="0"
              :max-fraction-digits="2"
              fluid
            />
          </div>
          <div class="flex flex-col gap-1 w-24">
            <label class="text-sm text-surface-600 dark:text-surface-400">
              {{ t('recipes.fields.nutrition.servingSizeUnit') }}
            </label>
            <InputText v-model="model.nutrition.servingSizeUnit" placeholder="g" />
          </div>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <div
            v-for="field in [
              { key: 'calories', label: t('recipes.fields.nutrition.calories') },
              { key: 'proteinG', label: t('recipes.fields.nutrition.proteinG') },
              { key: 'carbsG', label: t('recipes.fields.nutrition.carbsG') },
              { key: 'sugarG', label: t('recipes.fields.nutrition.sugarG') },
              { key: 'fatG', label: t('recipes.fields.nutrition.fatG') },
              { key: 'saturatedFatG', label: t('recipes.fields.nutrition.saturatedFatG') },
              { key: 'fiberG', label: t('recipes.fields.nutrition.fiberG') },
              { key: 'sodiumMg', label: t('recipes.fields.nutrition.sodiumMg') },
            ]"
            :key="field.key"
            class="flex flex-col gap-1"
          >
            <label class="text-sm text-surface-600 dark:text-surface-400">{{ field.label }}</label>
            <InputNumber
              v-model="(model.nutrition as any)[field.key]"
              :min="0"
              :max-fraction-digits="2"
              fluid
            />
          </div>
        </div>
      </div>

      <p v-else class="text-sm text-surface-400 dark:text-surface-500">
        {{ t('recipes.fields.nutrition.empty') }}
      </p>
    </template>
  </Card>
</template>
