import type { LocaleSettings } from './types';

export const DEFAULT_LOCALE_SETTINGS: LocaleSettings = {
  locales: { en: 'English' },
  default: 'en',
};

let localeSettings: LocaleSettings = DEFAULT_LOCALE_SETTINGS;

export const getLocaleSettings = (): LocaleSettings => localeSettings;

export const setLocaleSettings = (settings: LocaleSettings) => {
  localeSettings = settings;
};
