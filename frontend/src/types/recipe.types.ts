import type { CategoryResponse } from './category.types'
import type { DifficultyResponse } from './difficulty.types'

export interface RecipeRequest {
  title: string
  description: string | null
  categoryId: string | null
  difficultyId: string | null
  prepTime: number | null
  cookTime: number | null
  servings: number
  imageUrl: string | null
  shared: boolean
  steps: RecipeStepRequest[]
  ingredients: RecipeIngredientRequest[]
  nutrition: RecipeNutritionRequest | null
}

export interface RecipeResponse {
  id: string
  userId: string
  category: CategoryResponse | null
  difficulty: DifficultyResponse | null
  title: string
  description: string | null
  prepTime: number | null
  cookTime: number | null
  servings: number
  imageUrl: string | null
  shared: boolean
  steps: RecipeStepResponse[]
  ingredients: RecipeIngredientResponse[]
  nutrition: RecipeNutritionResponse | null
  createdAt: string
  updatedAt: string
}

export interface RecipeStepRequest {
  stepNumber: number
  instructions: string
  duration: number | null
}

export interface RecipeStepResponse {
  id: number
  stepNumber: number
  instructions: string
  duration: number | null
}

export interface RecipeIngredientRequest {
  name: string
  quantity: number | null
  unit: string | null
  notes: string | null
  sortOrder: number
}

export interface RecipeIngredientResponse {
  id: number
  name: string
  quantity: number | null
  unit: string | null
  notes: string | null
  sortOrder: number
}

export interface RecipeNutritionRequest {
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
}

export interface RecipeNutritionResponse {
  id: number
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
}

export type RecipeTemplate = 'default-en' | 'default-es'
