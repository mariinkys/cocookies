<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { CategoryResponse } from '@/types/category.types'
import { categoriesService } from '@/services/categories.service'
import CreateCategoryDialog from '@/components/category/CreateCategoryDialog.vue'

const { t } = useI18n({ useScope: 'global' })

defineOptions({ inheritAttrs: false })

const modelValue = defineModel<string | null>({ required: true })
defineProps<{ disabled?: boolean }>()

const all = ref<CategoryResponse[]>([])
const loading = ref(false)
const createDialogOpen = ref(false)

const items = computed(() =>
  all.value.map((category) => ({ label: category.name, value: category.id })),
)

onMounted(async () => {
  loading.value = true
  try {
    all.value = await categoriesService.getSelector()
  } finally {
    loading.value = false
  }
})

function onCategoryCreated(category: CategoryResponse) {
  all.value.push(category)
  modelValue.value = category.id
}
</script>

<template>
  <div class="flex items-center gap-2" v-bind="$attrs">
    <USelectMenu
      :model-value="modelValue ?? undefined"
      @update:model-value="modelValue = ($event as string | undefined) ?? null"
      :items="items"
      value-key="value"
      :disabled="disabled || loading"
      :placeholder="t('recipes.fields.category.placeholder')"
      class="flex-1"
    >
      <template v-if="modelValue" #trailing>
        <UButton
          icon="i-lucide-x"
          color="neutral"
          variant="ghost"
          size="xs"
          :aria-label="t('common.actions.clear')"
          @click.stop="
            () => {
              modelValue = null
            }
          "
        />
      </template>
    </USelectMenu>
    <UButton
      icon="i-lucide-plus"
      color="neutral"
      variant="outline"
      :disabled="disabled"
      :aria-label="t('categories.actions.create')"
      @click="
        () => {
          createDialogOpen = true
        }
      "
    />
  </div>

  <CreateCategoryDialog v-model:open="createDialogOpen" @created="onCategoryCreated" />
</template>
