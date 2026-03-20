import { createI18n } from 'vue-i18n';

type LocaleSettings = {
  locales: Record<string, string>;
  default: string;
};

export const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages: {},
});

const BASE = import.meta.env.BASE_URL || '/';

const DEFAULT_LOCALE_SETTINGS: LocaleSettings = {
  locales: { en: 'English' },
  default: 'en',
};

export const loadLocale = async (locale: string): Promise<boolean> => {
  if (i18n.global.availableLocales.includes(locale)) {
    i18n.global.locale.value = locale;
    return true;
  }

  try {
    const response = await fetch(`${BASE}locales/${locale}.json`, {
      headers: { Accept: 'application/json' },
    });

    if (!response.ok) return false;

    const messages = await response.json();

    i18n.global.setLocaleMessage(locale, messages);
    i18n.global.locale.value = locale;

    return true;
  } catch {
    return false;
  }
};

export const getAvailableLocales = async (): Promise<LocaleSettings> => {
  try {
    const res = await fetch(`${BASE}locales/index.json`);

    if (!res.ok) return DEFAULT_LOCALE_SETTINGS;

    const data = await res.json();

    return data || DEFAULT_LOCALE_SETTINGS;
  } catch {
    return DEFAULT_LOCALE_SETTINGS;
  }
};

async function initI18n() {
  getAvailableLocales().then(({ default: defaultLocale }) => {
    loadLocale(defaultLocale);
  });
}

initI18n();
