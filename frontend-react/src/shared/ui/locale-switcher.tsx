import { useTranslation } from 'react-i18next';

import { useLocaleSettings } from '../lib';

import { Select } from './select';

export const LocaleSwitcher = () => {
  const { i18n, t } = useTranslation();
  const { locales } = useLocaleSettings();

  const handleChange = async (lng: string) => {
    await i18n.changeLanguage(lng);
  };

  const items = Object.entries(locales).map(([value, label]) => ({
    value,
    label,
  }));

  return (
    <Select
      items={items}
      selected={i18n.language}
      onChange={(lang) => handleChange(lang)}
      placeholder={t('ui.select_locale')}
    />
  );
};
