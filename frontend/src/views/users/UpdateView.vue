<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Form, FormField } from '@primevue/forms'
import { useI18n } from 'vue-i18n'
import type { FormResolverOptions, FormSubmitEvent } from '@primevue/forms'
import Card from 'primevue/card'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import Message from 'primevue/message'
import { useToast } from 'primevue/usetoast'
import { usersService } from '@/services/users.service'
import type { UserUpdatePayload, UpdatePasswordPayload } from '@/types/user.types'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const route = useRoute()
const toast = useToast()

const userId = route.params.id as string
const loading = ref(false)
const fetchLoading = ref(true)
const passwordLoading = ref(false)

const model = ref({
  name: '',
  email: '',
})

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

  if (!values.name) {
    errors.name = [{ message: t('users.fields.name.required') }]
  } else if (String(values.name).length > 100) {
    errors.name = [{ message: t('users.fields.name.max') }]
  }

  if (!values.email) {
    errors.email = [{ message: t('users.fields.email.required') }]
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(String(values.email))) {
    errors.email = [{ message: t('users.fields.email.invalid') }]
  }

  return { errors }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return
  loading.value = true
  try {
    const payload: UserUpdatePayload = { name: model.value.name, email: model.value.email }
    await usersService.update(userId, payload)
    toast.add({
      severity: 'success',
      summary: t('common.feedback.saved'),
      detail: t('users.messages.updated'),
      life: 3000,
    })
    router.push('/')
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('users.messages.updateError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

const passwordModel = ref({
  currentPassword: '',
  newPassword: '',
  confirmPassword: '',
})
const passwordError = ref('')

const passwordResolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

  if (!values.currentPassword) {
    errors.currentPassword = [{ message: t('users.passwordCard.fields.currentPassword.required') }]
  }

  if (!values.newPassword) {
    errors.newPassword = [{ message: t('users.passwordCard.fields.newPassword.required') }]
  } else if (String(values.newPassword).length < 8) {
    errors.newPassword = [{ message: t('users.passwordCard.fields.newPassword.min') }]
  }

  if (!values.confirmPassword) {
    errors.confirmPassword = [{ message: t('users.passwordCard.fields.confirmPassword.required') }]
  } else if (values.newPassword !== values.confirmPassword) {
    errors.confirmPassword = [{ message: t('users.passwordCard.fields.confirmPassword.mismatch') }]
  }

  return { errors }
}

async function onPasswordSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return
  passwordLoading.value = true
  passwordError.value = ''
  try {
    const payload: UpdatePasswordPayload = {
      currentPassword: passwordModel.value.currentPassword,
      newPassword: passwordModel.value.newPassword,
    }
    await usersService.updatePassword(userId, payload)
    toast.add({
      severity: 'success',
      summary: t('users.passwordCard.successTitle'),
      detail: t('users.passwordCard.successDetail'),
      life: 3000,
    })
    passwordModel.value = { currentPassword: '', newPassword: '', confirmPassword: '' }
  } catch {
    passwordError.value = t('users.passwordCard.error')
  } finally {
    passwordLoading.value = false
  }
}

onMounted(async () => {
  try {
    const user = await usersService.getById(userId)
    model.value = { name: user.name, email: user.email }
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('users.messages.loadError'),
      life: 3000,
    })
    router.push('/')
  } finally {
    fetchLoading.value = false
  }
})
</script>

<template>
  <div class="p-6 max-w-2xl mx-auto space-y-6">
    <div class="flex items-center justify-between gap-3 flex-wrap">
      <div class="flex items-center gap-3">
        <Button
          icon="pi pi-arrow-left"
          severity="secondary"
          text
          rounded
          :aria-label="t('common.actions.back')"
          @click="router.push('/')"
        />
        <div>
          <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
            {{ t('users.titles.edit') }}
          </h1>
          <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
            {{ t('users.descriptions.edit') }}
          </p>
        </div>
      </div>
    </div>

    <div v-if="fetchLoading" class="flex items-center justify-center py-24">
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
    </div>

    <template v-else>
      <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
        <template #content>
          <Form
            v-slot="$form"
            :initialValues="model"
            :resolver
            :validateOnBlur="true"
            :validateOnValueUpdate="true"
            class="p-2 space-y-6"
            @submit="onSubmit"
          >
            <FormField v-slot="$field" name="name" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('users.fields.name.label') }} <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="model.name"
                :placeholder="t('users.fields.name.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="email" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                Email <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="model.email"
                type="email"
                :placeholder="t('users.fields.email.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <div class="flex items-center justify-end gap-3 pt-2">
              <Button
                :label="t('common.actions.cancel')"
                severity="secondary"
                outlined
                @click="router.push('/')"
              />
              <Button
                type="submit"
                :label="t('common.actions.saveChanges')"
                icon="pi pi-check"
                iconPos="right"
                :loading="loading"
                :disabled="!$form.valid"
              />
            </div>
          </Form>
        </template>
      </Card>

      <!-- Password card -->
      <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
        <template #content>
          <Form
            :initialValues="passwordModel"
            :resolver="passwordResolver"
            :validateOnBlur="true"
            :validateOnValueUpdate="true"
            class="p-2 space-y-4"
            @submit="onPasswordSubmit"
          >
            <div class="flex items-center gap-2">
              <i class="pi pi-lock text-primary-500 dark:text-primary-400"></i>
              <h2
                class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
              >
                {{ t('users.passwordCard.title') }}
              </h2>
            </div>

            <FormField v-slot="$field" name="currentPassword" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('users.passwordCard.fields.currentPassword.label') }}
                <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="passwordModel.currentPassword"
                type="password"
                :placeholder="t('users.passwordCard.fields.currentPassword.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="newPassword" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('users.passwordCard.fields.newPassword.label') }}
                <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="passwordModel.newPassword"
                type="password"
                :placeholder="t('users.passwordCard.fields.newPassword.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="confirmPassword" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('users.passwordCard.fields.confirmPassword.label') }}
                <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="passwordModel.confirmPassword"
                type="password"
                :placeholder="t('users.passwordCard.fields.confirmPassword.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <Message v-if="passwordError" severity="error" size="small" variant="simple">
              {{ passwordError }}
            </Message>

            <div class="flex justify-end pt-1">
              <Button
                type="submit"
                :label="t('users.passwordCard.submit')"
                icon="pi pi-lock"
                iconPos="right"
                :loading="passwordLoading"
              />
            </div>
          </Form>
        </template>
      </Card>
    </template>
  </div>
</template>
