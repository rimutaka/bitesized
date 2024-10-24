import './assets/main.css'
import 'primeicons/primeicons.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import PrimeVue from "primevue/config";
import Aura from '@primevue/themes/aura';
import { createAuth0 } from '@auth0/auth0-vue';

const app = createApp(App)

app.use(router)

app.use(PrimeVue, {
  theme: {
      preset: Aura
  }
})

app.use(
  createAuth0({
    domain: "dev-lbpjc402mmk4uxbs.us.auth0.com",
    clientId: "p2xjvyoxb8HoKSt1QNDx7CQ8Ka2lXgUJ",
    authorizationParams: {
      redirect_uri: window.location.origin
    }
  })
);

app.mount('#app')
