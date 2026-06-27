import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import { i18n } from '@/i18n'
import ui from '@nuxt/ui/vue-plugin'

import './assets/main.css'

const app = createApp(App)

app.use(createPinia())
app.use(i18n)
app.use(router)
app.use(ui)

app.mount('#app')
