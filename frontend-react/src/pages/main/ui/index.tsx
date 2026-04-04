import { useTranslation } from 'react-i18next';

export const MainPage = () => {
  const { t } = useTranslation();

  return (
    <div>
      <span>{t('login.username')}</span>
    </div>
  );
};
