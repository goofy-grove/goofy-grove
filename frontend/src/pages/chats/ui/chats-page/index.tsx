import { IconPlusFilled } from '@tabler/icons-react';
import { Link, useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import {
  toChatSummary,
  useChatActions,
  useChatsQuery,
} from '@pages/chats/model';
import { ChatListItem } from '@pages/chats/ui/chat-list-item';

import {
  GroveScene,
  PageHeader,
  Text,
  IconLoader,
  Button,
  useBreakpoints,
  ActionSheet,
  Dropdown,
  ConfirmModal,
} from '@shared/ui';

import './styles.scss';

export const ChatsPage = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { data, isLoading } = useChatsQuery();

  const chats = (data ?? []).map(toChatSummary);

  const handleRedirectToChat = (uid: string) => {
    void navigate({
      to: '/chats/$uid',
      params: { uid },
    });
  };

  const { isMobileSm } = useBreakpoints();

  const {
    actions,
    isActionsOpened,
    isDeleteConfirmationOpen,
    handleClose,
    handleOpen,
    handleCloseDeleteConfirmation,
    handleConfirmDeletion,
    isRemovePending,
  } = useChatActions();

  const [actionButton, setActionButton] = useState<HTMLButtonElement | null>(
    null,
  );

  return (
    <div className="chats-page scrollbar">
      <PageHeader className="chats-page__header" title={t('menu.chats')}>
        <Button
          leftIcon={<IconPlusFilled size={18} />}
          onClick={() => void navigate({ to: '/chats/new' })}
        >
          {t('chat.create_title')}
        </Button>
      </PageHeader>

      <p className="chats-page__intro">{t('grove.chats_intro')}</p>

      <div className="chats-page__list scrollbar">
        {chats.map((chat) => (
          <ChatListItem
            key={chat.uid}
            {...chat}
            onOpen={handleRedirectToChat}
            onActions={(uid, ref) => {
              setActionButton(ref);
              handleOpen(uid);
            }}
          />
        ))}

        {!isMobileSm && (
          <Dropdown
            contentClass="chats-page__list__dropdown"
            virtualReference={actionButton}
            isOpen={isActionsOpened}
            onHide={handleClose}
          >
            {actions.map(({ Icon, title, color, onClick }, index) => (
              <Button
                key={index}
                variant="ghost"
                color={color}
                leftIcon={Icon}
                onClick={onClick}
              >
                {title}
              </Button>
            ))}
          </Dropdown>
        )}

        {isMobileSm && (
          <ActionSheet
            isOpened={isActionsOpened}
            items={actions}
            showClose
            onClose={handleClose}
          />
        )}
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

      <ConfirmModal
        isOpen={isDeleteConfirmationOpen}
        message={t('chat.confirm_delete')}
        confirmLabel={t('chat.actions.delete')}
        cancelLabel={t('common.cancel')}
        isPending={isRemovePending}
        onConfirm={handleConfirmDeletion}
        onCancel={handleCloseDeleteConfirmation}
      />
    </div>
  );
};
