<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import Select from 'primevue/select'
import Button from 'primevue/button'
import type { DifficultyResponse } from '@/types/difficulty.types'
import { difficultiesService } from '@/services/difficulties.service'
import CreateDifficultyDialog from '@/components/difficulty/CreateDifficultyDialog.vue'

const props = defineProps<{ modelValue: string | null; invalid?: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [value: string | null] }>()

const { t } = useI18n({ useScope: 'global' })

const all = ref<DifficultyResponse[]>([])
const createDialog = ref<InstanceType<typeof CreateDifficultyDialog> | null>(null)

onMounted(async () => {
  all.value = await difficultiesService.getSelector()
})

watch(
  () => props.modelValue,
  () => {},
)

function onChange(value: string | null) {
  emit('update:modelValue', value)
}

function onDifficultyCreated(difficulty: DifficultyResponse) {
  all.value.push(difficulty)
  emit('update:modelValue', difficulty.id)
}
</script>

<template>
  <div class="flex items-center gap-2">
    <Select
      :model-value="modelValue"
      :options="all"
      option-label="name"
      option-value="id"
      :placeholder="t('recipes.fields.difficulty.placeholder')"
      filter
      show-clear
      fluid
      :invalid="invalid"
      @change="onChange($event.value)"
    />
    <Button
      icon="pi pi-plus"
      severity="secondary"
      outlined
      :aria-label="t('difficulties.actions.create')"
      @click="createDialog?.open()"
    />
  </div>

  <CreateDifficultyDialog ref="createDialog" @created="onDifficultyCreated" />
</template>
