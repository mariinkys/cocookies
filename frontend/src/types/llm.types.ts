export interface LlmRecipeResult {
  title: string
  description: string | null
  prepTime: number | null
  cookTime: number | null
  servings: number
  imageUrl: string | null
  steps: Array<{
    stepNumber: number
    instructions: string
    duration: number | null
  }>
  ingredients: Array<{
    name: string
    quantity: number | null
    unit: string | null
    notes: string | null
    sortOrder: number
  }>
  nutrition: {
    servingSizeValue: number
    servingSizeUnit: string
    calories: number | null
    proteinG: number | null
    carbsG: number | null
    sugarG: number | null
    fatG: number | null
    saturatedFatG: number | null
    fiberG: number | null
    sodiumMg: number | null
  } | null
}

export interface LlmRequest {
  prompt: string
}

export interface LlmResponse {
  recipe: LlmRecipeResult
  confidence: number
}
