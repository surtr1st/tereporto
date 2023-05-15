import { createI18n } from 'vue-i18n';
import ja from './locales/ja.json';
import en from './locales/en.json';
import vi from './locales/vi.json';

type I18nLocale = 'en' | 'ja' | 'vi';

const messages = {
  en: {
    message: en,
  },
  ja: {
    message: ja,
  },
  vi: {
    message: vi,
  },
};

export const SUPPORT_LOCALES = ['en', 'ja', 'vi'];

export function setupI18n(locale: I18nLocale) {
  const i18n = createI18n({
    locale,
    legacy: false,
    fallbackLocale: 'en',
    messages,
  });
  return i18n;
}
