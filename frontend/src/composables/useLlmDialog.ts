import { ref, type Ref } from 'vue'
import type { RecipeRequest } from '@/types/recipe.types'
import type { LlmRecipeResult } from '@/types/llm.types'

export function useLlmDialog(model: Ref<RecipeRequest>) {
  const llmDialogVisible = ref(false)

  function applyLlmImport(recipe: LlmRecipeResult) {
    model.value.title = recipe.title
    model.value.description = recipe.description
    model.value.prepTime = recipe.prepTime
    model.value.cookTime = recipe.cookTime
    model.value.servings = recipe.servings
    model.value.imageUrl = recipe.imageUrl

    model.value.steps = recipe.steps.map((step, index) => ({
      stepNumber: index + 1,
      instructions: step.instructions,
      duration: step.duration,
    }))

    model.value.ingredients = recipe.ingredients.map((ingredient, index) => ({
      name: ingredient.name,
      quantity: ingredient.quantity,
      unit: ingredient.unit,
      notes: ingredient.notes,
      sortOrder: index + 1,
    }))

    model.value.nutrition = recipe.nutrition

    llmDialogVisible.value = false
  }

  return { llmDialogVisible, applyLlmImport }
}
