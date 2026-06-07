import { createApp } from 'vue'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config'
import ToastService from 'primevue/toastservice'
import ConfirmationService from 'primevue/confirmationservice'
import Tooltip from 'primevue/tooltip'

import App from './App.vue'
import router from './router'
import { i18n } from '@/i18n'

import './assets/main.css'
import 'primeicons/primeicons.css'
import { PinkyPreset } from './themes/pinky.ts'

const app = createApp(App)

app.use(createPinia())
app.use(i18n)
app.use(PrimeVue, {
  theme: {
    preset: PinkyPreset,
    options: {
      darkModeSelector: '.my-app-dark',
    },
  },
})
app.use(ConfirmationService)
app.directive('tooltip', Tooltip)
app.use(ToastService)
app.use(router)

app.mount('#app')
