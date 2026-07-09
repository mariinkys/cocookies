import { ref, computed, reactive, type Ref } from 'vue'
import { useI18n } from 'vue-i18n'
import type { RecipeRequest } from '@/types/recipe.types'

export function useRecipeSteps(model: Ref<RecipeRequest>) {
  const { t } = useI18n({ useScope: 'global' })

  const visible = ref(false)
  const editingIndex = ref<number | null>(null)
  const instructions = ref('')
  const duration = ref<number | null>(null)
  const error = ref('')

  const isAdding = computed(() => editingIndex.value === null)
  const dialogTitle = computed(() =>
    isAdding.value ? t('recipes.dialogs.addStep') : t('recipes.dialogs.editStep'),
  )

  function resetFields() {
    instructions.value = ''
    duration.value = null
    error.value = ''
  }

  function openAdd() {
    editingIndex.value = null
    resetFields()
    visible.value = true
  }

  function openEdit(index: number) {
    const step = model.value.steps[index]
    if (!step) return
    editingIndex.value = index
    instructions.value = step.instructions
    duration.value = step.duration
    error.value = ''
    visible.value = true
  }

  function validate(): boolean {
    if (!instructions.value.trim()) {
      error.value = t('recipes.validation.stepInstructionsRequired')
      return false
    }
    error.value = ''
    return true
  }

  function persist() {
    const payload = { instructions: instructions.value.trim(), duration: duration.value }
    if (editingIndex.value === null) {
      model.value.steps.push({ stepNumber: model.value.steps.length + 1, ...payload })
    } else {
      const step = model.value.steps[editingIndex.value]
      if (step) Object.assign(step, payload)
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
    model.value.steps.forEach((step, index) => {
      step.stepNumber = index + 1
    })
  }

  function remove(index: number) {
    if (model.value.steps.length <= 1) return
    model.value.steps.splice(index, 1)
    renumber()
  }

  function move(index: number, direction: 'up' | 'down') {
    const target = direction === 'up' ? index - 1 : index + 1
    if (target < 0 || target >= model.value.steps.length) return
    const steps = model.value.steps
    const a = steps[index]!
    const b = steps[target]!
    steps[index] = b
    steps[target] = a
    renumber()
  }

  function reorder(fromIndex: number, toIndex: number) {
    const steps = model.value.steps
    const [moved] = steps.splice(fromIndex, 1)
    if (!moved) return
    steps.splice(toIndex, 0, moved)
    renumber()
  }

  return reactive({
    visible,
    instructions,
    duration,
    error,
    isAdding,
    dialogTitle,
    openAdd,
    openEdit,
    save,
    saveAndClose,
    close,
    remove,
    move,
    reorder,
  })
}
