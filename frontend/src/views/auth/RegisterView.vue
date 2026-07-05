<script setup lang="ts">
import { ref } from 'vue'
import * as v from 'valibot'
import type { FormSubmitEvent } from '@nuxt/ui'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { usersService } from '@/services/users.service'
import type { RegisterPayload } from '@/types/auth.types'
import type { AxiosError } from 'axios'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const toast = useToast()
const loading = ref(false)

const schema = v.pipe(
  v.object({
    name: v.pipe(v.string(), v.minLength(1, t('common.validation.nameRequired'))),
    email: v.pipe(
      v.string(),
      v.minLength(1, t('common.validation.emailRequired')),
      v.email(t('common.validation.invalidEmail')),
    ),
    password: v.pipe(
      v.string(),
      v.minLength(1, t('common.validation.passwordRequired')),
      v.minLength(8, t('common.validation.passwordMinLength')),
    ),
    confirmPassword: v.pipe(
      v.string(),
      v.minLength(1, t('common.validation.confirmPasswordRequired')),
    ),
  }),
  v.forward(
    v.partialCheck(
      [['password'], ['confirmPassword']],
      (input) => input.password === input.confirmPassword,
      t('common.validation.passwordsDoNotMatch'),
    ),
    ['confirmPassword'],
  ),
)
type Schema = v.InferOutput<typeof schema>

const model = ref<RegisterPayload & { confirmPassword: string }>({
  name: '',
  email: '',
  password: '',
  confirmPassword: '',
})

async function onSubmit(event: FormSubmitEvent<Schema>) {
  loading.value = true
  try {
    await usersService.create({
      name: event.data.name,
      email: event.data.email,
      password: event.data.password,
    })
    toast.add({
      color: 'success',
      title: t('common.feedback.created'),
      description: t('auth.register.success'),
    })
    router.push('/login')
  } catch (e) {
    const err = e as AxiosError<{ message: string }>
    toast.add({
      color: 'error',
      title: t('common.feedback.error'),
      description: err.response?.data?.message ?? t('auth.register.errors.registerFailed'),
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
            {{ t('auth.register.title') }}
          </h1>
          <p class="text-sm text-muted">
            {{ t('auth.register.description') }}
          </p>
        </div>

        <UForm :schema="schema" :state="model" class="space-y-5" @submit="onSubmit">
          <UFormField :label="t('common.fields.name')" name="name" class="flex flex-col gap-1.5">
            <UInput
              v-model="model.name"
              :placeholder="t('common.placeholders.enterYourName')"
              autocomplete="name"
              class="w-full"
            />
          </UFormField>

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
              autocomplete="new-password"
              class="w-full"
            />
          </UFormField>

          <UFormField
            :label="t('common.fields.confirmPassword')"
            name="confirmPassword"
            class="flex flex-col gap-1.5"
          >
            <UInput
              v-model="model.confirmPassword"
              type="password"
              :placeholder="t('common.placeholders.confirmPassword')"
              autocomplete="new-password"
              class="w-full"
            />
          </UFormField>

          <UButton
            type="submit"
            :label="t('auth.register.submit')"
            :loading="loading"
            trailing-icon="i-lucide-user-plus"
            class="w-full justify-center"
            block
          />

          <p class="text-center text-sm text-muted">
            <RouterLink to="/login" class="font-medium text-primary hover:underline">
              {{ t('navigation.items.alreadyHaveAccount') }}
            </RouterLink>
          </p>
        </UForm>
      </div>
    </UCard>
  </div>
</template>
