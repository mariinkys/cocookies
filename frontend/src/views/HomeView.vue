<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Select from 'primevue/select'
import ConfirmDialog from 'primevue/confirmdialog'
import Paginator from 'primevue/paginator'
import ProgressSpinner from 'primevue/progressspinner'
import { useToast } from 'primevue/usetoast'
import { useConfirm } from 'primevue/useconfirm'
import { recipesService } from '@/services/recipes.service'
import type { RecipeResponse } from '@/types/recipe.types'

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()
const confirm = useConfirm()
const router = useRouter()

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

async function fetchRecipes() {
  loading.value = true
  try {
    const result = await recipesService.getAll({
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
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('recipes.messages.loadListError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function onPageChange(event: { page: number }) {
  page.value = event.page
  fetchRecipes()
}

let searchTimeout: ReturnType<typeof setTimeout>
function onSearchInput() {
  clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    page.value = 0
    fetchRecipes()
  }, 350)
}

function onSortChange() {
  page.value = 0
  fetchRecipes()
}

function confirmDelete(event: Event, recipe: RecipeResponse) {
  event.stopPropagation()
  confirm.require({
    message: t('recipes.deleteDialog.message', { name: recipe.title }),
    header: t('recipes.deleteDialog.title'),
    icon: 'pi pi-exclamation-triangle',
    rejectProps: {
      label: t('common.actions.cancel'),
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: { label: t('common.actions.delete'), severity: 'danger' },
    accept: async () => {
      try {
        await recipesService.delete(recipe.id)
        toast.add({
          severity: 'success',
          summary: t('common.feedback.deleted'),
          detail: t('recipes.deleteDialog.success', { name: recipe.title }),
          life: 3000,
        })
        fetchRecipes()
      } catch {
        toast.add({
          severity: 'error',
          summary: t('common.feedback.error'),
          detail: t('recipes.deleteDialog.error'),
          life: 3000,
        })
      }
    },
  })
}

onMounted(fetchRecipes)
</script>

<template>
  <div class="pb-20 p-6 space-y-6">
    <ConfirmDialog />

    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
          {{ t('recipes.titles.list') }}
        </h1>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
          {{ t('recipes.list.count', totalRecords) }}
        </p>
      </div>

      <div class="flex items-center gap-2 flex-wrap">
        <InputText
          v-model="search"
          :placeholder="t('common.placeholders.searchRecipes')"
          class="w-full sm:w-56"
          @input="onSearchInput"
        />
        <Select
          v-model="selectedSort"
          :options="sortOptions"
          option-label="label"
          option-value="value"
          class="w-full sm:w-48"
          @change="onSortChange"
        />
        <Button
          :label="t('recipes.actions.createNew')"
          icon="pi pi-plus"
          class="w-full sm:w-max sm:shrink-0"
          @click="router.push('/recipes/new')"
        />
      </div>
    </div>

    <div
      v-if="loading"
      class="flex flex-col items-center justify-center py-24 gap-4 text-surface-400 dark:text-surface-500"
    >
      <ProgressSpinner
        style="width: 50px; height: 50px"
        strokeWidth="8"
        fill="transparent"
        animationDuration=".5s"
        aria-label="Custom ProgressSpinner"
      />
    </div>

    <div
      v-else-if="recipes.length"
      class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5"
    >
      <div
        v-for="recipe in recipes"
        :key="recipe.id"
        class="group relative flex flex-col rounded-2xl overflow-hidden border border-surface-200 dark:border-surface-700 bg-surface-0 dark:bg-surface-900 shadow-sm hover:shadow-md transition-shadow duration-200 cursor-pointer"
        @click="router.push(`/recipes/${recipe.id}/view`)"
      >
        <div class="relative h-44 bg-surface-100 dark:bg-surface-800 overflow-hidden shrink-0">
          <img
            v-if="recipe.imageUrl"
            :src="recipe.imageUrl"
            :alt="recipe.title"
            class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105"
            loading="lazy"
          />
          <div
            v-else
            class="w-full h-full flex items-center justify-center text-surface-300 dark:text-surface-600"
          >
            <i class="pi pi-image text-4xl"></i>
          </div>

          <span
            v-if="recipe.shared"
            class="absolute top-2 left-2 inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium bg-primary-500 text-white shadow-sm"
          >
            <i class="pi pi-globe text-[10px]"></i>
            {{ t('recipes.badges.shared') }}
          </span>

          <div
            class="absolute top-2 right-2 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
            @click.stop
          >
            <Button
              icon="pi pi-pencil"
              size="small"
              rounded
              class="w-8! h-8! bg-surface-0/90 dark:bg-surface-900/90 text-surface-700! dark:text-surface-200! hover:bg-surface-0! border-0 shadow-sm"
              :aria-label="t('recipes.actions.edit')"
              @click="router.push(`/recipes/${recipe.id}/edit`)"
            />
            <Button
              icon="pi pi-trash"
              size="small"
              rounded
              severity="danger"
              class="w-8! h-8! bg-surface-0/90 dark:bg-surface-900/90 shadow-sm"
              :aria-label="t('recipes.actions.delete')"
              @click="confirmDelete($event, recipe)"
            />
          </div>
        </div>

        <div class="flex flex-col flex-1 p-4 gap-3">
          <div>
            <h2
              class="font-semibold text-surface-900 dark:text-surface-0 leading-snug line-clamp-1"
            >
              {{ recipe.title }}
            </h2>
            <p
              v-if="recipe.description"
              class="text-sm text-surface-500 dark:text-surface-400 mt-1 line-clamp-2"
            >
              {{ recipe.description }}
            </p>
          </div>

          <div
            class="mt-auto flex items-center gap-3 text-xs text-surface-400 dark:text-surface-500 flex-wrap"
          >
            <span v-if="recipe.prepTime ?? recipe.cookTime" class="flex items-center gap-1">
              <i class="pi pi-clock text-[11px]"></i>
              {{ (recipe.prepTime ?? 0) + (recipe.cookTime ?? 0) }}
              {{ t('recipes.fields.minutes') }}
            </span>
            <span class="flex items-center gap-1">
              <i class="pi pi-users text-[11px]"></i>
              {{ t('recipes.fields.servingsList', recipe.servings) }}
            </span>
            <span v-if="recipe.difficulty" class="flex items-center gap-1">
              <i class="pi pi-star text-[11px]"></i>
              {{ recipe.difficulty.name }}
            </span>
            <span
              v-if="recipe.category"
              class="ml-auto inline-block rounded-full px-2 py-0.5 bg-surface-100 dark:bg-surface-800 text-surface-600 dark:text-surface-300 font-medium"
            >
              {{ recipe.category.name }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <div
      v-else
      class="flex flex-col items-center justify-center py-24 gap-4 text-surface-400 dark:text-surface-500"
    >
      <i class="pi pi-book text-5xl"></i>
      <p class="text-sm">{{ t('recipes.list.empty') }}</p>
      <Button
        :label="t('recipes.actions.createFirst')"
        icon="pi pi-plus"
        outlined
        @click="router.push('/recipes/new')"
      />
    </div>

    <div
      class="fixed bottom-0 left-0 right-0 bg-surface-0 dark:bg-surface-900 border-t border-surface-200 dark:border-surface-700 px-6 py-0 z-10"
    >
      <Paginator
        :rows="pageSize"
        :total-records="totalRecords"
        :first="page * pageSize"
        template="FirstPageLink PrevPageLink CurrentPageReport NextPageLink LastPageLink"
        current-page-report-template="{first}–{last} of {totalRecords}"
        @page="onPageChange"
      />
    </div>
  </div>
</template>
