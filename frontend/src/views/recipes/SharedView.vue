<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { recipesService } from '@/services/recipes.service'
import { useAuthStore } from '@/stores/auth'
import type { RecipeResponse } from '@/types/recipe.types'

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()
const router = useRouter()
const auth = useAuthStore()

const recipes = ref<RecipeResponse[]>([])
const loading = ref(false)
const search = ref('')
const totalRecords = ref(0)
const page = ref(0)
const pageSize = ref(12)

const sortOptions = [
  { label: t('common.sort.titleAsc'), value: 'title:asc' },
  { label: t('common.sort.titleDesc'), value: 'title:desc' },
  { label: t('common.sort.newestFirst'), value: 'createdAt:desc' },
  { label: t('common.sort.oldestFirst'), value: 'createdAt:asc' },
]
const selectedSort = ref<string>(sortOptions[0]!.value)

const sortBy = computed(() => selectedSort.value.split(':')[0])
const sortDir = computed(() => selectedSort.value.split(':')[1] as 'asc' | 'desc')

const rangeStart = computed(() => (totalRecords.value === 0 ? 0 : page.value * pageSize.value + 1))
const rangeEnd = computed(() => Math.min((page.value + 1) * pageSize.value, totalRecords.value))

// UPagination is 1-indexed, the API is 0-indexed
const uiPage = computed({
  get: () => page.value + 1,
  set: (value: number) => {
    page.value = value - 1
    fetchRecipes()
  },
})

async function fetchRecipes() {
  loading.value = true
  try {
    const result = await recipesService.getAllShared({
      page: page.value,
      size: pageSize.value,
      sortBy: sortBy.value,
      sortDir: sortDir.value,
      search: search.value.trim(),
    })
    recipes.value = result.content
    totalRecords.value = result.totalElements
  } catch {
    toast.add({
      color: 'error',
      title: t('common.feedback.error'),
      description: t('recipes.messages.loadListError'),
    })
  } finally {
    loading.value = false
  }
}

let searchTimeout: ReturnType<typeof setTimeout>
watch(search, () => {
  clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    page.value = 0
    fetchRecipes()
  }, 350)
})

watch(selectedSort, () => {
  page.value = 0
  fetchRecipes()
})

function isOwnRecipe(recipe: RecipeResponse): boolean {
  return recipe.userId === auth.user?.id
}

const deleteTarget = ref<RecipeResponse | null>(null)
const isDeleteModalOpen = ref(false)
const deleting = ref(false)

function askDelete(event: Event, recipe: RecipeResponse) {
  event.stopPropagation()
  deleteTarget.value = recipe
  isDeleteModalOpen.value = true
}

function closeDeleteModal() {
  isDeleteModalOpen.value = false
}

async function confirmDeleteRecipe() {
  if (!deleteTarget.value) return
  const recipe = deleteTarget.value
  deleting.value = true
  try {
    await recipesService.delete(recipe.id)
    toast.add({
      color: 'success',
      title: t('common.feedback.deleted'),
      description: t('recipes.deleteDialog.success', { name: recipe.title }),
    })
    isDeleteModalOpen.value = false
    fetchRecipes()
  } catch {
    toast.add({
      color: 'error',
      title: t('common.feedback.error'),
      description: t('recipes.deleteDialog.error'),
    })
  } finally {
    deleting.value = false
  }
}

onMounted(fetchRecipes)
</script>

