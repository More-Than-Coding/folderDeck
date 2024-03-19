// Import CSS
import './main.css'

// Import modules
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createI18n } from 'vue-i18n'

// Import App
import App from './App.vue'

// Import plugins
import { locales } from '@src/plugins/locale.js'
import { router } from '@src/plugins/router.js'

// Create App
const app = createApp(App)

// Use
app.use(createPinia())
app.use(
  createI18n({
    locale: 'en',
    legacy: false,
    fallbackLocale: 'en',
    messages: locales,
  })
)
app.use(router)

// Mount
app.mount('#app')

// Vue Devtools
if (import.meta.env.DEV) {
  console.warn('👨🏾‍💻 Running in Dev Mode')
}
