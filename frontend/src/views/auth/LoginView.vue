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
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import type { LoginPayload } from '@/types/auth.types'
import type { AxiosError } from 'axios'
import { useToast } from 'primevue/usetoast'

const { t } = useI18n({ useScope: 'global' })
const auth = useAuthStore()
const router = useRouter()
const route = useRoute()
const toast = useToast()
const loading = ref(false)

const model = ref<LoginPayload>({ email: '', password: '' })

const initialValues = { email: '', password: '' }

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}
  const email = values.email as string
  const password = values.password as string

  if (!email) {
    errors.email = [{ message: t('common.validation.emailRequired') }]
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    errors.email = [{ message: t('common.validation.invalidEmail') }]
  }

  if (!password) {
    errors.password = [{ message: t('common.validation.passwordRequired') }]
  }

  return { errors }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return

  loading.value = true
  try {
    await auth.login(model.value)
    const redirect = (route.query.redirect as string) ?? '/'
    router.push(redirect)
  } catch (e) {
    const err = e as AxiosError<{ message: string }>
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: err.response?.data?.message ?? t('auth.login.errors.loginFailed'),
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
              {{ t('auth.login.title') }}
            </h1>
            <p class="text-sm text-surface-500 dark:text-surface-400">
              {{ t('auth.login.description') }}
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

            <Button
              type="submit"
              :label="t('auth.login.submit')"
              icon="pi pi-sign-in"
              icon-pos="right"
              class="w-full"
              :loading="loading"
              :disabled="!!$form.invalid"
            />

            <p class="text-center text-sm text-surface-500 dark:text-surface-400">
              <RouterLink
                to="/register"
                class="font-medium text-primary-600 dark:text-primary-400 hover:underline"
              >
                {{ t('navigation.items.newAccount') }}
              </RouterLink>
            </p>
          </Form>
        </div>
      </template>
    </Card>
  </div>
</template>