<template>
  <div class="pb-24">
    <div
      class="flex flex-col gap-4 border-b border-default px-6 py-6 lg:flex-row lg:items-end lg:justify-between"
    >
      <div>
        <h1 class="text-2xl font-semibold tracking-tight text-highlighted">
          {{ t('recipes.titles.shared') }}
        </h1>
        <p class="mt-1 text-sm text-muted">
          {{ t('recipes.list.count', totalRecords) }}
        </p>
      </div>

      <div class="flex flex-col gap-2 sm:flex-row sm:items-center">
        <UInput
          v-model="search"
          icon="i-lucide-search"
          :placeholder="t('common.placeholders.searchRecipes')"
          class="w-full sm:w-64"
        />
        <USelect v-model="selectedSort" :items="sortOptions" class="w-full sm:w-48" />
      </div>
    </div>

    <div class="px-6 py-6">
      <div
        v-if="loading"
        class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
      >
        <div
          v-for="n in pageSize"
          :key="n"
          class="flex flex-col overflow-hidden rounded-xl border border-default"
        >
          <USkeleton class="h-44 w-full rounded-none" />
          <div class="flex flex-col gap-3 p-4">
            <USkeleton class="h-4 w-3/4" />
            <USkeleton class="h-3 w-full" />
            <USkeleton class="h-3 w-2/3" />
            <div class="mt-2 flex gap-3">
              <USkeleton class="h-3 w-12" />
              <USkeleton class="h-3 w-12" />
            </div>
          </div>
        </div>
      </div>

      <div
        v-else-if="recipes.length"
        class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
      >
        <article
          v-for="recipe in recipes"
          :key="recipe.id"
          class="group relative flex cursor-pointer flex-col overflow-hidden rounded-xl border border-default bg-default transition-all duration-200 hover:-translate-y-0.5 hover:border-accented hover:shadow-lg"
          @click="router.push(`/recipes/${recipe.id}/view`)"
        >
          <div class="relative h-44 shrink-0 overflow-hidden bg-elevated">
            <img
              v-if="recipe.imageUrl"
              :src="recipe.imageUrl"
              :alt="recipe.title"
              loading="lazy"
              class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
            />
            <div v-else class="flex h-full w-full items-center justify-center text-dimmed">
              <UIcon name="i-lucide-image" class="size-10" />
            </div>

            <UBadge
              v-if="recipe.shared"
              color="primary"
              variant="solid"
              size="sm"
              class="absolute left-2 top-2 gap-1"
            >
              <UIcon name="i-lucide-globe" class="size-3" />
              {{ t('recipes.badges.shared') }}
            </UBadge>

            <div
              v-if="isOwnRecipe(recipe)"
              class="absolute right-2 top-2 flex gap-1 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
              @click.stop
            >
              <UButton
                icon="i-lucide-pencil"
                color="neutral"
                variant="solid"
                size="sm"
                :aria-label="t('recipes.actions.edit')"
                @click="
                  () => {
                    router.push(`/recipes/${recipe.id}/edit`)
                  }
                "
              />
              <UButton
                icon="i-lucide-trash-2"
                color="error"
                variant="solid"
                size="sm"
                :aria-label="t('recipes.actions.delete')"
                @click="askDelete($event, recipe)"
              />
            </div>
          </div>

          <div class="flex flex-1 flex-col gap-3 p-4">
            <div>
              <h2 class="line-clamp-1 font-semibold leading-snug text-highlighted">
                {{ recipe.title }}
              </h2>
              <p v-if="recipe.description" class="mt-1 line-clamp-2 text-sm text-muted">
                {{ recipe.description }}
              </p>
            </div>

            <div class="mt-auto flex flex-wrap items-center gap-3 text-xs text-dimmed">
              <span v-if="recipe.prepTime ?? recipe.cookTime" class="flex items-center gap-1">
                <UIcon name="i-lucide-clock" class="size-3.5" />
                {{ (recipe.prepTime ?? 0) + (recipe.cookTime ?? 0) }}
                {{ t('recipes.fields.minutes') }}
              </span>
              <span class="flex items-center gap-1">
                <UIcon name="i-lucide-users" class="size-3.5" />
                {{ t('recipes.fields.servingsList', recipe.servings) }}
              </span>
              <span v-if="recipe.difficulty" class="flex items-center gap-1">
                <UIcon name="i-lucide-star" class="size-3.5" />
                {{ recipe.difficulty.name }}
              </span>
              <UBadge
                v-if="recipe.category"
                color="neutral"
                variant="subtle"
                size="sm"
                class="ml-auto"
              >
                {{ recipe.category.name }}
              </UBadge>
            </div>
          </div>
        </article>
      </div>

      <div
        v-else
        class="flex flex-col items-center justify-center gap-4 py-24 text-center text-muted"
      >
        <div class="flex size-16 items-center justify-center rounded-full bg-elevated">
          <UIcon name="i-lucide-book-open" class="size-7 text-dimmed" />
        </div>
        <p class="text-sm">{{ t('recipes.list.empty') }}</p>
        <UButton
          :label="t('recipes.actions.createFirst')"
          icon="i-lucide-plus"
          variant="outline"
          @click="
            () => {
              router.push('/recipes/new')
            }
          "
        />
      </div>
    </div>

    <div class="fixed inset-x-0 bottom-0 z-10 border-t border-default bg-default px-6 py-3">
      <div class="flex flex-col-reverse items-center justify-between gap-2 sm:flex-row">
        <p class="text-sm text-muted">{{ rangeStart }}–{{ rangeEnd }} of {{ totalRecords }}</p>
        <UPagination
          v-model:page="uiPage"
          :total="totalRecords"
          :items-per-page="pageSize"
          :sibling-count="1"
          size="sm"
          color="primary"
        />
      </div>
    </div>

    <UModal
      v-model:open="isDeleteModalOpen"
      :title="t('recipes.deleteDialog.title')"
      :description="
        deleteTarget ? t('recipes.deleteDialog.message', { name: deleteTarget.title }) : ''
      "
      :dismissible="!deleting"
      :ui="{ footer: 'justify-end' }"
    >
      <template #footer>
        <UButton
          :label="t('common.actions.cancel')"
          color="neutral"
          variant="outline"
          :disabled="deleting"
          @click="closeDeleteModal"
        />
        <UButton
          :label="t('common.actions.delete')"
          color="error"
          :loading="deleting"
          @click="confirmDeleteRecipe"
        />
      </template>
    </UModal>
  </div>
</template>
