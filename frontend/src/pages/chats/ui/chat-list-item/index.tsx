import { IconDotsVertical } from '@tabler/icons-react';
import { useTranslation } from 'react-i18next';

import { FileAvatar, Button, IconForest, useBreakpoints } from '@shared/ui';

import type { ChatListItemProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const ChatListItem: FC<ChatListItemProps> = ({
  uid,
  title,
  avatarUid,
  characters,
  lastMessage,
  generatingCharacterName,
  onOpen,
  onActions,
}) => {
  const { t } = useTranslation();
  const { isMobileSm } = useBreakpoints();
  // FIXME: move to contstant
  const visibleCharacters = characters.slice(0, 3);

  const handleActions = (e: React.MouseEvent) => {
    e.stopPropagation();

    onActions?.(uid);
  };

  return (
    <article className="chat-list-item" onClick={() => onOpen?.(uid)}>
      <div className="chat-list-item__image">
        <FileAvatar
          fileUid={avatarUid}
          fallback={<IconForest />}
          variant="unbordered"
        />
      </div>

      <div className="chat-list-item__content">
        <h3 className="chat-list-item__title">{title}</h3>

        <div
          className={`chat-list-item__preview ${generatingCharacterName ? 'chat-list-item__preview--generating' : ''}`}
        >
          {generatingCharacterName ? (
            <div className="chat-list-item__generating-block">
              <span className="chat-list-item__pulse" />

              <p className="chat-list-item__preview-responding-name">
                {generatingCharacterName}
              </p>

              <span>{t('chat.list.responding')}</span>
            </div>
          ) : lastMessage ? (
            <div className="chat-list-item__message-block">
              <span className="chat-list-item__author">
                {lastMessage.author}
              </span>

              <span className="chat-list-item__text">
                {isMobileSm ? lastMessage.text : `: ${lastMessage.text}`}
              </span>
            </div>
          ) : (
            t('chat.list.noMessages')
          )}
        </div>

        <div className="chat-list-item__cast">
          <div className="chat-list-item__avatars">
            {visibleCharacters.map((character) => (
              <div className="chat-list-item__avatar" key={character.id}>
                <FileAvatar
                  previewUrl={character.previewUrl}
                  fileUid={character.avatarUid}
                  fallback={character.initials}
                  variant="unbordered"
                />
              </div>
            ))}
          </div>

          <span
            title={characters.map((character) => character.name).join(', ')}
          >
            {t('chat.list.characters', { count: characters.length })}
          </span>
        </div>
      </div>

      {onActions && (
        <div className="chat-list-item__actions">
          <Button
            variant="ghost"
            leftIcon={<IconDotsVertical size={24} />}
            onClick={handleActions}
          />
        </div>
      )}
    </article>
  );
};
