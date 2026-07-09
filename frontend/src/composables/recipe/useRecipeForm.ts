import { ref, computed, type Ref, type ComputedRef } from 'vue'
import * as v from 'valibot'
import type { FormSubmitEvent } from '@nuxt/ui'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import type { AxiosError } from 'axios'
import { recipesService } from '@/services/recipes.service'
import { useAuthStore } from '@/stores/auth'
import type { RecipeRequest, RecipeResponse } from '@/types/recipe.types'

function createDefaultModel(): RecipeRequest {
  return {
    title: '',
    description: null,
    categoryId: null,
    difficultyId: null,
    prepTime: null,
    cookTime: null,
    servings: 1,
    imageUrl: null,
    shared: false,
    steps: [],
    ingredients: [],
    nutrition: null,
  }
}

function mapResponseToRequest(recipe: RecipeResponse): RecipeRequest {
  return {
    title: recipe.title,
    description: recipe.description,
    categoryId: recipe.category?.id ?? null,
    difficultyId: recipe.difficulty?.id ?? null,
    prepTime: recipe.prepTime,
    cookTime: recipe.cookTime,
    servings: recipe.servings,
    imageUrl: recipe.imageUrl,
    shared: recipe.shared,
    steps: recipe.steps.map((step) => ({
      stepNumber: step.stepNumber,
      instructions: step.instructions,
      duration: step.duration,
    })),
    ingredients: recipe.ingredients.map((ingredient) => ({
      name: ingredient.name,
      quantity: ingredient.quantity,
      unit: ingredient.unit,
      notes: ingredient.notes,
      sortOrder: ingredient.sortOrder,
    })),
    nutrition: recipe.nutrition
      ? {
          servingSizeValue: recipe.nutrition.servingSizeValue,
          servingSizeUnit: recipe.nutrition.servingSizeUnit,
          calories: recipe.nutrition.calories,
          proteinG: recipe.nutrition.proteinG,
          carbsG: recipe.nutrition.carbsG,
          sugarG: recipe.nutrition.sugarG,
          fatG: recipe.nutrition.fatG,
          saturatedFatG: recipe.nutrition.saturatedFatG,
          fiberG: recipe.nutrition.fiberG,
          sodiumMg: recipe.nutrition.sodiumMg,
        }
      : null,
  }
}

