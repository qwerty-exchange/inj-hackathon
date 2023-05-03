import VueGtag, { trackRouter } from 'vue-gtag-next'
export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.use(VueGtag, {
    property: {
      id: process.env.VITE_GOOGLE_TAG_MANAGER
    }
  })
  trackRouter(useRouter())
})
