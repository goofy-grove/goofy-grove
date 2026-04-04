import { useTranslation } from 'react-i18next';
import { IconChevronDown } from '@tabler/icons-react';

import { Button, Card, Input, Modal, useModal } from '../../../shared/ui';

import { LocaleSwitcher } from './locale-switcher';

export const MainPage = () => {
  const { t } = useTranslation();
  const { openModal } = useModal();

  return (
    <Card closable>
      <span>{t('login.username')}</span>
      <LocaleSwitcher />
      <Input placeholder={t('login.username')} />
      <Button leftIcon={<IconChevronDown />} onClick={() => openModal('hello')}>
        {t('login.login')}
      </Button>

      <Modal id="hello">Hello, world</Modal>
    </Card>
  );
};
