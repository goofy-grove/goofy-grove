import { getLocaleSettings } from './locale-settings';

import type { LocaleSettings } from './types';

export const useLocaleSettings = (): LocaleSettings => getLocaleSettings();
