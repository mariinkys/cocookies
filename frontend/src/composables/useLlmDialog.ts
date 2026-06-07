import { ref } from 'vue'
import type { Ref } from 'vue'
import type { RecipeRequest } from '@/types/recipe.types'
import type { LlmRecipeResult } from '@/types/llm.types'

export function useLlmDialog(model: Ref<RecipeRequest>) {
  const llmDialogVisible = ref(false)
  const formKey = ref(0)

  function applyLlmImport(recipe: LlmRecipeResult) {
    model.value = {
      title: recipe.title,
      description: recipe.description ?? null,
      categoryId: null,
      difficultyId: null,
      prepTime: recipe.prepTime ?? null,
      cookTime: recipe.cookTime ?? null,
      servings: recipe.servings ?? 1,
      imageUrl: recipe.imageUrl ?? null,
      shared: model.value.shared,
      steps: recipe.steps.map((s) => ({
        stepNumber: s.stepNumber,
        instructions: s.instructions,
        duration: s.duration ?? null,
      })),
      ingredients: recipe.ingredients.map((ing, i) => ({
        name: ing.name,
        quantity: ing.quantity ?? null,
        unit: ing.unit ?? null,
        notes: ing.notes ?? null,
        sortOrder: i,
      })),
      nutrition: recipe.nutrition
        ? {
            servingSizeValue: recipe.nutrition.servingSizeValue,
            servingSizeUnit: recipe.nutrition.servingSizeUnit,
            calories: recipe.nutrition.calories ?? null,
            proteinG: recipe.nutrition.proteinG ?? null,
            carbsG: recipe.nutrition.carbsG ?? null,
            sugarG: recipe.nutrition.sugarG ?? null,
            fatG: recipe.nutrition.fatG ?? null,
            saturatedFatG: recipe.nutrition.saturatedFatG ?? null,
            fiberG: recipe.nutrition.fiberG ?? null,
            sodiumMg: recipe.nutrition.sodiumMg ?? null,
          }
        : null,
    }

    formKey.value++
  }

  return {
    llmDialogVisible,
    formKey,
    applyLlmImport,
  }
}
