<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n({ useScope: 'global' })

defineProps<{
  steps: Array<{ stepNumber: number; instructions: string; duration: number | null }>
  disabled?: boolean
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
  <div class="space-y-4 p-3 sm:p-4">
    <div class="flex items-center justify-between">
      <h2 class="text-xs font-semibold uppercase tracking-wide text-muted">
        {{ t('recipes.sections.steps') }}
      </h2>
      <UButton
        v-if="!disabled"
        icon="i-lucide-plus"
        size="sm"
        color="neutral"
        variant="outline"
        :aria-label="t('recipes.actions.addStep')"
        @click="emit('add')"
      />
    </div>

    <div
      v-if="steps.length === 0"
      class="flex flex-col items-center justify-center gap-2 py-8 text-dimmed"
    >
      <UIcon name="i-lucide-list" class="size-6" />
      <p class="text-sm">{{ t('recipes.fields.steps.empty') }}</p>
    </div>

    <div v-else class="space-y-2">
      <div
        v-for="(step, index) in steps"
        :key="index"
        :draggable="!disabled"
        @dragstart="onDragStart(index, $event)"
        @dragover="onDragOver(index, $event)"
        @drop="onDrop(index)"
        @dragend="onDragEnd"
        :class="[
          'flex flex-col rounded-lg border px-3 py-2.5 transition-colors select-none',
          dragOverIndex === index && dragFromIndex !== index
            ? 'border-primary bg-primary/5'
            : 'border-default hover:border-accented',
          dragFromIndex === index ? 'opacity-40' : 'opacity-100',
        ]"
      >
        <!-- Top row -->
        <div class="flex min-w-0 items-start gap-3">
          <UIcon
            v-if="!disabled"
            name="i-lucide-grip-vertical"
            class="mt-0.5 size-3.5 shrink-0 cursor-grab text-dimmed active:cursor-grabbing"
          />

          <span
            class="flex size-6 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-semibold text-primary"
          >
            {{ step.stepNumber }}
          </span>

          <p class="min-w-0 flex-1 text-sm wrap-break-word text-muted">
            {{ step.instructions }}
          </p>

          <span v-if="step.duration" class="hidden shrink-0 text-xs text-dimmed sm:inline">
            {{ step.duration }} {{ t('recipes.fields.minutes') }}
          </span>
        </div>

        <!-- Bottom row -->
        <div class="mt-1.5 flex items-center justify-between pl-9">
          <span v-if="step.duration" class="text-xs text-dimmed sm:hidden">
            {{ step.duration }} {{ t('recipes.fields.minutes') }}
          </span>
          <span v-else class="sm:hidden"></span>

          <div v-if="!disabled" class="ml-auto flex items-center gap-0.5">
            <UButton
              icon="i-lucide-chevron-up"
              size="sm"
              color="neutral"
              variant="ghost"
              :disabled="index === 0"
              :aria-label="t('recipes.actions.moveStepUp')"
              @click="emit('move', index, 'up')"
            />
            <UButton
              icon="i-lucide-chevron-down"
              size="sm"
              color="neutral"
              variant="ghost"
              :disabled="index === steps.length - 1"
              :aria-label="t('recipes.actions.moveStepDown')"
              @click="emit('move', index, 'down')"
            />
            <UButton
              icon="i-lucide-pencil"
              size="sm"
              color="neutral"
              variant="ghost"
              :aria-label="t('recipes.actions.editStep')"
              @click="emit('edit', index)"
            />
            <UButton
              icon="i-lucide-trash-2"
              size="sm"
              color="error"
              variant="ghost"
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
