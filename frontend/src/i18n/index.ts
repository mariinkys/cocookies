import { createI18n } from 'vue-i18n'
import es from '@/i18n/locales/es'
import en from '@/i18n/locales/en'

export const i18n = createI18n({
  legacy: false,
  locale: 'es',
  fallbackLocale: 'en',
  messages: {
    es,
    en,
  },
})
