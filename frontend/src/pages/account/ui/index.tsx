import { IconLogout } from '@tabler/icons-react';
import { useNavigate } from '@tanstack/react-router';
import { useTranslation } from 'react-i18next';

import { useAuth } from '@entities/auth';

import { Button, Form, Input, PageHeader, Text } from '@shared/ui';

import './styles.scss';

export const AccountPage = () => {
  const { t } = useTranslation();

  const { logout, user } = useAuth();
  const navigate = useNavigate();

  const handleLogout = async () => {
    await logout();
    void navigate({ to: '/login', replace: true });
  };

  return (
    <div className="account-page">
      <PageHeader title={t('account.title')}>
        <Button
          variant="ghost"
          color="error"
          onClick={() => void handleLogout()}
          leftIcon={<IconLogout />}
        >
          {t('auth.logout')}
        </Button>
      </PageHeader>

      <Text className="account-page__intro">{t('account.intro')}</Text>

      <Form
        headingText={t('account.info.title')}
        submitLabel={t('account.info.save')}
        hasPortrait
      >
        <Input
          label={t('account.info.username')}
          value={user?.username}
          isCopyable
          disabled
        />
      </Form>
    </div>
  );
};
