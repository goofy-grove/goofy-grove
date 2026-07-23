import { DEFAULT_LOCALE_SETTINGS } from './locale-settings';

import type { LocaleSettings } from './types';

const BASE = import.meta.env.BASE_URL || '/';

export const getAvailableLocales = async (): Promise<LocaleSettings> => {
  try {
    const res = await fetch(`${BASE}locales/index.json`);

    if (!res.ok) {
      return DEFAULT_LOCALE_SETTINGS;
    }

    const data = (await res.json()) as LocaleSettings;

    return data || DEFAULT_LOCALE_SETTINGS;
  } catch {
    return DEFAULT_LOCALE_SETTINGS;
  }
};
