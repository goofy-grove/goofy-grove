import { useTranslation } from 'react-i18next';

import { useLoginForm } from '@pages/login/model';

import {
  Alert,
  Button,
  Card,
  IconLoader,
  Input,
  LocaleSwitcher,
} from '@shared/ui';

import './styles.scss';

export const LoginPage = () => {
  const { t } = useTranslation();

  const {
    isLoading,
    username,
    password,
    error,
    setUsername,
    setPassword,
    handleSubmit,
  } = useLoginForm();

  return (
    <div className="login-page">
      <Card className="login-page-card" title={t('login.title')}>
        <div className="login-page-card__form">
          {error && <Alert type="error" message={t(error)} closable />}

          <Input
            id="username"
            value={username}
            disabled={isLoading}
            label={t('login.labels.username')}
            placeholder={t('login.labels.username')}
            onChange={setUsername}
          />

          <Input
            id="password"
            type="password"
            value={password}
            disabled={isLoading}
            label={t('login.labels.password')}
            placeholder={t('login.labels.password')}
            onChange={setPassword}
          />

          <Button
            onClick={handleSubmit}
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
