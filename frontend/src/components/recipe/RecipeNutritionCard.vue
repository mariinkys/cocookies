<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
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
  blue: 'bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300',
  yellow: 'bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300',
  orange: 'bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300',
  red: 'bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300',
  green: 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300',
  purple: 'bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300',
}
</script>

<template>
  <UCard>
    <template #header>
      <h2 class="text-sm font-semibold uppercase tracking-wide text-muted">
        {{ t('recipes.fields.nutrition.title') }}
      </h2>
    </template>

    <div class="space-y-4">
      <div
        v-if="nutrition.calories != null"
        class="flex items-center justify-between rounded-xl border border-primary/15 bg-primary/5 p-4"
      >
        <div>
          <p class="text-xs font-medium uppercase tracking-wide text-primary">
            {{ t('recipes.fields.nutrition.calories') }}
          </p>
          <p class="mt-1 text-3xl font-bold leading-none tabular-nums text-primary">
            {{ nutrition.calories }}
            <span class="ml-1 text-sm font-normal">kcal</span>
          </p>
          <p class="mt-1 text-xs text-primary/70">
            {{
              t('recipes.fields.nutrition.perServing', {
                value: nutrition.servingSizeValue,
                unit: nutrition.servingSizeUnit,
              })
            }}
          </p>
        </div>
        <div class="text-right">
          <p class="text-xs font-medium uppercase tracking-wide text-primary/70">
            {{ t('recipes.fields.nutrition.total') }}
          </p>
          <p class="mt-1 text-xl font-bold leading-none tabular-nums text-primary/90">
            {{ total.calories }}
            <span class="ml-0.5 text-xs font-normal">kcal</span>
          </p>
          <p class="mt-1 text-xs text-primary/70">
            {{ t('recipes.fields.nutrition.forServings', { count: servings }) }}
          </p>
        </div>
      </div>

      <div v-if="rows.length" class="divide-y divide-default">
        <div
          v-for="row in rows"
          :key="row.key"
          class="flex items-center justify-between gap-3 py-2.5 first:pt-0 last:pb-0"
        >
          <div class="flex items-center gap-2">
            <span
              class="inline-flex items-center rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide"
              :class="accentClasses[row.accent]"
            >
              {{ row.unit }}
            </span>
            <span class="text-sm text-toned">
              {{ t(row.labelKey) }}
            </span>
          </div>
          <div class="shrink-0 text-right">
            <span class="text-sm font-semibold tabular-nums text-highlighted">
              {{ row.per }}{{ row.unit }}
            </span>
            <span class="ml-2 text-xs tabular-nums text-dimmed">
              / {{ row.tot }}{{ row.unit }} {{ t('recipes.fields.nutrition.totalShort') }}
            </span>
          </div>
        </div>
      </div>

      <p class="border-t border-default pt-2 text-[10px] text-dimmed">
        {{
          t('recipes.fields.nutrition.footnote', {
            value: nutrition.servingSizeValue,
            unit: nutrition.servingSizeUnit,
            servings,
          })
        }}
      </p>
    </div>
  </UCard>
</template>
