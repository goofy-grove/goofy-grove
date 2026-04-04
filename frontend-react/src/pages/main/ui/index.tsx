import { useTranslation } from 'react-i18next';

import { LocaleSwitcher } from './locale-switcher';

export const MainPage = () => {
  const { t } = useTranslation();

  return (
    <div>
      <span>{t('login.username')}</span>
      <LocaleSwitcher />
    </div>
  );
};
