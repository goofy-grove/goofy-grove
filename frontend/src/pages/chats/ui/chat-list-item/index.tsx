import { IconDots } from '@tabler/icons-react';
import { useState, type FC } from 'react';
import { useTranslation } from 'react-i18next';

import { FileAvatar, Button, IconForest, useFileUrl } from '@shared/ui';

import type { ChatListItemProps } from './types';

import './styles.scss';

export const ChatListItem: FC<ChatListItemProps> = ({
  uid,
  title,
  imageUrl,
  avatarUid,
  characters,
  lastMessage,
  activity,
  unreadCount = 0,
  generatingCharacterName,
  onOpen,
  onActions,
}) => {
  const { t } = useTranslation();
  const fileUrl = useFileUrl(imageUrl ? null : avatarUid);
  const sceneUrl = imageUrl ?? fileUrl;
  const [failedImage, setFailedImage] = useState<string>();
  const hasUnread = unreadCount > 0;
  const visibleCharacters = characters.slice(0, 3);

  return (
    <article
      className={`chat-list-item ${hasUnread ? 'chat-list-item--unread' : ''}`}
      aria-labelledby={`${uid}-title`}
    >
      <div className="chat-list-item__image" aria-hidden="true">
        {sceneUrl && failedImage !== sceneUrl ? (
          <img src={sceneUrl} alt="" onError={() => setFailedImage(sceneUrl)} />
        ) : (
          <IconForest />
        )}
      </div>

      <div className="chat-list-item__content">
        <h3 className="chat-list-item__title" id={`${uid}-title`}>
          <button
            type="button"
            onClick={() => onOpen(uid)}
            aria-describedby={`${uid}-preview${hasUnread ? ` ${uid}-unread` : ''}`}
          >
            {title}
          </button>
        </h3>

        <p
          id={`${uid}-preview`}
          className={`chat-list-item__preview ${generatingCharacterName ? 'chat-list-item__preview--generating' : ''}`}
        >
          {generatingCharacterName ? (
            <>
              <span className="chat-list-item__pulse" aria-hidden="true" />

              {t('chat.list.responding', { name: generatingCharacterName })}
            </>
          ) : lastMessage ? (
            <>
              <span className="chat-list-item__author">
                {`${lastMessage.author}: `}
              </span>

              {lastMessage.text}
            </>
          ) : (
            t('chat.list.noMessages')
          )}
        </p>

        <div className="chat-list-item__cast">
          <div className="chat-list-item__avatars" aria-hidden="true">
            {visibleCharacters.map((character) => (
              <div className="chat-list-item__avatar" key={character.id}>
                <FileAvatar
                  previewUrl={character.previewUrl}
                  fileUid={character.avatarUid}
                  fallback={character.initials}
                  alt=""
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

      <div className="chat-list-item__meta">
        {activity && <time dateTime={activity.dateTime}>{activity.label}</time>}

        {hasUnread && (
          <span className="chat-list-item__unread" id={`${uid}-unread`}>
            <span aria-hidden="true">
              {unreadCount > 99 ? '99+' : unreadCount}
            </span>

            <span className="chat-list-item__unread-label">
              {t('chat.list.unread', { count: unreadCount })}
            </span>
          </span>
        )}
      </div>

      {onActions && (
        <div className="chat-list-item__actions">
          <Button
            variant="ghost"
            aria-label={t('grove.item_actions', { name: title })}
            leftIcon={<IconDots size={20} />}
            onClick={() => onActions(uid)}
          />
        </div>
      )}
    </article>
  );
};
