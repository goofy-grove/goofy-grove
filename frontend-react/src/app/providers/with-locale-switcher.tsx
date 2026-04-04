import { useTranslation } from 'react-i18next';

import { useLocaleSettings } from '../lib';

export const withLocaleSwitcher = (component: () => React.ReactNode) => () => {
  const { i18n } = useTranslation();
  const { locales } = useLocaleSettings();

  const handleChange = async (lng: string) => {
    await i18n.changeLanguage(lng);
  };

  return (
    <>
      <select
        value={i18n.language}
        onChange={(e) => handleChange(e.target.value)}
      >
        {Object.entries(locales).map(([code, name]) => (
          <option key={code} value={code}>
            {name}
          </option>
        ))}
      </select>

      {component()}
    </>
  );
};
