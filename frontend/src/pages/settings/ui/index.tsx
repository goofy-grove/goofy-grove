import { useNavigate } from '@tanstack/react-router';
import { useTranslation } from 'react-i18next';

import { useAuth } from '@entities/auth';

import { Button, LocaleSwitcher, PageHeader, Text } from '@shared/ui';

import type { FC } from 'react';

import './styles.scss';

export const SettingsPage: FC = () => {
  const { t } = useTranslation();
  const { logout } = useAuth();
  const navigate = useNavigate();

  const handleLogout = async () => {
    await logout();
    void navigate({ to: '/login', replace: true });
  };

  return (
    <div className="settings-page">
      <PageHeader title={t('menu.settings')} />

      <Text>{t('grove.settings_intro')}</Text>

      <section className="settings-page__section">
        <Text tag="h3">{t('ui.select_locale')}</Text>

        <LocaleSwitcher />
      </section>

      <section className="settings-page__section">
        <Text tag="h3">{t('grove.session')}</Text>

        <Button color="error" onClick={() => void handleLogout()}>
          {t('auth.logout')}
        </Button>
      </section>
    </div>
  );
};
