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
const showPassword = ref(false)
const showConfirmPassword = ref(false)

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
  <div
    class="relative flex min-h-full items-center justify-center overflow-hidden px-6 py-12 transition-colors duration-200"
  >
    <div class="pointer-events-none absolute inset-0" aria-hidden="true">
      <div class="absolute -left-32 -top-32 size-96 rounded-full bg-primary/10 blur-3xl" />
      <div class="absolute -bottom-40 -right-32 size-96 rounded-full bg-primary/5 blur-3xl" />
    </div>

    <UCard
      class="relative w-full max-w-md shadow-lg ring-1 ring-default"
      :ui="{ body: 'p-6 sm:p-8' }"
    >
      <div class="space-y-8">
        <div class="flex flex-col items-center gap-4 text-center">
          <div
            class="flex size-14 items-center justify-center rounded-2xl bg-primary/10 ring-1 ring-primary/20"
          >
            <UIcon name="i-lucide-chef-hat" class="size-7 text-primary" />
          </div>
          <div class="space-y-1">
            <h1 class="text-2xl font-semibold tracking-tight text-highlighted">
              {{ t('auth.register.title') }}
            </h1>
            <p class="text-sm text-muted">
              {{ t('auth.register.description') }}
            </p>
          </div>
        </div>

        <UForm :schema="schema" :state="model" class="space-y-5" @submit="onSubmit">
          <UFormField :label="t('common.fields.name')" name="name">
            <UInput
              v-model="model.name"
              size="lg"
              icon="i-lucide-user"
              :placeholder="t('common.placeholders.enterYourName')"
              autocomplete="name"
              class="w-full"
            />
          </UFormField>

          <UFormField :label="t('common.fields.email')" name="email">
            <UInput
              v-model="model.email"
              type="email"
              size="lg"
              icon="i-lucide-mail"
              :placeholder="t('common.placeholders.enterYourEmail')"
              autocomplete="email"
              class="w-full"
            />
          </UFormField>

          <UFormField :label="t('common.fields.password')" name="password">
            <UInput
              v-model="model.password"
              :type="showPassword ? 'text' : 'password'"
              size="lg"
              icon="i-lucide-lock"
              :placeholder="t('common.placeholders.enterPassword')"
              autocomplete="new-password"
              class="w-full"
              :ui="{ trailing: 'pe-1' }"
            >
              <template #trailing>
                <UButton
                  color="neutral"
                  variant="link"
                  size="sm"
                  :icon="showPassword ? 'i-lucide-eye-off' : 'i-lucide-eye'"
                  :aria-label="showConfirmPassword ? 'Hide Password' : 'Show Password'"
                  :aria-pressed="showPassword"
                  @click="
                    () => {
                      showPassword = !showPassword
                    }
                  "
                />
              </template>
            </UInput>
          </UFormField>

          <UFormField :label="t('common.fields.confirmPassword')" name="confirmPassword">
            <UInput
              v-model="model.confirmPassword"
              :type="showConfirmPassword ? 'text' : 'password'"
              size="lg"
              icon="i-lucide-lock-keyhole"
              :placeholder="t('common.placeholders.confirmPassword')"
              autocomplete="new-password"
              class="w-full"
              :ui="{ trailing: 'pe-1' }"
            >
              <template #trailing>
                <UButton
                  color="neutral"
                  variant="link"
                  size="sm"
                  :icon="showConfirmPassword ? 'i-lucide-eye-off' : 'i-lucide-eye'"
                  :aria-label="showConfirmPassword ? 'Hide Password' : 'Show Password'"
                  :aria-pressed="showConfirmPassword"
                  @click="
                    () => {
                      showConfirmPassword = !showConfirmPassword
                    }
                  "
                />
              </template>
            </UInput>
          </UFormField>

          <UButton
            type="submit"
            size="lg"
            :label="t('auth.register.submit')"
            :loading="loading"
            trailing-icon="i-lucide-user-plus"
            class="justify-center"
            block
          />
        </UForm>

        <div class="space-y-4">
          <USeparator />
          <UButton
            to="/login"
            :label="t('navigation.items.alreadyHaveAccount')"
            icon="i-lucide-log-in"
            color="neutral"
            variant="outline"
            class="justify-center"
            block
          />
        </div>
      </div>
    </UCard>
  </div>
</template>
