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
const showPassword = ref(false)

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
  <div class="relative flex h-full items-center justify-center overflow-hidden px-6 py-12">
    <div class="pointer-events-none absolute inset-0 -z-10" aria-hidden="true">
      <div
        class="absolute left-1/2 top-0 size-144 -translate-x-1/2 -translate-y-1/2 rounded-full bg-primary/5 blur-3xl"
      />
    </div>

    <div class="w-full max-w-md">
      <div class="overflow-hidden rounded-xl border border-default bg-default shadow-sm">
        <div
          class="relative flex h-28 items-center justify-center overflow-hidden border-b border-default bg-elevated"
        >
          <div
            class="absolute inset-0 flex items-center justify-around text-dimmed/40"
            aria-hidden="true"
          >
            <UIcon name="i-lucide-carrot" class="size-6 -rotate-12" />
            <UIcon name="i-lucide-cooking-pot" class="size-7 rotate-6" />
            <UIcon name="i-lucide-utensils" class="size-6 -rotate-6" />
            <UIcon name="i-lucide-croissant" class="size-7 rotate-12" />
          </div>

          <div
            class="relative flex size-14 items-center justify-center rounded-full bg-default shadow-sm ring-1 ring-default"
          >
            <UIcon name="i-lucide-chef-hat" class="size-7 text-primary" />
          </div>
        </div>

        <div class="space-y-6 p-6 sm:p-8">
          <div class="space-y-1 text-center">
            <h1 class="text-xl font-semibold tracking-tight text-highlighted">
              {{ t('auth.login.title') }}
            </h1>
            <p class="text-sm text-muted">
              {{ t('auth.login.description') }}
            </p>
          </div>

          <UForm :schema="schema" :state="model" class="space-y-5" @submit="onSubmit">
            <UFormField :label="t('common.fields.email')" name="email">
              <UInput
                v-model="model.email"
                type="email"
                icon="i-lucide-mail"
                :placeholder="t('common.placeholders.enterYourEmail')"
                autocomplete="email"
                size="lg"
                class="w-full"
              />
            </UFormField>

            <UFormField :label="t('common.fields.password')" name="password">
              <UInput
                v-model="model.password"
                :type="showPassword ? 'text' : 'password'"
                icon="i-lucide-lock"
                :placeholder="t('common.placeholders.enterPassword')"
                autocomplete="current-password"
                size="lg"
                class="w-full"
                :ui="{ trailing: 'pe-1' }"
              >
                <template #trailing>
                  <UButton
                    color="neutral"
                    variant="link"
                    size="sm"
                    :icon="showPassword ? 'i-lucide-eye-off' : 'i-lucide-eye'"
                    :aria-label="showPassword ? 'Hide Password' : 'Show Password'"
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

            <UButton
              type="submit"
              :label="t('auth.login.submit')"
              :loading="loading"
              trailing-icon="i-lucide-log-in"
              size="lg"
              block
            />
          </UForm>

          <USeparator :label="t('auth.login.noAccount')" :ui="{ label: 'text-dimmed text-xs' }" />

          <UButton
            :label="t('navigation.items.newAccount')"
            icon="i-lucide-user-plus"
            color="neutral"
            variant="outline"
            block
            @click="
              () => {
                router.push('/register')
              }
            "
          />
        </div>
      </div>
    </div>
  </div>
</template>
