<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '@/stores/auth'
import { recipesService } from '@/services/recipes.service'
import RecipeNutritionCard from '@/components/recipe/RecipeNutritionCard.vue'
import type { RecipeResponse, RecipeTemplate } from '@/types/recipe.types'

const { t, locale } = useI18n({ useScope: 'global' })
const router = useRouter()
const route = useRoute()
const toast = useToast()
const auth = useAuthStore()

function goTo(path: string) {
  router.push(path)
}

const recipeId = computed(() => route.params.id as string)
const recipe = ref<RecipeResponse | null>(null)
const fetchLoading = ref(true)
const downloadingPdf = ref(false)

const isOwnRecipe = computed(() => auth.user?.id === recipe.value?.userId)

const totalTime = computed(() => {
  if (!recipe.value) return null
  const total = (recipe.value.prepTime ?? 0) + (recipe.value.cookTime ?? 0)
  return total > 0 ? total : null
})

const recipeTemplate = computed<RecipeTemplate>(() =>
  locale.value.startsWith('es') ? 'default-es' : 'default-en',
)

function formatDate(value: string) {
  return new Date(value).toLocaleDateString(locale.value)
}

async function fetchRecipe() {
  fetchLoading.value = true
  try {
    recipe.value = await recipesService.getById(recipeId.value)
  } catch {
    toast.add({
      color: 'error',
      title: t('common.feedback.error'),
      description: t('recipes.messages.loadError'),
    })
    goTo('/')
  } finally {
    fetchLoading.value = false
  }
}

async function onDownloadPdf() {
  if (!recipe.value) return
  downloadingPdf.value = true
  try {
    await recipesService.downloadPdf(recipe.value.id, recipeTemplate.value)
  } catch {
    toast.add({
      color: 'error',
      title: t('common.feedback.error'),
      description: t('recipes.messages.pdfError'),
    })
  } finally {
    downloadingPdf.value = false
  }
}

onMounted(fetchRecipe)
</script>

