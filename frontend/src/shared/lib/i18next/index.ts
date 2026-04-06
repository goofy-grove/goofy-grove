import i18next from 'i18next';
import Backend from 'i18next-http-backend';
import { initReactI18next } from 'react-i18next';

export type LocaleSettings = {
  locales: Record<string, string>;
  default: string;
};

const BASE = import.meta.env.BASE_URL || '/';
const DEFAULT_LOCALE_SETTINGS: LocaleSettings = {
  locales: { en: 'English' },
  default: 'en',
};

let _localeSettings: LocaleSettings = DEFAULT_LOCALE_SETTINGS;
let _initPromise: Promise<void> | null = null;

export const getAvailableLocales = async (): Promise<LocaleSettings> => {
  try {
    const res = await fetch(`${BASE}locales/index.json`);
    if (!res.ok) return DEFAULT_LOCALE_SETTINGS;
    const data = (await res.json()) as LocaleSettings;
    return data || DEFAULT_LOCALE_SETTINGS;
  } catch {
    return DEFAULT_LOCALE_SETTINGS;
  }
};

export const initI18n = async (): Promise<void> => {
  if (_initPromise) return _initPromise;

  _initPromise = (async () => {
    _localeSettings = await getAvailableLocales();

    // eslint-disable-next-line import/no-named-as-default-member
    await i18next
      .use(Backend)
      .use(initReactI18next)
      .init({
        lng: _localeSettings.default,
        fallbackLng: _localeSettings.default,
        debug: import.meta.env.DEV,
        load: 'languageOnly',
        backend: {
          loadPath: `${BASE}locales/{{lng}}.json`,
          crossDomain: false,
          allowMultiLoading: true,
        },
        interpolation: { escapeValue: false },
      });
  })();

  return _initPromise;
};

export const useLocaleSettings = (): LocaleSettings => _localeSettings;

export default i18next;
