<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import Card from 'primevue/card'
import type { RecipeNutritionResponse } from '@/types/recipe.types'

const props = defineProps<{
  nutrition: RecipeNutritionResponse
  servings: number
}>()

const { t } = useI18n({ useScope: 'global' })

// Per-serving multiplier: stored values are per servingSizeValue unit,
// total = value * servings (user selected how many servings the recipe makes)
const total = computed(() => {
  const n = props.nutrition
  const s = props.servings
  return {
    calories: n.calories != null ? +(n.calories * s).toFixed(1) : null,
    proteinG: n.proteinG != null ? +(n.proteinG * s).toFixed(1) : null,
    carbsG: n.carbsG != null ? +(n.carbsG * s).toFixed(1) : null,
    sugarG: n.sugarG != null ? +(n.sugarG * s).toFixed(1) : null,
    fatG: n.fatG != null ? +(n.fatG * s).toFixed(1) : null,
    saturatedFatG: n.saturatedFatG != null ? +(n.saturatedFatG * s).toFixed(1) : null,
    fiberG: n.fiberG != null ? +(n.fiberG * s).toFixed(1) : null,
    sodiumMg: n.sodiumMg != null ? +(n.sodiumMg * s).toFixed(1) : null,
  }
})

const rows = computed(() =>
  [
    {
      key: 'proteinG',
      labelKey: 'recipes.fields.nutrition.proteinG',
      per: props.nutrition.proteinG,
      tot: total.value.proteinG,
      unit: 'g',
      accent: 'blue',
    },
    {
      key: 'carbsG',
      labelKey: 'recipes.fields.nutrition.carbsG',
      per: props.nutrition.carbsG,
      tot: total.value.carbsG,
      unit: 'g',
      accent: 'yellow',
    },
    {
      key: 'sugarG',
      labelKey: 'recipes.fields.nutrition.sugarG',
      per: props.nutrition.sugarG,
      tot: total.value.sugarG,
      unit: 'g',
      accent: 'orange',
    },
    {
      key: 'fatG',
      labelKey: 'recipes.fields.nutrition.fatG',
      per: props.nutrition.fatG,
      tot: total.value.fatG,
      unit: 'g',
      accent: 'red',
    },
    {
      key: 'saturatedFatG',
      labelKey: 'recipes.fields.nutrition.saturatedFatG',
      per: props.nutrition.saturatedFatG,
      tot: total.value.saturatedFatG,
      unit: 'g',
      accent: 'red',
    },
    {
      key: 'fiberG',
      labelKey: 'recipes.fields.nutrition.fiberG',
      per: props.nutrition.fiberG,
      tot: total.value.fiberG,
      unit: 'g',
      accent: 'green',
    },
    {
      key: 'sodiumMg',
      labelKey: 'recipes.fields.nutrition.sodiumMg',
      per: props.nutrition.sodiumMg,
      tot: total.value.sodiumMg,
      unit: 'mg',
      accent: 'purple',
    },
  ].filter((r) => r.per != null),
)

const accentClasses: Record<string, string> = {
  blue: 'bg-blue-100   dark:bg-blue-900/30   text-blue-700   dark:text-blue-300',
  yellow: 'bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300',
  orange: 'bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300',
  red: 'bg-red-100    dark:bg-red-900/30    text-red-700    dark:text-red-300',
  green: 'bg-green-100  dark:bg-green-900/30  text-green-700  dark:text-green-300',
  purple: 'bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300',
}
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #content>
      <div class="p-2 space-y-4">
        <h2
          class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
        >
          {{ t('recipes.fields.nutrition.title') }}
        </h2>

        <!-- Calories hero row -->
        <div
          v-if="nutrition.calories != null"
          class="rounded-xl bg-primary-50 dark:bg-primary-900/20 border border-primary-100 dark:border-primary-800 p-4 flex items-center justify-between"
        >
          <div>
            <p
              class="text-xs text-primary-500 dark:text-primary-400 font-medium uppercase tracking-wide"
            >
              {{ t('recipes.fields.nutrition.calories') }}
            </p>
            <p
              class="text-3xl font-bold text-primary-700 dark:text-primary-300 tabular-nums leading-none mt-1"
            >
              {{ nutrition.calories }}
              <span class="text-sm font-normal ml-1">kcal</span>
            </p>
            <p class="text-xs text-primary-400 dark:text-primary-500 mt-1">
              {{
                t('recipes.fields.nutrition.perServing', {
                  value: nutrition.servingSizeValue,
                  unit: nutrition.servingSizeUnit,
                })
              }}
            </p>
          </div>
          <div class="text-right">
            <p
              class="text-xs text-primary-400 dark:text-primary-500 font-medium uppercase tracking-wide"
            >
              {{ t('recipes.fields.nutrition.total') }}
            </p>
            <p
              class="text-xl font-bold text-primary-600 dark:text-primary-400 tabular-nums leading-none mt-1"
            >
              {{ total.calories }}
              <span class="text-xs font-normal ml-0.5">kcal</span>
            </p>
            <p class="text-xs text-primary-400 dark:text-primary-500 mt-1">
              {{ t('recipes.fields.nutrition.forServings', { count: servings }) }}
            </p>
          </div>
        </div>

        <!-- Nutrient rows -->
        <div
          v-if="rows.length"
          class="space-y-0 divide-y divide-surface-100 dark:divide-surface-800"
        >
          <div
            v-for="row in rows"
            :key="row.key"
            class="flex items-center justify-between py-2.5 first:pt-0 last:pb-0 gap-3"
          >
            <div class="flex items-center gap-2">
              <span
                class="inline-flex items-center rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide"
                :class="accentClasses[row.accent]"
              >
                {{ row.unit }}
              </span>
              <span class="text-sm text-surface-700 dark:text-surface-300">
                {{ t(row.labelKey) }}
              </span>
            </div>
            <div class="text-right shrink-0">
              <span class="text-sm font-semibold text-surface-900 dark:text-surface-0 tabular-nums">
                {{ row.per }}{{ row.unit }}
              </span>
              <span class="text-xs text-surface-400 dark:text-surface-500 tabular-nums ml-2">
                / {{ row.tot }}{{ row.unit }} {{ t('recipes.fields.nutrition.totalShort') }}
              </span>
            </div>
          </div>
        </div>

        <!-- Serving size footnote -->
        <p
          class="text-[10px] text-surface-400 dark:text-surface-500 border-t border-surface-100 dark:border-surface-800 pt-2"
        >
          {{
            t('recipes.fields.nutrition.footnote', {
              value: nutrition.servingSizeValue,
              unit: nutrition.servingSizeUnit,
              servings,
            })
          }}
        </p>
      </div>
    </template>
  </Card>
</template>