<template>
  <div class="mx-auto max-w-6xl space-y-6 p-6">
    <div v-if="fetchLoading" class="space-y-6">
      <USkeleton class="h-8 w-28" />

      <div class="overflow-hidden rounded-xl ring ring-default">
        <div class="flex flex-col sm:flex-row">
          <USkeleton class="h-52 shrink-0 rounded-none sm:h-64 sm:w-64" />
          <div class="flex-1 space-y-4 p-6">
            <USkeleton class="h-7 w-2/3" />
            <div class="flex gap-2">
              <USkeleton class="h-5 w-20 rounded-full" />
              <USkeleton class="h-5 w-20 rounded-full" />
            </div>
            <USkeleton class="h-4 w-full" />
            <USkeleton class="h-4 w-5/6" />
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <div class="space-y-3 lg:col-span-2">
          <USkeleton v-for="n in 4" :key="n" class="h-16 w-full" />
        </div>
        <div class="space-y-3">
          <USkeleton v-for="n in 5" :key="n" class="h-10 w-full" />
        </div>
      </div>
    </div>

    <template v-else-if="recipe">
      <UButton
        icon="i-lucide-arrow-left"
        :label="t('common.actions.back')"
        color="neutral"
        variant="ghost"
        size="sm"
        @click="router.back()"
      />

      <UCard :ui="{ body: 'p-0' }" class="overflow-hidden">
        <div class="flex flex-col sm:flex-row">
          <div v-if="recipe.imageUrl" class="h-52 shrink-0 sm:h-auto sm:w-64">
            <img
              :src="recipe.imageUrl"
              :alt="recipe.title"
              loading="lazy"
              class="h-full w-full object-cover"
            />
          </div>

          <div class="flex flex-1 flex-col justify-between gap-4 p-6">
            <div class="space-y-3">
              <div class="space-y-2">
                <div class="flex flex-wrap items-start justify-between gap-3">
                  <h1 class="text-2xl font-semibold leading-snug text-highlighted">
                    {{ recipe.title }}
                  </h1>
                  <div class="flex items-center gap-2">
                    <UButton
                      :label="t('recipes.actions.downloadPdf')"
                      trailing-icon="i-lucide-download"
                      color="neutral"
                      variant="outline"
                      size="sm"
                      :loading="downloadingPdf"
                      @click="onDownloadPdf"
                    />
                    <UButton
                      v-if="isOwnRecipe"
                      :label="t('recipes.actions.edit')"
                      trailing-icon="i-lucide-pencil"
                      color="primary"
                      variant="outline"
                      size="sm"
                      @click="goTo(`/recipes/${recipeId}/edit`)"
                    />
                  </div>
                </div>

                <div class="flex flex-wrap items-center gap-2">
                  <UBadge v-if="recipe.category" color="neutral" variant="subtle" size="sm">
                    <UIcon name="i-lucide-tag" class="size-3" />
                    {{ recipe.category.name }}
                  </UBadge>
                  <UBadge v-if="recipe.difficulty" color="neutral" variant="subtle" size="sm">
                    <UIcon name="i-lucide-star" class="size-3" />
                    {{ recipe.difficulty.name }}
                  </UBadge>
                  <UBadge v-if="recipe.shared" color="primary" variant="subtle" size="sm">
                    <UIcon name="i-lucide-globe" class="size-3" />
                    {{ t('recipes.badges.shared') }}
                  </UBadge>
                </div>
              </div>

              <p v-if="recipe.description" class="line-clamp-3 text-sm leading-relaxed text-muted">
                {{ recipe.description }}
              </p>
            </div>

            <div class="flex flex-wrap items-center gap-6 border-t border-default pt-3">
              <div v-if="recipe.prepTime" class="flex items-center gap-1.5">
                <UIcon name="i-lucide-clock" class="size-4 text-dimmed" />
                <div>
                  <p class="text-[10px] font-medium uppercase tracking-wide text-dimmed">
                    {{ t('recipes.fields.prepTime.label') }}
                  </p>
                  <p class="text-sm font-semibold leading-none tabular-nums text-highlighted">
                    {{ recipe.prepTime
                    }}<span class="ml-0.5 text-xs font-normal text-dimmed">{{
                      t('recipes.fields.minutes')
                    }}</span>
                  </p>
                </div>
              </div>

              <div v-if="recipe.cookTime" class="flex items-center gap-1.5">
                <UIcon name="i-lucide-flame" class="size-4 text-dimmed" />
                <div>
                  <p class="text-[10px] font-medium uppercase tracking-wide text-dimmed">
                    {{ t('recipes.fields.cookTime.label') }}
                  </p>
                  <p class="text-sm font-semibold leading-none tabular-nums text-highlighted">
                    {{ recipe.cookTime
                    }}<span class="ml-0.5 text-xs font-normal text-dimmed">{{
                      t('recipes.fields.minutes')
                    }}</span>
                  </p>
                </div>
              </div>

              <div v-if="totalTime" class="flex items-center gap-1.5">
                <UIcon name="i-lucide-timer" class="size-4 text-dimmed" />
                <div>
                  <p class="text-[10px] font-medium uppercase tracking-wide text-dimmed">
                    {{ t('recipes.fields.totalTime') }}
                  </p>
                  <p class="text-sm font-semibold leading-none tabular-nums text-highlighted">
                    {{ totalTime
                    }}<span class="ml-0.5 text-xs font-normal text-dimmed">{{
                      t('recipes.fields.minutes')
                    }}</span>
                  </p>
                </div>
              </div>

              <div class="flex items-center gap-1.5">
                <UIcon name="i-lucide-users" class="size-4 text-dimmed" />
                <div>
                  <p class="text-[10px] font-medium uppercase tracking-wide text-dimmed">
                    {{ t('recipes.fields.servings.label') }}
                  </p>
                  <p class="text-sm font-semibold leading-none tabular-nums text-highlighted">
                    {{ recipe.servings }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </UCard>

      <div class="grid grid-cols-1 items-start gap-6 lg:grid-cols-3">
        <div class="lg:col-span-2">
          <UCard>
            <template #header>
              <h2 class="text-sm font-semibold uppercase tracking-wide text-muted">
                {{ t('recipes.sections.steps') }}
              </h2>
            </template>

            <ol class="divide-y divide-default">
              <li
                v-for="step in recipe.steps"
                :key="step.id"
                class="flex items-start gap-4 py-4 first:pt-0 last:pb-0"
              >
                <span
                  class="mt-0.5 flex size-8 shrink-0 items-center justify-center rounded-full bg-primary/10 text-sm font-semibold text-primary"
                >
                  {{ step.stepNumber }}
                </span>
                <div class="flex-1 space-y-1.5 pt-1">
                  <p class="text-sm leading-relaxed text-toned">
                    {{ step.instructions }}
                  </p>
                  <span
                    v-if="step.duration"
                    class="inline-flex items-center gap-1 rounded-full bg-elevated px-2 py-0.5 text-xs text-dimmed"
                  >
                    <UIcon name="i-lucide-clock" class="size-3" />
                    {{ step.duration }} {{ t('recipes.fields.minutes') }}
                  </span>
                </div>
              </li>
            </ol>
          </UCard>
        </div>

        <div class="space-y-4 lg:col-span-1">
          <UCard>
            <template #header>
              <h2 class="text-sm font-semibold uppercase tracking-wide text-muted">
                {{ t('recipes.sections.ingredients') }}
              </h2>
            </template>

            <ul class="divide-y divide-default">
              <li
                v-for="ingredient in recipe.ingredients"
                :key="ingredient.id"
                class="flex items-center justify-between gap-3 py-2.5 first:pt-0 last:pb-0"
              >
                <div class="flex min-w-0 items-center gap-2">
                  <span class="size-1.5 shrink-0 rounded-full bg-primary"></span>
                  <div class="min-w-0">
                    <span class="block truncate text-sm font-medium text-highlighted">
                      {{ ingredient.name }}
                    </span>
                    <span v-if="ingredient.notes" class="block truncate text-xs text-dimmed">
                      {{ ingredient.notes }}
                    </span>
                  </div>
                </div>
                <span
                  v-if="ingredient.quantity || ingredient.unit"
                  class="shrink-0 whitespace-nowrap text-xs font-medium tabular-nums text-muted"
                >
                  {{ ingredient.quantity ?? '' }} {{ ingredient.unit ?? '' }}
                </span>
              </li>
            </ul>
          </UCard>

          <RecipeNutritionCard
            v-if="recipe.nutrition"
            :nutrition="recipe.nutrition"
            :servings="recipe.servings"
          />

          <UCard>
            <template #header>
              <h2 class="text-sm font-semibold uppercase tracking-wide text-muted">
                {{ t('recipes.sections.metadata') }}
              </h2>
            </template>

            <dl class="space-y-2">
              <div class="flex items-center justify-between">
                <dt class="text-xs text-dimmed">{{ t('common.fields.createdAt') }}</dt>
                <dd class="text-xs tabular-nums text-toned">{{ formatDate(recipe.createdAt) }}</dd>
              </div>
              <div class="flex items-center justify-between">
                <dt class="text-xs text-dimmed">{{ t('common.fields.updatedAt') }}</dt>
                <dd class="text-xs tabular-nums text-toned">{{ formatDate(recipe.updatedAt) }}</dd>
              </div>
            </dl>
          </UCard>
        </div>
      </div>
    </template>
  </div>
</template>
