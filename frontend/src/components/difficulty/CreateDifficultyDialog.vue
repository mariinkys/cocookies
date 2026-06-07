<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Form, FormField } from '@primevue/forms'
import type { FormResolverOptions, FormSubmitEvent } from '@primevue/forms'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Button from 'primevue/button'
import Message from 'primevue/message'
import { useToast } from 'primevue/usetoast'
import { difficultiesService } from '@/services/difficulties.service'
import type { DifficultyRequest, DifficultyResponse } from '@/types/difficulty.types'

const emit = defineEmits<{ created: [difficulty: DifficultyResponse] }>()

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()

const visible = ref(false)
const loading = ref(false)

const model = ref<DifficultyRequest>({ name: '', description: null })
const initialValues: DifficultyRequest = { name: '', description: null }

function open() {
  model.value = { name: '', description: null }
  visible.value = true
}

defineExpose({ open })

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}
  if (!String(values.name ?? '').trim()) {
    errors.name = [{ message: t('common.validation.nameRequired') }]
  }
  return { errors }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return
  loading.value = true
  try {
    const payload: DifficultyRequest = {
      name: model.value.name.trim(),
      description: model.value.description?.trim() || null,
    }
    const created = await difficultiesService.create(payload)
    toast.add({
      severity: 'success',
      summary: t('common.feedback.created'),
      detail: t('difficulties.messages.created'),
      life: 3000,
    })
    emit('created', created)
    visible.value = false
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('difficulties.messages.createError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <Dialog
    v-model:visible="visible"
    :header="t('difficulties.dialog.title')"
    :style="{ width: '24rem' }"
    modal
    :draggable="false"
  >
    <Form
      v-slot="$form"
      :initialValues
      :resolver
      :validateOnBlur="true"
      :validateOnValueUpdate="true"
      class="flex flex-col gap-4 pt-1"
      @submit="onSubmit"
    >
      <FormField v-slot="$field" name="name" class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('common.fields.name') }}
        </label>
        <InputText
          v-model="model.name"
          :placeholder="t('difficulties.dialog.namePlaceholder')"
          :invalid="$field?.invalid"
          autofocus
          fluid
        />
        <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
          {{ $field.error?.message }}
        </Message>
      </FormField>

      <FormField name="description" class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('common.fields.description') }}
          <span class="text-surface-400 dark:text-surface-500 font-normal ml-1">
            {{ t('common.fields.optional') }}
          </span>
        </label>
        <Textarea
          v-model="model.description"
          :placeholder="t('difficulties.dialog.descriptionPlaceholder')"
          rows="3"
          fluid
          auto-resize
        />
      </FormField>

      <div class="flex justify-end gap-2 pt-1">
        <Button
          :label="t('common.actions.cancel')"
          severity="secondary"
          outlined
          @click="visible = false"
        />
        <Button
          type="submit"
          :label="t('common.actions.create')"
          icon="pi pi-plus"
          :loading="loading"
          :disabled="!!$form.invalid"
        />
      </div>
    </Form>
  </Dialog>
</template>
