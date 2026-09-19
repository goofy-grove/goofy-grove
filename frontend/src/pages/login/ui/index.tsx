import { useTranslation } from 'react-i18next';

import { useLoginForm } from '@pages/login/model';

import {
  Alert,
  Button,
  Card,
  GroveScene,
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
      <section className="login-page__welcome">
        <span className="login-page__brand">goofy grove.</span>

        <GroveScene />

        <h1>{t('grove.welcome')}</h1>

        <p>{t('grove.welcome_description')}</p>
      </section>

      <div className="login-page__entry">
        <Card className="login-page-card" title={t('login.title')}>
          <form
            className="login-page-card__form"
            onSubmit={(event) => {
              event.preventDefault();
              if (!isLoading) void handleSubmit();
            }}
          >
            {error && <Alert type="error" message={t(error)} closable />}

            <Input
              id="username"
              autoComplete="username"
              name="username"
              value={username}
              disabled={isLoading}
              label={t('login.labels.username')}
              placeholder={t('login.labels.username')}
              onChange={setUsername}
            />

            <Input
              id="password"
              autoComplete="current-password"
              name="password"
              type="password"
              value={password}
              disabled={isLoading}
              label={t('login.labels.password')}
              placeholder={t('login.labels.password')}
              onChange={setPassword}
            />

            <Button
              type="submit"
              disabled={isLoading}
              leftIcon={isLoading && <IconLoader isAnimated />}
            >
              {t('login.login')}
            </Button>
          </form>
        </Card>

        <LocaleSwitcher />
      </div>
    </div>
  );
};
