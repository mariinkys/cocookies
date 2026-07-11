<script setup lang="ts">
import { ref, watch } from 'vue'
import * as v from 'valibot'
import type { FormSubmitEvent } from '@nuxt/ui'
import { useI18n } from 'vue-i18n'
import { categoriesService } from '@/services/categories.service'
import type { CategoryRequest, CategoryResponse } from '@/types/category.types'

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()

const props = defineProps<{ open: boolean }>()
const emit = defineEmits<{
  'update:open': [value: boolean]
  created: [category: CategoryResponse]
}>()

const loading = ref(false)

function createDefaultModel(): CategoryRequest {
  return { name: '', description: null }
}

const model = ref<CategoryRequest>(createDefaultModel())

watch(
  () => props.open,
  (isOpen) => {
    if (isOpen) model.value = createDefaultModel()
  },
)

const schema = v.object({
  name: v.pipe(v.string(), v.trim(), v.minLength(1, t('common.validation.nameRequired'))),
  description: v.nullable(v.string()),
})
type Schema = v.InferOutput<typeof schema>

async function onSubmit(event: FormSubmitEvent<Schema>) {
  loading.value = true
  try {
    const payload: CategoryRequest = {
      name: event.data.name,
      description: event.data.description?.trim() || null,
    }
    const created = await categoriesService.create(payload)
    toast.add({
      color: 'success',
      title: t('common.feedback.created'),
      description: t('categories.messages.created'),
    })
    emit('created', created)
    emit('update:open', false)
  } catch {
    toast.add({
      color: 'error',
      title: t('common.feedback.error'),
      description: t('categories.messages.createError'),
    })
  } finally {
    loading.value = false
  }
}

function close() {
  emit('update:open', false)
}
</script>

<template>
  <UModal
    :open="open"
    @update:open="emit('update:open', $event)"
    :title="t('categories.dialog.title')"
    :dismissible="!loading"
  >
    <template #body>
      <UForm :schema="schema" :state="model" class="space-y-4" @submit="onSubmit">
        <UFormField :label="t('common.fields.name')" name="name" required>
          <UInput
            :model-value="model.name"
            @update:model-value="model.name = ($event as string) ?? ''"
            :placeholder="t('categories.dialog.namePlaceholder')"
            autofocus
            class="w-full"
          />
        </UFormField>

        <UFormField
          :label="t('common.fields.description')"
          name="description"
          :hint="t('common.fields.optional')"
        >
          <UTextarea
            :model-value="model.description ?? undefined"
            @update:model-value="model.description = ($event as string) || null"
            :placeholder="t('categories.dialog.descriptionPlaceholder')"
            :rows="3"
            autoresize
            class="w-full"
          />
        </UFormField>

        <div class="flex justify-end gap-2 pt-1">
          <UButton
            :label="t('common.actions.cancel')"
            color="neutral"
            variant="outline"
            :disabled="loading"
            @click="close"
          />
          <UButton
            type="submit"
            :label="t('common.actions.create')"
            trailing-icon="i-lucide-plus"
            :loading="loading"
          />
        </div>
      </UForm>
    </template>
  </UModal>
</template>
