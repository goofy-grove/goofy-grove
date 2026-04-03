import { useTranslation } from 'react-i18next';

import { LanguageSelector } from './language-selector';

export const MainPage = () => {
  const { t } = useTranslation();

  return (
    <div>
      <span>{t('login.username')}</span>
      <LanguageSelector />
    </div>
  );
};
