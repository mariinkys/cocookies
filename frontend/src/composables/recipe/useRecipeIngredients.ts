import { ref, computed, reactive, type Ref } from 'vue'
import { useI18n } from 'vue-i18n'
import type { RecipeRequest } from '@/types/recipe.types'

export function useRecipeIngredients(model: Ref<RecipeRequest>) {
  const { t } = useI18n({ useScope: 'global' })

  const visible = ref(false)
  const editingIndex = ref<number | null>(null)
  const name = ref('')
  const quantity = ref<number | null>(null)
  const unit = ref<string | null>(null)
  const notes = ref<string | null>(null)
  const error = ref('')

  const isAdding = computed(() => editingIndex.value === null)
  const dialogTitle = computed(() =>
    isAdding.value ? t('recipes.dialogs.addIngredient') : t('recipes.dialogs.editIngredient'),
  )

  function resetFields() {
    name.value = ''
    quantity.value = null
    unit.value = null
    notes.value = null
    error.value = ''
  }

  function openAdd() {
    editingIndex.value = null
    resetFields()
    visible.value = true
  }

  function openEdit(index: number) {
    const ingredient = model.value.ingredients[index]
    if (!ingredient) return
    editingIndex.value = index
    name.value = ingredient.name
    quantity.value = ingredient.quantity
    unit.value = ingredient.unit
    notes.value = ingredient.notes
    error.value = ''
    visible.value = true
  }

  function validate(): boolean {
    if (!name.value.trim()) {
      error.value = t('common.validation.required')
      return false
    }
    error.value = ''
    return true
  }

  function persist() {
    const payload = {
      name: name.value.trim(),
      quantity: quantity.value,
      unit: unit.value,
      notes: notes.value,
    }
    if (editingIndex.value === null) {
      model.value.ingredients.push({ sortOrder: model.value.ingredients.length + 1, ...payload })
    } else {
      const ingredient = model.value.ingredients[editingIndex.value]
      if (ingredient) Object.assign(ingredient, payload)
    }
  }

  function save() {
    if (!validate()) return
    persist()
    if (editingIndex.value === null) {
      resetFields()
    } else {
      visible.value = false
    }
  }

  function saveAndClose() {
    if (!validate()) return
    persist()
    visible.value = false
    resetFields()
  }

  function close() {
    visible.value = false
    resetFields()
    editingIndex.value = null
  }

  function renumber() {
    model.value.ingredients.forEach((ingredient, index) => {
      ingredient.sortOrder = index + 1
    })
  }

  function remove(index: number) {
    if (model.value.ingredients.length <= 1) return
    model.value.ingredients.splice(index, 1)
    renumber()
  }

  return reactive({
    visible,
    name,
    quantity,
    unit,
    notes,
    error,
    isAdding,
    dialogTitle,
    openAdd,
    openEdit,
    save,
    saveAndClose,
    close,
    remove,
  })
}
