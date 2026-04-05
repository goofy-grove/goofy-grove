import { useTranslation } from 'react-i18next';
import { useState } from 'react';

import { Button, Card, IconLoader, Input } from '../../../shared/ui';
import './styles.scss';
import { LocaleSwitcher } from '../../main/ui/locale-switcher';

export const LoginPage = () => {
  const { t } = useTranslation();

  const [isLoading, setIsLoading] = useState(false);

  const handleLogin = () => {
    setIsLoading(true);
  };

  return (
    <div className="login-page">
      <Card className="login-page-card" title={t('login.title')}>
        <div className="login-page-card__form">
          <Input
            id="username"
            disabled={isLoading}
            label={t('login.labels.username')}
            placeholder={t('login.labels.username')}
          />

          <Input
            id="password"
            disabled={isLoading}
            label={t('login.labels.password')}
            placeholder={t('login.labels.password')}
          />

          <Button
            onClick={handleLogin}
            disabled={isLoading}
            leftIcon={isLoading && <IconLoader isAnimated />}
          >
            {t('login.login')}
          </Button>
        </div>
      </Card>

      <LocaleSwitcher />
    </div>
  );
};
