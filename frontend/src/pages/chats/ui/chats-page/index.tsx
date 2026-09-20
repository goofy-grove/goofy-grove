import { IconPlusFilled } from '@tabler/icons-react';
import { Link } from '@tanstack/react-router';
import { useTranslation } from 'react-i18next';

import { toChatSummary, useChatsQuery } from '@pages/chats/model';
import { ChatListItem } from '@pages/chats/ui/chat-list-item';

import { GroveScene, PageHeader, Text, IconLoader, Button } from '@shared/ui';

import './styles.scss';

export const ChatsPage = () => {
  const { t } = useTranslation();
  const { data, isLoading } = useChatsQuery();

  const chats = (data ?? []).map(toChatSummary);

  return (
    <div className="chats-page scrollbar">
      <PageHeader className="chats-page__header" title={t('menu.chats')}>
        <Button leftIcon={<IconPlusFilled size={18} />} onClick={() => {}}>
          {t('chat.create_title')}
        </Button>
      </PageHeader>

      <p className="chats-page__intro">{t('grove.chats_intro')}</p>

      <div className="chats-page__list scrollbar">
        {chats.map((chat) => (
          <ChatListItem
            key={chat.uid}
            {...chat}
            onOpen={() => {}}
            onActions={() => {}}
          />
        ))}

        {chats.map((chat) => (
          <ChatListItem
            key={chat.uid}
            {...chat}
            onOpen={() => {}}
            onActions={() => {}}
          />
        ))}

        {chats.map((chat) => (
          <ChatListItem
            key={chat.uid}
            {...chat}
            onOpen={() => {}}
            onActions={() => {}}
          />
        ))}
      </div>

      {isLoading && <IconLoader size={64} isAnimated />}

      {data?.length === 0 && (
        <div className="chats-page__empty">
          <GroveScene />

          <span className="chats-page__badge">{t('common.coming_soon')}</span>

          <Text tag="h2">{t('grove.chats_title')}</Text>

          <Text>{t('grove.chats_description')}</Text>

          <Link className="chats-page__link" to="/characters">
            {t('grove.explore_characters')} →
          </Link>
        </div>
      )}
    </div>
  );
};
