<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Form } from '@primevue/forms'
import Card from 'primevue/card'
import Button from 'primevue/button'
import ConfirmDialog from 'primevue/confirmdialog'
import { useI18n } from 'vue-i18n'

import { useRecipeForm } from '@/composables/useRecipeForm'
import { useRecipeSteps } from '@/composables/useRecipeSteps'
import { useRecipeIngredients } from '@/composables/useRecipeIngredients'
import { useRecipeNutrition } from '@/composables/useRecipeNutrition'
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
  isEdit,
  isOwnRecipe,
  loading,
  fetchLoading,
  deleteLoading,
  hasListErrors,
  fetchRecipe,
  resolver,
  onSubmit,
  confirmDelete,
} = useRecipeForm(recipeId)

const steps = useRecipeSteps(model)
const ingredients = useRecipeIngredients(model)
const nutrition = useRecipeNutrition(model)
const { llmDialogVisible, formKey, applyLlmImport } = useLlmDialog(model)

onMounted(fetchRecipe)
</script>

<template>
  <div class="px-4 py-4 sm:px-6 sm:py-6 space-y-4 sm:space-y-6">
    <div v-if="fetchLoading" class="flex items-center justify-center py-24">
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
    </div>

    <Form
      v-else
      :key="formKey"
      v-slot="$form"
      :initialValues="model"
      :resolver
      :validateOnBlur="true"
      :validateOnValueUpdate="true"
      class="space-y-4 sm:space-y-6"
      @submit="onSubmit"
    >
      <ConfirmDialog />

      <StepDialog
        :visible="steps.visible.value"
        @update:visible="steps.visible.value = $event"
        :instructions="steps.instructions.value"
        @update:instructions="steps.instructions.value = $event"
        :duration="steps.duration.value"
        @update:duration="steps.duration.value = $event"
        :title="steps.dialogTitle.value"
        :isAdding="steps.isAdding.value"
        :error="steps.error.value"
        @save="steps.save"
        @saveAndClose="steps.saveAndClose"
        @close="steps.close"
      />

      <IngredientDialog
        :visible="ingredients.visible.value"
        @update:visible="ingredients.visible.value = $event"
        :name="ingredients.name.value"
        @update:name="ingredients.name.value = $event"
        :quantity="ingredients.quantity.value"
        @update:quantity="ingredients.quantity.value = $event"
        :unit="ingredients.unit.value"
        @update:unit="ingredients.unit.value = $event"
        :notes="ingredients.notes.value"
        @update:notes="ingredients.notes.value = $event"
        :title="ingredients.dialogTitle.value"
        :isAdding="ingredients.isAdding.value"
        :error="ingredients.error.value"
        @save="ingredients.save"
        @saveAndClose="ingredients.saveAndClose"
        @close="ingredients.close"
      />

      <AskLlmDialog v-model:visible="llmDialogVisible" @import="applyLlmImport" />

      <!-- Page header -->
      <div class="flex items-center justify-between gap-3 flex-wrap">
        <div class="flex items-center gap-3">
          <Button
            icon="pi pi-arrow-left"
            severity="secondary"
            text
            rounded
            :aria-label="t('common.actions.back')"
            @click="router.push('/')"
          />
          <div>
            <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
              {{ isEdit ? t('recipes.titles.edit') : t('recipes.titles.create') }}
            </h1>
            <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
              {{ isEdit ? t('recipes.descriptions.edit') : t('recipes.descriptions.create') }}
            </p>
          </div>
        </div>
        <div v-if="isOwnRecipe" class="flex items-center gap-2 shrink-0">
          <Button
            :label="t('common.actions.cancel')"
            severity="secondary"
            outlined
            @click="router.push('/')"
          />
          <Button
            type="submit"
            :label="isEdit ? t('common.actions.saveChanges') : t('recipes.actions.create')"
            :icon="isEdit ? 'pi pi-check' : 'pi pi-plus'"
            icon-pos="right"
            :loading="loading"
            :disabled="!$form.valid || hasListErrors"
          />
          <Button
            v-if="!isEdit"
            :label="t('recipes.import.trigger')"
            icon="pi pi-sparkles"
            severity="secondary"
            outlined
            @click="llmDialogVisible = true"
          />
          <Button
            v-if="isEdit"
            icon="pi pi-trash"
            severity="danger"
            outlined
            :loading="deleteLoading"
            :aria-label="t('recipes.actions.delete')"
            @click="confirmDelete"
          />
        </div>
      </div>

      <!-- Body grid -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-4 sm:gap-6 items-start">
        <div class="lg:col-span-2 space-y-4 sm:space-y-6">
          <RecipeBasicInfo v-model="model" />

          <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
            <template #content>
              <IngredientList
                :ingredients="model.ingredients"
                :formError="($form.errors as Record<string, any>)?.['ingredients']?.[0]?.message"
                @add="ingredients.openAdd"
                @edit="ingredients.openEdit"
                @remove="ingredients.remove"
              />
            </template>
          </Card>

          <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
            <template #content>
              <StepList
                :steps="model.steps"
                :formError="($form.errors as Record<string, any>)?.['steps']?.[0]?.message"
                @add="steps.openAdd"
                @edit="steps.openEdit"
                @remove="steps.remove"
                @move="steps.move"
                @reorder="steps.reorder"
              />
            </template>
          </Card>
        </div>

        <!-- Right Side Col -->
        <div class="lg:col-span-1 space-y-4 sm:space-y-6">
          <RecipeOrganisation v-model="model" />
          <RecipeTiming v-model="model" />
          <RecipeNutrition
            v-model="model"
            @enable="nutrition.enable"
            @disable="nutrition.disable"
          />
          <RecipeVisibility v-model="model" />
        </div>
      </div>
    </Form>
  </div>
</template>
