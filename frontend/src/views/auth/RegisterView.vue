<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { Form, FormField } from '@primevue/forms'
import type { FormResolverOptions, FormSubmitEvent } from '@primevue/forms'
import Card from 'primevue/card'
import InputText from 'primevue/inputtext'
import Password from 'primevue/password'
import Button from 'primevue/button'
import Message from 'primevue/message'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useToast } from 'primevue/usetoast'
import { usersService } from '@/services/users.service'
import type { RegisterPayload } from '@/types/auth.types'
import type { AxiosError } from 'axios'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const toast = useToast()
const loading = ref(false)

const model = ref<RegisterPayload & { confirmPassword: string }>({
  name: '',
  email: '',
  password: '',
  confirmPassword: '',
})

const initialValues = { name: '', email: '', password: '', confirmPassword: '' }

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}
  const name = values.name as string
  const email = values.email as string
  const password = values.password as string
  const confirmPassword = values.confirmPassword as string

  if (!name?.trim()) {
    errors.name = [{ message: t('common.validation.nameRequired') }]
  }

  if (!email) {
    errors.email = [{ message: t('common.validation.emailRequired') }]
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    errors.email = [{ message: t('common.validation.invalidEmail') }]
  }

  if (!password) {
    errors.password = [{ message: t('common.validation.passwordRequired') }]
  } else if (password.length < 8) {
    errors.password = [{ message: t('common.validation.passwordMinLength') }]
  }

  if (!confirmPassword) {
    errors.confirmPassword = [{ message: t('common.validation.confirmPasswordRequired') }]
  } else if (password && confirmPassword !== password) {
    errors.confirmPassword = [{ message: t('common.validation.passwordsDoNotMatch') }]
  }

  return { errors }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return
  loading.value = true
  try {
    await usersService.create({
      name: model.value.name,
      email: model.value.email,
      password: model.value.password,
    })
    toast.add({
      severity: 'success',
      summary: t('common.feedback.created'),
      detail: t('auth.register.success'),
      life: 3000,
    })
    router.push('/login')
  } catch (e) {
    const err = e as AxiosError<{ message: string }>
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: err.response?.data?.message ?? t('auth.register.errors.registerFailed'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div
    class="flex h-full items-center justify-center bg-surface-50 dark:bg-surface-950 px-6 transition-colors duration-200"
  >
    <Card class="w-full max-w-md border border-surface-200 dark:border-surface-700 shadow-lg">
      <template #content>
        <div class="space-y-8 p-2">
          <div class="text-center space-y-1">
            <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
              {{ t('auth.register.title') }}
            </h1>
            <p class="text-sm text-surface-500 dark:text-surface-400">
              {{ t('auth.register.description') }}
            </p>
          </div>

          <Form
            v-slot="$form"
            :initialValues
            :resolver
            :validateOnBlur="true"
            :validateOnValueUpdate="true"
            class="space-y-5"
            @submit="onSubmit"
          >
            <FormField v-slot="$field" name="name" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('common.fields.name') }}
              </label>
              <InputText
                v-model="model.name"
                :placeholder="t('common.placeholders.enterYourName')"
                autocomplete="name"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="email" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('common.fields.email') }}
              </label>
              <InputText
                v-model="model.email"
                type="email"
                :placeholder="t('common.placeholders.enterYourEmail')"
                autocomplete="email"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="password" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('common.fields.password') }}
              </label>
              <Password
                v-model="model.password"
                :placeholder="t('common.placeholders.enterPassword')"
                :feedback="false"
                :invalid="$field?.invalid"
                toggleMask
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="confirmPassword" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('common.fields.confirmPassword') }}
              </label>
              <Password
                v-model="model.confirmPassword"
                :placeholder="t('common.placeholders.confirmPassword')"
                :feedback="false"
                :invalid="$field?.invalid"
                toggleMask
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <Button
              type="submit"
              :label="t('auth.register.submit')"
              icon="pi pi-user-plus"
              icon-pos="right"
              class="w-full"
              :loading="loading"
              :disabled="!!$form.invalid"
            />

            <p class="text-center text-sm text-surface-500 dark:text-surface-400">
              <RouterLink
                to="/login"
                class="font-medium text-primary-600 dark:text-primary-400 hover:underline"
              >
                {{ t('navigation.items.alreadyHaveAccount') }}
              </RouterLink>
            </p>
          </Form>
        </div>
      </template>
    </Card>
  </div>
</template>
