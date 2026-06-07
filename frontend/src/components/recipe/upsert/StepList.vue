<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import Button from 'primevue/button'
import Message from 'primevue/message'

const { t } = useI18n({ useScope: 'global' })

defineProps<{
  steps: Array<{ stepNumber: number; instructions: string; duration: number | null }>
  formError?: string
}>()

const emit = defineEmits<{
  add: []
  edit: [index: number]
  remove: [index: number]
  move: [index: number, direction: 'up' | 'down']
  reorder: [fromIndex: number, toIndex: number]
}>()

const dragFromIndex = ref<number | null>(null)
const dragOverIndex = ref<number | null>(null)

function onDragStart(index: number, event: DragEvent) {
  dragFromIndex.value = index
  event.dataTransfer!.effectAllowed = 'move'
}

function onDragOver(index: number, event: DragEvent) {
  event.preventDefault()
  event.dataTransfer!.dropEffect = 'move'
  dragOverIndex.value = index
}

function onDrop(toIndex: number) {
  if (dragFromIndex.value !== null && dragFromIndex.value !== toIndex) {
    emit('reorder', dragFromIndex.value, toIndex)
  }
  dragFromIndex.value = null
  dragOverIndex.value = null
}

function onDragEnd() {
  dragFromIndex.value = null
  dragOverIndex.value = null
}
</script>

<template>
  <div class="p-3 sm:p-4 space-y-4">
    <div class="flex items-center justify-between">
      <h2
        class="text-xs font-semibold text-surface-500 dark:text-surface-400 uppercase tracking-wide"
      >
        {{ t('recipes.sections.steps') }}
      </h2>
      <Button
        icon="pi pi-plus"
        size="small"
        severity="secondary"
        outlined
        rounded
        :aria-label="t('recipes.actions.addStep')"
        v-tooltip.left="t('recipes.actions.addStep')"
        @click="emit('add')"
      />
    </div>

    <Message v-if="formError" severity="error" size="small" variant="simple">{{
      formError
    }}</Message>

    <div
      v-if="steps.length === 0"
      class="flex flex-col items-center justify-center py-8 gap-2 text-surface-400 dark:text-surface-500"
    >
      <i class="pi pi-list text-2xl"></i>
      <p class="text-sm">
        {{ t('recipes.fields.steps.empty') }}
      </p>
    </div>

    <div v-else class="space-y-2">
      <div
        v-for="(step, index) in steps"
        :key="step.stepNumber"
        draggable="true"
        @dragstart="onDragStart(index, $event)"
        @dragover="onDragOver(index, $event)"
        @drop="onDrop(index)"
        @dragend="onDragEnd"
        :class="[
          'flex flex-col rounded-lg border px-3 py-2.5 transition-colors select-none',
          dragOverIndex === index && dragFromIndex !== index
            ? 'border-primary-400 dark:border-primary-500 bg-primary-50 dark:bg-primary-950'
            : 'border-surface-200 dark:border-surface-700 hover:border-surface-300 dark:hover:border-surface-600',
          dragFromIndex === index ? 'opacity-40' : 'opacity-100',
        ]"
      >
        <!-- Top row -->
        <div class="flex items-start gap-3 min-w-0">
          <i
            class="pi pi-bars text-surface-300 dark:text-surface-600 cursor-grab active:cursor-grabbing shrink-0 text-xs"
          ></i>

          <span
            class="w-6 h-6 rounded-full bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 text-xs font-semibold flex items-center justify-center shrink-0"
          >
            {{ step.stepNumber }}
          </span>

          <p class="flex-1 text-sm text-surface-700 dark:text-surface-300 wrap-break-word min-w-0">
            {{ step.instructions }}
          </p>

          <span
            v-if="step.duration"
            class="hidden sm:inline text-xs text-surface-400 dark:text-surface-500 shrink-0"
          >
            {{ step.duration }} {{ t('recipes.fields.minutes') }}
          </span>
        </div>

        <!-- Bottom row -->
        <div class="flex items-center justify-between mt-1.5 pl-9">
          <span
            v-if="step.duration"
            class="sm:hidden text-xs text-surface-400 dark:text-surface-500"
          >
            {{ step.duration }} {{ t('recipes.fields.minutes') }}
          </span>
          <span v-else class="sm:hidden"></span>

          <div class="flex items-center gap-0.5 ml-auto">
            <Button
              icon="pi pi-angle-up"
              size="small"
              text
              rounded
              severity="secondary"
              class="inline-flex"
              :disabled="index === 0"
              :aria-label="t('recipes.actions.moveStepUp')"
              @click="emit('move', index, 'up')"
            />
            <Button
              icon="pi pi-angle-down"
              size="small"
              text
              rounded
              severity="secondary"
              class="inline-flex"
              :disabled="index === steps.length - 1"
              :aria-label="t('recipes.actions.moveStepDown')"
              @click="emit('move', index, 'down')"
            />
            <Button
              icon="pi pi-pencil"
              size="small"
              text
              rounded
              severity="secondary"
              :aria-label="t('recipes.actions.editStep')"
              @click="emit('edit', index)"
            />
            <Button
              icon="pi pi-trash"
              size="small"
              text
              rounded
              severity="danger"
              :disabled="steps.length === 1"
              :aria-label="t('recipes.actions.removeStep')"
              @click="emit('remove', index)"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
