<script setup lang="ts">
import { ref } from 'vue'
import * as v from 'valibot'
import type { FormSubmitEvent } from '@nuxt/ui'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { useRoute, useRouter } from 'vue-router'
import type { LoginPayload } from '@/types/auth.types'
import type { AxiosError } from 'axios'

const { t } = useI18n({ useScope: 'global' })
const auth = useAuthStore()
const router = useRouter()
const route = useRoute()
const toast = useToast()
const loading = ref(false)

const schema = v.object({
  email: v.pipe(
    v.string(),
    v.minLength(1, t('common.validation.emailRequired')),
    v.email(t('common.validation.invalidEmail')),
  ),
  password: v.pipe(v.string(), v.minLength(1, t('common.validation.passwordRequired'))),
})
type Schema = v.InferOutput<typeof schema>

const model = ref<LoginPayload>({
  email: '',
  password: '',
})

async function onSubmit(event: FormSubmitEvent<Schema>) {
  loading.value = true
  model.value = event.data
  try {
    await auth.login(model.value)
    const redirect = (route.query.redirect as string) ?? '/'
    router.push(redirect)
  } catch (e) {
    const err = e as AxiosError<{ message: string }>
    toast.add({
      color: 'error',
      title: t('common.feedback.error'),
      description: err.response?.data?.message ?? t('auth.login.errors.loginFailed'),
    })
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="flex h-full items-center justify-center px-6 transition-colors duration-200">
    <UCard class="w-full max-w-md">
      <div class="space-y-8 p-2">
        <div class="text-center space-y-1">
          <h1 class="text-xl font-semibold text-highlighted">
            {{ t('auth.login.title') }}
          </h1>
          <p class="text-sm text-muted">
            {{ t('auth.login.description') }}
          </p>
        </div>

        <UForm :schema="schema" :state="model" class="space-y-5" @submit="onSubmit">
          <UFormField :label="t('common.fields.email')" name="email" class="flex flex-col gap-1.5">
            <UInput
              v-model="model.email"
              type="email"
              :placeholder="t('common.placeholders.enterYourEmail')"
              autocomplete="email"
              class="w-full"
            />
          </UFormField>

          <UFormField
            :label="t('common.fields.password')"
            name="password"
            class="flex flex-col gap-1.5"
          >
            <UInput
              v-model="model.password"
              type="password"
              :placeholder="t('common.placeholders.enterPassword')"
              autocomplete="current-password"
              class="w-full"
            />
          </UFormField>

          <UButton
            type="submit"
            :label="t('auth.login.submit')"
            :loading="loading"
            trailing-icon="i-lucide-log-in"
            class="w-full justify-center"
            block
          />

          <p class="text-center text-sm text-muted">
            <RouterLink to="/register" class="font-medium text-primary hover:underline">
              {{ t('navigation.items.newAccount') }}
            </RouterLink>
          </p>
        </UForm>
      </div>
    </UCard>
  </div>
</template>
