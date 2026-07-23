import { useTranslation } from 'react-i18next';

import { Text } from '@shared/ui';

import type { FC } from 'react';

import './styles.scss';

export const ChatsPage: FC = () => {
  const { t } = useTranslation();

  return (
    <div className="chats-page">
      <Text tag="h2">{t('menu.chats')}</Text>
      <Text>{t('common.coming_soon')}</Text>
    </div>
  );
};
