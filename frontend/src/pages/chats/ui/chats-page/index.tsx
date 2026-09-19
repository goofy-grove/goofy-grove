import { Link } from '@tanstack/react-router';
import { useTranslation } from 'react-i18next';

import { GroveScene, PageHeader, Text } from '@shared/ui';

import './styles.scss';

export const ChatsPage = () => {
  const { t } = useTranslation();

  return (
    <div className="chats-page scrollbar">
      <PageHeader title={t('menu.chats')} />

      <div className="chats-page__empty">
        <GroveScene />

        <span className="chats-page__badge">{t('common.coming_soon')}</span>

        <Text tag="h2">{t('grove.chats_title')}</Text>

        <Text>{t('grove.chats_description')}</Text>

        <Link className="chats-page__link" to="/characters">
          {t('grove.explore_characters')} →
        </Link>
      </div>
    </div>
  );
};
