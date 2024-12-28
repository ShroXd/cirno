import { initReactI18next } from 'react-i18next'

import i18n from 'i18next'
import LanguageDetector from 'i18next-browser-languagedetector'

import en from '../locales/en.json'
import jp from '../locales/jp.json'
import cn from '../locales/zh-CN.json'

const resources = {
  en: { translation: en },
  'zh-CN': { translation: cn },
  jp: { translation: jp },
}

i18n
  .use(LanguageDetector) // Detect language automatically
  .use(initReactI18next) // Connect with React
  .init({
    resources,
    fallbackLng: 'en', // Default language
    debug: true, // Enable debug mode in development
    interpolation: {
      escapeValue: false, // React already safes from XSS
    },
  })

export default i18n
