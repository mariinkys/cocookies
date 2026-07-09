<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import * as v from 'valibot'
import type { FormSubmitEvent } from '@nuxt/ui'
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
const passwordError = ref('')

const model = ref({ name: '', email: '' })

const profileSchema = v.object({
  name: v.pipe(
    v.string(),
    v.trim(),
    v.minLength(1, t('users.fields.name.required')),
    v.maxLength(100, t('users.fields.name.max')),
  ),
  email: v.pipe(
    v.string(),
    v.trim(),
    v.minLength(1, t('users.fields.email.required')),
    v.email(t('users.fields.email.invalid')),
  ),
})
type ProfileSchema = v.InferOutput<typeof profileSchema>

async function onSubmit(event: FormSubmitEvent<ProfileSchema>) {
  loading.value = true
  try {
    const payload: UserUpdatePayload = { name: event.data.name, email: event.data.email }
    await usersService.update(userId, payload)
    toast.add({
      color: 'success',
      title: t('common.feedback.saved'),
      description: t('users.messages.updated'),
    })
    router.push('/')
  } catch {
    toast.add({
      color: 'error',
      title: t('common.feedback.error'),
      description: t('users.messages.updateError'),
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

const passwordSchema = v.pipe(
  v.object({
    currentPassword: v.pipe(
      v.string(),
      v.minLength(1, t('users.passwordCard.fields.currentPassword.required')),
    ),
    newPassword: v.pipe(v.string(), v.minLength(8, t('users.passwordCard.fields.newPassword.min'))),
    confirmPassword: v.pipe(
      v.string(),
      v.minLength(1, t('users.passwordCard.fields.confirmPassword.required')),
    ),
  }),
  v.forward(
    v.partialCheck(
      [['newPassword'], ['confirmPassword']],
      (input) => input.newPassword === input.confirmPassword,
      t('users.passwordCard.fields.confirmPassword.mismatch'),
    ),
    ['confirmPassword'],
  ),
)
type PasswordSchema = v.InferOutput<typeof passwordSchema>

async function onPasswordSubmit(event: FormSubmitEvent<PasswordSchema>) {
  passwordLoading.value = true
  passwordError.value = ''
  try {
    const payload: UpdatePasswordPayload = {
      currentPassword: event.data.currentPassword,
      newPassword: event.data.newPassword,
    }
    await usersService.updatePassword(userId, payload)
    toast.add({
      color: 'success',
      title: t('users.passwordCard.successTitle'),
      description: t('users.passwordCard.successDetail'),
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
      color: 'error',
      title: t('common.feedback.error'),
      description: t('users.messages.loadError'),
    })
    router.push('/')
  } finally {
    fetchLoading.value = false
  }
})
</script>

<template>
  <div class="mx-auto max-w-2xl space-y-6 p-6">
    <div class="flex items-center gap-3">
      <UButton
        icon="i-lucide-arrow-left"
        color="neutral"
        variant="ghost"
        :aria-label="t('common.actions.back')"
        @click="
          () => {
            router.push('/')
          }
        "
      />
      <div>
        <h1 class="text-xl font-semibold text-highlighted">{{ t('users.titles.edit') }}</h1>
        <p class="mt-0.5 text-sm text-muted">{{ t('users.descriptions.edit') }}</p>
      </div>
    </div>

    <div v-if="fetchLoading" class="flex items-center justify-center py-24">
      <UIcon name="i-lucide-loader-2" class="size-6 animate-spin text-dimmed" />
    </div>

    <template v-else>
      <UCard>
        <UForm :schema="profileSchema" :state="model" class="space-y-6" @submit="onSubmit">
          <UFormField :label="t('users.fields.name.label')" name="name" required>
            <UInput
              :model-value="model.name"
              @update:model-value="model.name = ($event as string) ?? ''"
              :placeholder="t('users.fields.name.placeholder')"
              :maxlength="100"
              class="w-full"
            />
          </UFormField>

          <UFormField label="Email" name="email" required>
            <UInput
              :model-value="model.email"
              @update:model-value="model.email = ($event as string) ?? ''"
              type="email"
              :placeholder="t('users.fields.email.placeholder')"
              class="w-full"
            />
          </UFormField>

          <div class="flex items-center justify-end gap-3 pt-2">
            <UButton
              :label="t('common.actions.cancel')"
              color="neutral"
              variant="outline"
              @click="
                () => {
                  router.push('/')
                }
              "
            />
            <UButton
              type="submit"
              :label="t('common.actions.saveChanges')"
              trailing-icon="i-lucide-check"
              :loading="loading"
            />
          </div>
        </UForm>
      </UCard>

      <UCard>
        <UForm
          :schema="passwordSchema"
          :state="passwordModel"
          class="space-y-4"
          @submit="onPasswordSubmit"
        >
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-lock" class="size-4 text-primary" />
            <h2 class="text-xs font-semibold uppercase tracking-wide text-muted">
              {{ t('users.passwordCard.title') }}
            </h2>
          </div>

          <UFormField
            :label="t('users.passwordCard.fields.currentPassword.label')"
            name="currentPassword"
            required
          >
            <UInput
              :model-value="passwordModel.currentPassword"
              @update:model-value="passwordModel.currentPassword = ($event as string) ?? ''"
              type="password"
              :placeholder="t('users.passwordCard.fields.currentPassword.placeholder')"
              class="w-full"
            />
          </UFormField>

          <UFormField
            :label="t('users.passwordCard.fields.newPassword.label')"
            name="newPassword"
            required
          >
            <UInput
              :model-value="passwordModel.newPassword"
              @update:model-value="passwordModel.newPassword = ($event as string) ?? ''"
              type="password"
              :placeholder="t('users.passwordCard.fields.newPassword.placeholder')"
              class="w-full"
            />
          </UFormField>

          <UFormField
            :label="t('users.passwordCard.fields.confirmPassword.label')"
            name="confirmPassword"
            required
          >
            <UInput
              :model-value="passwordModel.confirmPassword"
              @update:model-value="passwordModel.confirmPassword = ($event as string) ?? ''"
              type="password"
              :placeholder="t('users.passwordCard.fields.confirmPassword.placeholder')"
              class="w-full"
            />
          </UFormField>

          <p v-if="passwordError" class="text-sm text-error">{{ passwordError }}</p>

          <div class="flex justify-end pt-1">
            <UButton
              type="submit"
              :label="t('users.passwordCard.submit')"
              trailing-icon="i-lucide-lock"
              :loading="passwordLoading"
            />
          </div>
        </UForm>
      </UCard>
    </template>
  </div>
</template>
