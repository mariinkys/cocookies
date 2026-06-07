import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { ComputedRef } from 'vue'
import type { FormResolverOptions, FormSubmitEvent } from '@primevue/forms'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'
import { useAuthStore } from '@/stores/auth'
import { recipesService } from '@/services/recipes.service'
import type { RecipeRequest } from '@/types/recipe.types'

export function useRecipeForm(recipeId: ComputedRef<string | undefined>) {
  const { t } = useI18n({ useScope: 'global' })
  const router = useRouter()
  const toast = useToast()
  const confirm = useConfirm()
  const auth = useAuthStore()

  const isEdit = computed(() => !!recipeId.value)

  const model = ref<RecipeRequest>({
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
  })

  const recipeUserId = ref<string | null>(null)
  const loading = ref(false)
  const fetchLoading = ref(!!recipeId.value)
  const deleteLoading = ref(false)

  const isOwnRecipe = computed(() => (isEdit.value ? auth.user?.id === recipeUserId.value : true))

  const hasStepErrors = computed(
    () => !model.value.steps.length || model.value.steps.some((s) => !s.instructions.trim()),
  )
  const hasIngredientErrors = computed(
    () =>
      !model.value.ingredients.length || model.value.ingredients.some((ing) => !ing.name.trim()),
  )
  const hasListErrors = computed(() => hasStepErrors.value || hasIngredientErrors.value)

  async function fetchRecipe() {
    if (!isEdit.value) return
    fetchLoading.value = true
    try {
      const recipe = await recipesService.getById(recipeId.value!)
      recipeUserId.value = recipe.userId
      model.value = {
        title: recipe.title,
        description: recipe.description,
        categoryId: recipe.category?.id ?? null,
        difficultyId: recipe.difficulty?.id ?? null,
        prepTime: recipe.prepTime,
        cookTime: recipe.cookTime,
        servings: recipe.servings,
        imageUrl: recipe.imageUrl,
        shared: recipe.shared,
        steps: recipe.steps.map((s) => ({
          stepNumber: s.stepNumber,
          instructions: s.instructions,
          duration: s.duration,
        })),
        ingredients: recipe.ingredients.map((ing) => ({
          name: ing.name,
          quantity: ing.quantity,
          unit: ing.unit,
          notes: ing.notes,
          sortOrder: ing.sortOrder,
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
    } catch {
      toast.add({
        severity: 'error',
        summary: t('common.feedback.error'),
        detail: t('recipes.messages.loadError'),
        life: 3000,
      })
      router.push('/')
    } finally {
      fetchLoading.value = false
    }
  }

  const resolver = ({ values }: FormResolverOptions) => {
    const errors: Record<string, { message: string }[]> = {}

    if (!values.title?.toString().trim()) {
      errors.title = [{ message: t('recipes.fields.title.required') }]
    } else if (values.title.toString().length > 255) {
      errors.title = [{ message: t('recipes.fields.title.max') }]
    }

    const servings = Number(values.servings)
    if (!servings || servings < 1) {
      errors.servings = [{ message: t('recipes.fields.servings.min') }]
    }

    if (!model.value.steps.length) {
      errors.steps = [{ message: t('recipes.fields.steps.required') }]
    } else {
      model.value.steps.forEach((s, i) => {
        if (!s.instructions.trim())
          errors[`step_${i}_instructions`] = [
            { message: t('recipes.fields.steps.instructionsRequired') },
          ]
      })
    }

    if (!model.value.ingredients.length) {
      errors.ingredients = [{ message: t('recipes.fields.ingredients.required') }]
    } else {
      model.value.ingredients.forEach((ing, i) => {
        if (!ing.name.trim())
          errors[`ingredient_${i}_name`] = [
            { message: t('recipes.fields.ingredients.nameRequired') },
          ]
      })
    }

    return { errors }
  }

  async function onSubmit({ valid }: FormSubmitEvent) {
    if (!valid || hasListErrors.value) return
    loading.value = true
    try {
      if (isEdit.value) {
        await recipesService.update(recipeId.value!, model.value)
        toast.add({
          severity: 'success',
          summary: t('common.feedback.saved'),
          detail: t('recipes.messages.updated'),
          life: 3000,
        })
        router.push('/recipes/' + recipeId.value! + '/view')
      } else {
        await recipesService.create(model.value)
        toast.add({
          severity: 'success',
          summary: t('common.feedback.created'),
          detail: t('recipes.messages.created'),
          life: 3000,
        })
        router.push('/')
      }
    } catch {
      toast.add({
        severity: 'error',
        summary: t('common.feedback.error'),
        detail: isEdit.value
          ? t('recipes.messages.updateError')
          : t('recipes.messages.createError'),
        life: 3000,
      })
    } finally {
      loading.value = false
    }
  }

  function confirmDelete() {
    confirm.require({
      message: t('recipes.deleteDialog.messageGeneric'),
      header: t('recipes.deleteDialog.title'),
      icon: 'pi pi-exclamation-triangle',
      rejectProps: { label: t('common.actions.cancel'), severity: 'secondary', outlined: true },
      acceptProps: { label: t('common.actions.delete'), severity: 'danger' },
      accept: async () => {
        deleteLoading.value = true
        try {
          await recipesService.delete(recipeId.value!)
          toast.add({
            severity: 'success',
            summary: t('common.feedback.deleted'),
            detail: t('recipes.deleteDialog.deletedSuccess'),
            life: 3000,
          })
          router.push('/')
        } catch {
          toast.add({
            severity: 'error',
            summary: t('common.feedback.error'),
            detail: t('recipes.deleteDialog.error'),
            life: 3000,
          })
        } finally {
          deleteLoading.value = false
        }
      },
    })
  }

  return {
    model,
    recipeUserId,
    isEdit,
    isOwnRecipe,
    loading,
    fetchLoading,
    deleteLoading,
    hasListErrors,
    fetchRecipe,
    resolver,
    onSubmit,
    confirmDelete,
  }
}
