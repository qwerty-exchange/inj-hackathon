import { nuxtMetaTags, metaTags } from './meta'

const meta = [
  { charset: 'utf-8' },
  { name: 'viewport', content: 'width=device-width, initial-scale=1' },
  ...nuxtMetaTags
]

if (process.env.VITE_GOOGLE_SITE_VERIFICATION_KEY) {
  meta.push({
    name: 'google-site-verification',
    content: '' // process.env.VITE_GOOGLE_SITE_VERIFICATION_KEY
  })
}

export default {
  title: metaTags.title,
  htmlAttrs: {
    lang: 'en'
  },
  bodyAttrs: {
    class: 'font-sans'
  },
  meta,
  link: [
    { rel: 'icon', type: 'image/png', href: '/favicon.png' },
    { rel: 'shortcut icon', type: 'image/png', href: '/favicon.png' },
    { rel: 'apple-touch-icon', type: 'image/png', href: '/favicon.png' }
  ],
  script: [
    {
      src: '/chart/charting_library/charting_library.js',
      body: 'true'
    },
    {
      innerHTML: `
      (function(h,o,t,j,a,r){
        h.hj=h.hj||function(){(h.hj.q=h.hj.q||[]).push(arguments)};
        h._hjSettings={hjid:3180711,hjsv:6};
        a=o.getElementsByTagName('head')[0];
        r=o.createElement('script');r.async=1;
        r.src=t+h._hjSettings.hjid+j+h._hjSettings.hjsv;
        a.appendChild(r);
      })(window,document,'https://static.hotjar.com/c/hotjar-','.js?sv=');
      `
    }
  ] as any
}
