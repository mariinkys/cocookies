import type { Ref } from 'vue'
import type { RecipeRequest, RecipeNutritionRequest } from '@/types/recipe.types'

function createDefaultNutrition(): RecipeNutritionRequest {
  return {
    servingSizeValue: 0,
    servingSizeUnit: '',
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

export function useRecipeNutrition(model: Ref<RecipeRequest>) {
  function enable() {
    model.value.nutrition = createDefaultNutrition()
  }

  function disable() {
    model.value.nutrition = null
  }

  return { enable, disable }
}
