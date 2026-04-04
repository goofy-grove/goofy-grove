import { useTranslation } from 'react-i18next';
import { IconChevronDown } from '@tabler/icons-react';

import { Button, Card, Input } from '../../../shared/ui';

import { LocaleSwitcher } from './locale-switcher';

export const MainPage = () => {
  const { t } = useTranslation();

  return (
    <Card closable>
      <span>{t('login.username')}</span>
      <LocaleSwitcher />
      <Input placeholder={t('login.username')} />
      <Button leftIcon={<IconChevronDown />}>{t('login.login')}</Button>
    </Card>
  );
};
