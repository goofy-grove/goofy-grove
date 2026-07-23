import { useNavigate } from '@tanstack/react-router';
import { useTranslation } from 'react-i18next';

import { useAuth } from '@entities/auth';

import { Button, LocaleSwitcher, Text } from '@shared/ui';

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
      <Text tag="h2">{t('menu.settings')}</Text>
      <LocaleSwitcher />
      <Button color="error" onClick={() => void handleLogout()}>
        {t('auth.logout')}
      </Button>
    </div>
  );
};
