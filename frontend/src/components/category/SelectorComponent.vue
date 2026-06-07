<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import Select from 'primevue/select'
import Button from 'primevue/button'
import type { CategoryResponse } from '@/types/category.types'
import { categoriesService } from '@/services/categories.service'
import CreateCategoryDialog from '@/components/category/CreateCategoryDialog.vue'

const props = defineProps<{ modelValue: string | null; invalid?: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [value: string | null] }>()

const { t } = useI18n({ useScope: 'global' })

const all = ref<CategoryResponse[]>([])
const createDialog = ref<InstanceType<typeof CreateCategoryDialog> | null>(null)

onMounted(async () => {
  all.value = await categoriesService.getSelector()
})

watch(
  () => props.modelValue,
  () => {},
)

function onChange(value: string | null) {
  emit('update:modelValue', value)
}

function onCategoryCreated(category: CategoryResponse) {
  all.value.push(category)
  emit('update:modelValue', category.id)
}
</script>

<template>
  <div class="flex items-center gap-2">
    <Select
      :model-value="modelValue"
      :options="all"
      option-label="name"
      option-value="id"
      :placeholder="t('recipes.fields.category.placeholder')"
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
      :aria-label="t('categories.actions.create')"
      @click="createDialog?.open()"
    />
  </div>

  <CreateCategoryDialog ref="createDialog" @created="onCategoryCreated" />
</template>
