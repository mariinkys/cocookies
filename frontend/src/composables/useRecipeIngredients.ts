import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Ref } from 'vue'
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

  const dialogTitle = computed(() =>
    editingIndex.value === null
      ? t('recipes.actions.addIngredient')
      : t('recipes.actions.editIngredient'),
  )
  const isAdding = computed(() => editingIndex.value === null)

  function openAdd() {
    editingIndex.value = null
    name.value = ''
    quantity.value = null
    unit.value = null
    notes.value = null
    error.value = ''
    visible.value = true
  }

  function openEdit(index: number) {
    editingIndex.value = index
    name.value = model.value.ingredients[index]!.name
    quantity.value = model.value.ingredients[index]!.quantity
    unit.value = model.value.ingredients[index]!.unit
    notes.value = model.value.ingredients[index]!.notes
    error.value = ''
    visible.value = true
  }

  function save() {
    if (!name.value.trim()) {
      error.value = t('recipes.fields.ingredients.nameRequired')
      return
    }
    error.value = ''
    if (editingIndex.value !== null) {
      model.value.ingredients[editingIndex.value]!.name = name.value
      model.value.ingredients[editingIndex.value]!.quantity = quantity.value
      model.value.ingredients[editingIndex.value]!.unit = unit.value
      model.value.ingredients[editingIndex.value]!.notes = notes.value
      visible.value = false
    } else {
      model.value.ingredients.push({
        name: name.value,
        quantity: quantity.value,
        unit: unit.value,
        notes: notes.value,
        sortOrder: model.value.ingredients.length,
      })
      name.value = ''
      quantity.value = null
      unit.value = null
      notes.value = null
      // Dialog stays open for next ingredient
    }
  }

  function saveAndClose() {
    save()
    if (!error.value) visible.value = false
  }

  function close() {
    visible.value = false
  }

  function remove(index: number) {
    model.value.ingredients.splice(index, 1)
    model.value.ingredients.forEach((ing, i) => (ing.sortOrder = i))
  }

  return {
    visible,
    editingIndex,
    name,
    quantity,
    unit,
    notes,
    error,
    dialogTitle,
    isAdding,
    openAdd,
    openEdit,
    save,
    saveAndClose,
    close,
    remove,
  }
}
