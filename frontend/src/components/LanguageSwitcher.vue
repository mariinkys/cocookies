<script setup lang="ts">
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const LANG_STORAGE_KEY = 'socios-peix-lang'
const { locale } = useI18n({ useScope: 'global' })

function setLanguage(lang: 'es' | 'en') {
  locale.value = lang
  localStorage.setItem(LANG_STORAGE_KEY, lang)
}

onMounted(() => {
  const saved = localStorage.getItem(LANG_STORAGE_KEY)
  if (saved === 'es' || saved === 'en') {
    locale.value = saved
  }
})
</script>

<template>
  <div class="flex items-center">
    <UButton
      :label="locale === 'es' ? 'ES' : 'EN'"
      :aria-label="locale === 'es' ? 'Cambiar a Español' : 'Switch to English'"
      @click="setLanguage(locale === 'es' ? 'en' : 'es')"
    />
  </div>
</template>
