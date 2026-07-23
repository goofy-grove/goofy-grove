import i18next from 'i18next';
import Backend from 'i18next-http-backend';
import { initReactI18next } from 'react-i18next';

import { getAvailableLocales } from './get-available-locales';
import { setLocaleSettings } from './locale-settings';

const BASE = import.meta.env.BASE_URL || '/';

let initPromise: Promise<void> | null = null;

export const initI18n = async (): Promise<void> => {
  if (initPromise) return initPromise;

  initPromise = (async () => {
    const localeSettings = await getAvailableLocales();
    setLocaleSettings(localeSettings);

    // eslint-disable-next-line import-x/no-named-as-default-member
    await i18next
      .use(Backend)
      .use(initReactI18next)
      .init({
        lng: localeSettings.default,
        fallbackLng: localeSettings.default,
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

  return initPromise;
};

export default i18next;