export function useRecipeForm(recipeId: ComputedRef<string | undefined> | Ref<string | undefined>) {
  const { t } = useI18n({ useScope: 'global' })
  const router = useRouter()
  const toast = useToast()
  const auth = useAuthStore()

  const isEdit = computed(() => !!recipeId.value)

  const model = ref<RecipeRequest>(createDefaultModel())
  const recipeUserId = ref<string | null>(null)
  const isOwnRecipe = computed(() => (isEdit.value ? auth.user?.id === recipeUserId.value : true))

  const loading = ref(false)
  const fetchLoading = ref(!!recipeId.value)
  const deleteLoading = ref(false)
  const isDeleteModalOpen = ref(false)

  const ingredientSchema = v.object({
    name: v.pipe(
      v.string(),
      v.trim(),
      v.minLength(1, t('common.validation.required')),
      v.maxLength(150, t('common.validation.exceedsMax')),
    ),
    quantity: v.nullable(v.number()),
    unit: v.nullable(v.pipe(v.string(), v.maxLength(50, t('common.validation.exceedsMax')))),
    notes: v.nullable(v.string()),
    sortOrder: v.pipe(v.number(), v.integer(t('common.validation.intOnly'))),
  })

  const stepSchema = v.object({
    stepNumber: v.pipe(
      v.number(),
      v.integer(t('common.validation.intOnly')),
      v.minValue(1, t('common.validation.min1')),
    ),
    instructions: v.pipe(v.string(), v.trim(), v.minLength(1, t('common.validation.required'))),
    duration: v.nullable(v.pipe(v.number(), v.integer(t('common.validation.intOnly')))),
  })

  const nutritionSchema = v.object({
    servingSizeValue: v.pipe(
      v.number(t('common.validation.required')),
      v.check((value) => value > 0, t('common.validation.min1')),
    ),
    servingSizeUnit: v.pipe(
      v.string(t('common.validation.required')),
      v.trim(),
      v.minLength(1, t('common.validation.required')),
    ),
    calories: v.nullable(v.number()),
    proteinG: v.nullable(v.number()),
    carbsG: v.nullable(v.number()),
    sugarG: v.nullable(v.number()),
    fatG: v.nullable(v.number()),
    saturatedFatG: v.nullable(v.number()),
    fiberG: v.nullable(v.number()),
    sodiumMg: v.nullable(v.number()),
  })

  const schema = v.object({
    title: v.pipe(
      v.string(),
      v.trim(),
      v.minLength(1, t('common.validation.required')),
      v.maxLength(255, t('common.validation.exceedsMax')),
    ),
    description: v.nullable(v.string()),
    categoryId: v.nullable(v.string()),
    difficultyId: v.nullable(v.string()),
    prepTime: v.nullable(v.pipe(v.number(), v.integer(t('common.validation.intOnly')))),
    cookTime: v.nullable(v.pipe(v.number(), v.integer(t('common.validation.intOnly')))),
    servings: v.pipe(
      v.number(),
      v.integer(t('common.validation.intOnly')),
      v.minValue(1, t('common.validation.min1')),
    ),
    imageUrl: v.nullable(v.string()),
    shared: v.boolean(),
    steps: v.pipe(v.array(stepSchema), v.minLength(1, t('common.validation.required'))),
    ingredients: v.pipe(v.array(ingredientSchema), v.minLength(1, t('common.validation.required'))),
    nutrition: v.nullable(nutritionSchema),
  })
  type Schema = v.InferOutput<typeof schema>

  async function fetchRecipe() {
    if (!isEdit.value) return
    fetchLoading.value = true
    try {
      const recipe = await recipesService.getById(recipeId.value!)
      recipeUserId.value = recipe.userId
      model.value = mapResponseToRequest(recipe)
    } catch {
      toast.add({
        color: 'error',
        title: t('common.feedback.error'),
        description: t('recipes.messages.loadError'),
      })
      router.push('/')
    } finally {
      fetchLoading.value = false
    }
  }

  async function onSubmit(event: FormSubmitEvent<Schema>) {
    loading.value = true
    try {
      const payload = event.data as RecipeRequest
      if (isEdit.value) {
        const saved = await recipesService.update(recipeId.value!, payload)
        toast.add({
          color: 'success',
          title: t('common.feedback.saved'),
          description: t('recipes.messages.updated'),
        })
        router.push(`/recipes/${saved.id}/view`)
      } else {
        await recipesService.create(payload)
        toast.add({
          color: 'success',
          title: t('common.feedback.created'),
          description: t('recipes.messages.created'),
        })
        router.push('/')
      }
    } catch (e) {
      const err = e as AxiosError<{ message: string }>
      toast.add({
        color: 'error',
        title: t('common.feedback.error'),
        description:
          err.response?.data?.message ??
          t(isEdit.value ? 'recipes.messages.updateError' : 'recipes.messages.createError'),
      })
    } finally {
      loading.value = false
    }
  }

  function openDeleteModal() {
    isDeleteModalOpen.value = true
  }

  function closeDeleteModal() {
    isDeleteModalOpen.value = false
  }

  async function confirmDeleteRecipe() {
    if (!recipeId.value) return
    deleteLoading.value = true
    try {
      await recipesService.delete(recipeId.value)
      toast.add({
        color: 'success',
        title: t('common.feedback.deleted'),
        description: t('recipes.deleteDialog.deletedSuccess'),
      })
      isDeleteModalOpen.value = false
      router.push('/')
    } catch {
      toast.add({
        color: 'error',
        title: t('common.feedback.error'),
        description: t('recipes.deleteDialog.error'),
      })
    } finally {
      deleteLoading.value = false
    }
  }

  return {
    model,
    schema,
    recipeUserId,
    isEdit,
    isOwnRecipe,
    loading,
    fetchLoading,
    deleteLoading,
    isDeleteModalOpen,
    fetchRecipe,
    onSubmit,
    openDeleteModal,
    closeDeleteModal,
    confirmDeleteRecipe,
  }
}
