<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'

import { useRecipeForm } from '@/composables/recipe/useRecipeForm'
import { useRecipeSteps } from '@/composables/recipe/useRecipeSteps'
import { useRecipeIngredients } from '@/composables/recipe/useRecipeIngredients'
import { useRecipeNutrition } from '@/composables/recipe/useRecipeNutrition'
import { useLlmDialog } from '@/composables/useLlmDialog'

import StepDialog from '@/components/recipe/upsert/StepDialog.vue'
import IngredientDialog from '@/components/recipe/upsert/IngredientDialog.vue'
import StepList from '@/components/recipe/upsert/StepList.vue'
import IngredientList from '@/components/recipe/upsert/IngredientList.vue'
import RecipeBasicInfo from '@/components/recipe/upsert/RecipeBasicInfo.vue'
import RecipeOrganisation from '@/components/recipe/upsert/RecipeOrganisation.vue'
import RecipeTiming from '@/components/recipe/upsert/RecipeTiming.vue'
import RecipeVisibility from '@/components/recipe/upsert/RecipeVisibility.vue'
import RecipeNutrition from '@/components/recipe/upsert/RecipeNutrition.vue'
import AskLlmDialog from '@/components/recipe/upsert/AskLlmDialog.vue'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const route = useRoute()

const recipeId = computed(() => route.params.id as string | undefined)

const {
  model,
  schema,
  isEdit,
  isOwnRecipe,
  loading,
  fetchLoading,
  deleteLoading,
  isDeleteModalOpen,
  fetchRecipe,
  onSubmit,
  openDeleteModal,
  closeDeleteModal,
  confirmDeleteRecipe,
} = useRecipeForm(recipeId)

const steps = useRecipeSteps(model)
const ingredients = useRecipeIngredients(model)
const nutrition = useRecipeNutrition(model)
const { llmDialogVisible, applyLlmImport } = useLlmDialog(model)

const readOnly = computed(() => isEdit.value && !isOwnRecipe.value)

onMounted(fetchRecipe)
</script>

<template>
  <div class="pb-6">
    <div v-if="fetchLoading" class="flex items-center justify-center py-24">
      <UIcon name="i-lucide-loader-2" class="size-6 animate-spin text-dimmed" />
    </div>

    <UForm v-else :schema="schema" :state="model" class="space-y-4 sm:space-y-6" @submit="onSubmit">
      <div
        class="flex flex-col gap-4 border-b border-default px-6 py-6 sm:flex-row sm:items-center sm:justify-between"
      >
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
            <h1 class="text-2xl font-semibold tracking-tight text-highlighted">
              {{ isEdit ? t('recipes.titles.edit') : t('recipes.titles.create') }}
            </h1>
            <p class="mt-1 text-sm text-muted">
              {{ isEdit ? t('recipes.descriptions.edit') : t('recipes.descriptions.create') }}
            </p>
          </div>
        </div>

        <div v-if="!readOnly" class="flex flex-wrap items-center gap-2">
          <UButton
            v-if="!isEdit"
            :label="t('recipes.import.trigger')"
            icon="i-lucide-sparkles"
            color="neutral"
            variant="outline"
            @click="
              () => {
                llmDialogVisible = true
              }
            "
          />
          <UButton
            v-if="isEdit"
            icon="i-lucide-trash-2"
            color="error"
            variant="outline"
            :loading="deleteLoading"
            :aria-label="t('recipes.actions.delete')"
            @click="openDeleteModal"
          />
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
            :label="isEdit ? t('common.actions.saveChanges') : t('recipes.actions.create')"
            :trailing-icon="isEdit ? 'i-lucide-check' : 'i-lucide-plus'"
            :loading="loading"
          />
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4 px-6 sm:gap-6 lg:grid-cols-3">
        <div class="space-y-4 lg:col-span-2 sm:space-y-6">
          <RecipeBasicInfo v-model="model" :disabled="readOnly" />

          <UCard>
            <UFormField name="ingredients">
              <IngredientList
                :ingredients="model.ingredients"
                :disabled="readOnly"
                @add="ingredients.openAdd"
                @edit="ingredients.openEdit"
                @remove="ingredients.remove"
              />
            </UFormField>
          </UCard>

          <UCard>
            <UFormField name="steps">
              <StepList
                :steps="model.steps"
                :disabled="readOnly"
                @add="steps.openAdd"
                @edit="steps.openEdit"
                @remove="steps.remove"
                @move="steps.move"
                @reorder="steps.reorder"
              />
            </UFormField>
          </UCard>
        </div>

        <div class="space-y-4 lg:col-span-1 sm:space-y-6">
          <RecipeOrganisation v-model="model" :disabled="readOnly" />
          <RecipeTiming v-model="model" :disabled="readOnly" />
          <RecipeNutrition
            v-model="model"
            :disabled="readOnly"
            @enable="nutrition.enable"
            @disable="nutrition.disable"
          />
          <RecipeVisibility v-model="model" :disabled="readOnly" />
        </div>
      </div>
    </UForm>

    <StepDialog
      v-model:open="steps.visible"
      v-model:instructions="steps.instructions"
      v-model:duration="steps.duration"
      :title="steps.dialogTitle"
      :is-adding="steps.isAdding"
      :error="steps.error"
      @save="steps.save"
      @save-and-close="steps.saveAndClose"
      @close="steps.close"
    />

    <IngredientDialog
      v-model:open="ingredients.visible"
      v-model:name="ingredients.name"
      v-model:quantity="ingredients.quantity"
      v-model:unit="ingredients.unit"
      v-model:notes="ingredients.notes"
      :title="ingredients.dialogTitle"
      :is-adding="ingredients.isAdding"
      :error="ingredients.error"
      @save="ingredients.save"
      @save-and-close="ingredients.saveAndClose"
      @close="ingredients.close"
    />

    <AskLlmDialog v-model:open="llmDialogVisible" @import="applyLlmImport" />

    <UModal
      v-model:open="isDeleteModalOpen"
      :title="t('recipes.deleteDialog.title')"
      :description="t('recipes.deleteDialog.message', { name: model.title })"
      :dismissible="!deleteLoading"
      :ui="{ footer: 'justify-end' }"
    >
      <template #footer>
        <UButton
          :label="t('common.actions.cancel')"
          color="neutral"
          variant="outline"
          :disabled="deleteLoading"
          @click="closeDeleteModal"
        />
        <UButton
          :label="t('common.actions.delete')"
          color="error"
          :loading="deleteLoading"
          @click="confirmDeleteRecipe"
        />
      </template>
    </UModal>
  </div>
</template>
