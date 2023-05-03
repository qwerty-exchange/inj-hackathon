import { IS_PRODUCTION } from '~/app/utils/constants'

export const englishLocale = 'en'
export const russianLocale = 'ru'

export const availableLocales = IS_PRODUCTION
  ? [englishLocale]
  : [englishLocale, russianLocale]

export interface Locale {
  name: string
  flag?: string
  locale: string
}

export const english = {
  name: 'English',
  // flag: '/flags/en.svg',
  locale: englishLocale
}

export const russian = {
  name: 'Русский (Альфа)',
  // flag: '/flags/cn.png',
  locale: russianLocale
}

export const locales = IS_PRODUCTION ? [english] : [english, russian]

export const getLocale = (local: string, defaultValue: Locale) => {
  return locales.find((x) => x.locale === local) || defaultValue
}
