<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import Card from 'primevue/card'
import Button from 'primevue/button'
import { useAuthStore } from '@/stores/auth'
import { useToast } from 'primevue/usetoast'
import { recipesService } from '@/services/recipes.service'
import RecipeNutritionCard from '@/components/recipe/RecipeNutritionCard.vue'
import type { RecipeResponse, RecipeTemplate } from '@/types/recipe.types'

const { t, locale } = useI18n({ useScope: 'global' })
const router = useRouter()
const route = useRoute()
const toast = useToast()
const auth = useAuthStore()

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

async function fetchRecipe() {
  fetchLoading.value = true
  try {
    recipe.value = await recipesService.getById(recipeId.value)
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('recipes.messages.loadError'),
      life: 3000,
    })
    router.push('/')
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
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('recipes.messages.pdfError'),
      life: 3000,
    })
  } finally {
    downloadingPdf.value = false
  }
}
onMounted(fetchRecipe)
</script>

<template>
  <div class="p-6 space-y-6">
    <div v-if="fetchLoading" class="flex items-center justify-center py-24">
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400" />
    </div>

    <template v-else-if="recipe">
      <Button
        icon="pi pi-arrow-left"
        :label="t('common.actions.back')"
        severity="secondary"
        text
        size="small"
        @click="router.back()"
      />

      <Card class="border border-surface-200 dark:border-surface-700 shadow-sm overflow-hidden">
        <template #content>
          <div class="flex flex-col sm:flex-row gap-0">
            <div v-if="recipe.imageUrl" class="sm:w-64 sm:shrink-0 h-52 sm:h-auto">
              <img
                :src="recipe.imageUrl"
                :alt="recipe.title"
                class="w-full h-full object-cover rounded-lg"
                loading="lazy"
              />
            </div>

            <div class="flex-1 p-6 flex flex-col justify-between gap-4">
              <div class="space-y-3">
                <div class="space-y-2">
                  <div class="flex items-start justify-between gap-3 flex-wrap">
                    <h1
                      class="text-2xl font-semibold text-surface-900 dark:text-surface-0 leading-snug"
                    >
                      {{ recipe.title }}
                    </h1>
                    <div class="flex items-center gap-2">
                      <Button
                        :label="t('recipes.actions.downloadPdf')"
                        icon="pi pi-file-pdf"
                        icon-pos="right"
                        size="small"
                        severity="secondary"
                        outlined
                        :loading="downloadingPdf"
                        @click="onDownloadPdf"
                      />
                      <Button
                        v-if="isOwnRecipe"
                        :label="t('recipes.actions.edit')"
                        icon="pi pi-pencil"
                        icon-pos="right"
                        size="small"
                        outlined
                        @click="router.push(`/recipes/${recipeId}/edit`)"
                      />
                    </div>
                  </div>

                  <div class="flex items-center gap-2 flex-wrap">
                    <span
                      v-if="recipe.category"
                      class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium bg-surface-100 dark:bg-surface-800 text-surface-600 dark:text-surface-300"
                    >
                      <i class="pi pi-tag text-[10px]"></i>
                      {{ recipe.category.name }}
                    </span>
                    <span
                      v-if="recipe.difficulty"
                      class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium bg-surface-100 dark:bg-surface-800 text-surface-600 dark:text-surface-300"
                    >
                      <i class="pi pi-star text-[10px]"></i>
                      {{ recipe.difficulty.name }}
                    </span>
                    <span
                      v-if="recipe.shared"
                      class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300"
                    >
                      <i class="pi pi-globe text-[10px]"></i>
                      {{ t('recipes.badges.shared') }}
                    </span>
                  </div>
                </div>

                <p
                  v-if="recipe.description"
                  class="text-sm text-surface-500 dark:text-surface-400 leading-relaxed line-clamp-3"
                >
                  {{ recipe.description }}
                </p>
              </div>

              <div
                class="flex items-center gap-6 flex-wrap pt-2 border-t border-surface-100 dark:border-surface-800"
              >
                <div v-if="recipe.prepTime" class="flex items-center gap-1.5">
                  <i class="pi pi-clock text-sm text-surface-400"></i>
                  <div>
                    <p
                      class="text-[10px] uppercase tracking-wide text-surface-400 dark:text-surface-500 font-medium"
                    >
                      {{ t('recipes.fields.prepTime.label') }}
                    </p>
                    <p
                      class="text-sm font-semibold text-surface-800 dark:text-surface-200 tabular-nums leading-none"
                    >
                      {{ recipe.prepTime
                      }}<span class="text-xs font-normal text-surface-400 ml-0.5">{{
                        t('recipes.fields.minutes')
                      }}</span>
                    </p>
                  </div>
                </div>

                <div v-if="recipe.cookTime" class="flex items-center gap-1.5">
                  <i class="pi pi-bolt text-sm text-surface-400"></i>
                  <div>
                    <p
                      class="text-[10px] uppercase tracking-wide text-surface-400 dark:text-surface-500 font-medium"
                    >
                      {{ t('recipes.fields.cookTime.label') }}
                    </p>
                    <p
                      class="text-sm font-semibold text-surface-800 dark:text-surface-200 tabular-nums leading-none"
                    >
                      {{ recipe.cookTime
                      }}<span class="text-xs font-normal text-surface-400 ml-0.5">{{
                        t('recipes.fields.minutes')
                      }}</span>
                    </p>
                  </div>
                </div>

                <div v-if="totalTime" class="flex items-center gap-1.5">
                  <i class="pi pi-stopwatch text-sm text-surface-400"></i>
                  <div>
                    <p
                      class="text-[10px] uppercase tracking-wide text-surface-400 dark:text-surface-500 font-medium"
                    >
                      {{ t('recipes.fields.totalTime') }}
                    </p>
                    <p
                      class="text-sm font-semibold text-surface-800 dark:text-surface-200 tabular-nums leading-none"
                    >
                      {{ totalTime
                      }}<span class="text-xs font-normal text-surface-400 ml-0.5">{{
                        t('recipes.fields.minutes')
                      }}</span>
                    </p>
                  </div>
                </div>

                <div class="flex items-center gap-1.5">
                  <i class="pi pi-users text-sm text-surface-400"></i>
                  <div>
                    <p
                      class="text-[10px] uppercase tracking-wide text-surface-400 dark:text-surface-500 font-medium"
                    >
                      {{ t('recipes.fields.servings.label') }}
                    </p>
                    <p
                      class="text-sm font-semibold text-surface-800 dark:text-surface-200 tabular-nums leading-none"
                    >
                      {{ recipe.servings }}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>
      </Card>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 items-start">
        <div class="lg:col-span-2">
          <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
            <template #content>
              <div class="p-2 space-y-5">
                <h2
                  class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
                >
                  {{ t('recipes.sections.steps') }}
                </h2>

                <ol class="space-y-0 divide-y divide-surface-100 dark:divide-surface-800">
                  <li
                    v-for="step in recipe.steps"
                    :key="step.id"
                    class="flex gap-4 items-start py-4 first:pt-0 last:pb-0"
                  >
                    <span
                      class="w-8 h-8 rounded-full bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 text-sm font-semibold flex items-center justify-center shrink-0 mt-0.5"
                    >
                      {{ step.stepNumber }}
                    </span>
                    <div class="flex-1 pt-1 space-y-1.5">
                      <p class="text-surface-800 dark:text-surface-200 leading-relaxed text-sm">
                        {{ step.instructions }}
                      </p>
                      <span
                        v-if="step.duration"
                        class="inline-flex items-center gap-1 text-xs text-surface-400 dark:text-surface-500 bg-surface-100 dark:bg-surface-800 rounded-full px-2 py-0.5"
                      >
                        <i class="pi pi-clock text-[10px]"></i>
                        {{ step.duration }} {{ t('recipes.fields.minutes') }}
                      </span>
                    </div>
                  </li>
                </ol>
              </div>
            </template>
          </Card>
        </div>

        <div class="lg:col-span-1 space-y-4">
          <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
            <template #content>
              <div class="p-2 space-y-4">
                <h2
                  class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
                >
                  {{ t('recipes.sections.ingredients') }}
                </h2>

                <ul class="space-y-0 divide-y divide-surface-100 dark:divide-surface-800">
                  <li
                    v-for="ingredient in recipe.ingredients"
                    :key="ingredient.id"
                    class="flex items-center justify-between py-2.5 gap-3 first:pt-0 last:pb-0"
                  >
                    <div class="flex items-center gap-2 min-w-0">
                      <span class="w-1.5 h-1.5 rounded-full bg-primary-400 shrink-0"></span>
                      <div class="min-w-0">
                        <span
                          class="text-sm font-medium text-surface-900 dark:text-surface-0 truncate block"
                        >
                          {{ ingredient.name }}
                        </span>
                        <span
                          v-if="ingredient.notes"
                          class="text-xs text-surface-400 dark:text-surface-500 truncate block"
                        >
                          {{ ingredient.notes }}
                        </span>
                      </div>
                    </div>
                    <span
                      v-if="ingredient.quantity || ingredient.unit"
                      class="text-xs font-medium text-surface-500 dark:text-surface-400 shrink-0 tabular-nums whitespace-nowrap"
                    >
                      {{ ingredient.quantity ?? '' }} {{ ingredient.unit ?? '' }}
                    </span>
                  </li>
                </ul>
              </div>
            </template>
          </Card>

          <RecipeNutritionCard
            v-if="recipe.nutrition"
            :nutrition="recipe.nutrition"
            :servings="recipe.servings"
          />

          <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
            <template #content>
              <div class="p-2 space-y-2">
                <h2
                  class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
                >
                  {{ t('recipes.sections.metadata') }}
                </h2>
                <dl class="space-y-2 pt-1">
                  <div class="flex items-center justify-between">
                    <dt class="text-xs text-surface-400 dark:text-surface-500">
                      {{ t('common.fields.createdAt') }}
                    </dt>
                    <dd class="text-xs text-surface-600 dark:text-surface-300 tabular-nums">
                      {{ new Date(recipe.createdAt).toLocaleDateString() }}
                    </dd>
                  </div>
                  <div class="flex items-center justify-between">
                    <dt class="text-xs text-surface-400 dark:text-surface-500">
                      {{ t('common.fields.updatedAt') }}
                    </dt>
                    <dd class="text-xs text-surface-600 dark:text-surface-300 tabular-nums">
                      {{ new Date(recipe.updatedAt).toLocaleDateString() }}
                    </dd>
                  </div>
                </dl>
              </div>
            </template>
          </Card>
        </div>
      </div>
    </template>
  </div>
</template>
