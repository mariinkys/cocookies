import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Ref } from 'vue'
import type { RecipeRequest } from '@/types/recipe.types'

export function useRecipeSteps(model: Ref<RecipeRequest>) {
  const { t } = useI18n({ useScope: 'global' })

  const visible = ref(false)
  const editingIndex = ref<number | null>(null)
  const instructions = ref('')
  const duration = ref<number | null>(null)
  const error = ref('')

  const dialogTitle = computed(() =>
    editingIndex.value === null ? t('recipes.actions.addStep') : t('recipes.actions.editStep'),
  )
  const isAdding = computed(() => editingIndex.value === null)

  function openAdd() {
    editingIndex.value = null
    instructions.value = ''
    duration.value = null
    error.value = ''
    visible.value = true
  }

  function openEdit(index: number) {
    editingIndex.value = index
    instructions.value = model.value.steps[index]!.instructions
    duration.value = model.value.steps[index]!.duration
    error.value = ''
    visible.value = true
  }

  function save() {
    if (!instructions.value.trim()) {
      error.value = t('recipes.fields.steps.instructionsRequired')
      return
    }
    error.value = ''
    if (editingIndex.value !== null) {
      model.value.steps[editingIndex.value]!.instructions = instructions.value
      model.value.steps[editingIndex.value]!.duration = duration.value
      visible.value = false
    } else {
      model.value.steps.push({
        stepNumber: model.value.steps.length + 1,
        instructions: instructions.value,
        duration: duration.value,
      })
      instructions.value = ''
      duration.value = null
      // Dialog stays open for next step
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
    model.value.steps.splice(index, 1)
    model.value.steps.forEach((s, i) => (s.stepNumber = i + 1))
  }

  function move(index: number, direction: 'up' | 'down') {
    const steps = model.value.steps
    const target = direction === 'up' ? index - 1 : index + 1
    if (target < 0 || target >= steps.length) return
    ;[steps[index], steps[target]] = [steps[target]!, steps[index]!]
    steps.forEach((s, i) => (s.stepNumber = i + 1))
  }

  function reorder(fromIndex: number, toIndex: number) {
    const steps = model.value.steps
    const [moved] = steps.splice(fromIndex, 1)
    steps.splice(toIndex, 0, moved!)
    steps.forEach((s, i) => (s.stepNumber = i + 1))
  }

  return {
    visible,
    editingIndex,
    instructions,
    duration,
    error,
    dialogTitle,
    isAdding,
    openAdd,
    openEdit,
    save,
    saveAndClose,
    close,
    remove,
    move,
    reorder,
  }
}
