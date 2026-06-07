import { computed } from 'vue'
import type { Ref } from 'vue'
import type { RecipeRequest } from '@/types/recipe.types'

export function useRecipeNutrition(model: Ref<RecipeRequest>) {
  const hasNutrition = computed(() => model.value.nutrition !== null)

  function enable() {
    model.value.nutrition = {
      servingSizeValue: 100,
      servingSizeUnit: 'g',
      calories: null,
      proteinG: null,
      carbsG: null,
      sugarG: null,
      fatG: null,
      saturatedFatG: null,
      fiberG: null,
      sodiumMg: null,
    }
  }

  function disable() {
    model.value.nutrition = null
  }

  return { hasNutrition, enable, disable }
}
