<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { DifficultyResponse } from '@/types/difficulty.types'
import { difficultiesService } from '@/services/difficulties.service'
import CreateDifficultyDialog from '@/components/difficulty/CreateDifficultyDialog.vue'

const { t } = useI18n({ useScope: 'global' })

defineOptions({ inheritAttrs: false })

const modelValue = defineModel<string | null>({ required: true })
defineProps<{ disabled?: boolean }>()

const all = ref<DifficultyResponse[]>([])
const loading = ref(false)
const createDialogOpen = ref(false)

const items = computed(() =>
  all.value.map((difficulty) => ({ label: difficulty.name, value: difficulty.id })),
)

onMounted(async () => {
  loading.value = true
  try {
    all.value = await difficultiesService.getSelector()
  } finally {
    loading.value = false
  }
})

function onDifficultyCreated(difficulty: DifficultyResponse) {
  all.value.push(difficulty)
  modelValue.value = difficulty.id
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
      :placeholder="t('recipes.fields.difficulty.placeholder')"
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
      :aria-label="t('difficulties.actions.create')"
      @click="
        () => {
          createDialogOpen = true
        }
      "
    />
  </div>

  <CreateDifficultyDialog v-model:open="createDialogOpen" @created="onDifficultyCreated" />
</template>
