import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import enUS from "../locales/en-US.json";
import cn from "../locales/zh-CN.json";
import jp from "../locales/jp.json";

const resources = {
  "en-US": enUS,
  en: enUS,
  "zh-CN": cn,
  zh: cn,
  jp: jp,
};

i18n
  .use(LanguageDetector) // Detect language automatically
  .use(initReactI18next) // Connect with React
  .init({
    resources,
    fallbackLng: "en-US", // Default language
    debug: true, // Enable debug mode in development
  });

export default i18n;
