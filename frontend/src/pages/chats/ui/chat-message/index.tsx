import { IconPencil, IconRefresh } from '@tabler/icons-react';
import { useId, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { ChatRole } from '@pages/chats/ui/chat-role';
import { IdentityAvatar } from '@pages/chats/ui/identity-avatar';
import { RichEditor } from '@pages/chats/ui/rich-editor';
import { RichMessage } from '@pages/chats/ui/rich-message';

import { Button } from '@shared/ui';

import type { ChatMessageProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const ChatMessage: FC<ChatMessageProps> = ({
  message,
  onEdit,
  onRetry,
  onEditingChange,
}) => {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });
  const [editing, setEditing] = useState(false);
  const titleId = useId();
  const canBeEdited =
    message.author?.kind === 'persona' &&
    message.author.own &&
    onEdit &&
    !message.removed &&
    message.status !== 'failed' &&
    !editing;

  const getMessageOwner = () => {
    if (message.author?.kind == 'persona') {
      return message.author.owner;
    }

    if (message.author) {
      return t('ai');
    }

    return '';
  };

  return (
    <article
      className={`chat-message chat-message--${message.author?.kind ?? 'unknown'} ${message.status === 'failed' ? 'chat-message--failed' : ''}`}
      aria-labelledby={titleId}
    >
      {message.author && <IdentityAvatar identity={message.author} />}

      <div className="chat-message__main">
        <header className="chat-message__header">
          <h3 id={titleId}>{message.author?.name ?? t('unknownAuthor')}</h3>

          <ChatRole kind={message.author?.kind} />

          <span className="chat-message__owner">{getMessageOwner()}</span>

          <time>{message.time}</time>

          {message.edited && (
            <span className="chat-message__edited">{t('edited')}</span>
          )}

          {canBeEdited && (
            <Button
              size="compact"
              variant="ghost"
              type="button"
              className="chat-message__edit"
              onClick={() => {
                setEditing(true);
                onEditingChange?.(message.id, true);
              }}
              aria-label={`${t('edit')}: ${message.author?.name ?? t('unknownAuthor')}`}
            >
              <IconPencil size={14} />

              {t('edit')}
            </Button>
          )}
        </header>

        {message.removed ? (
          <p>{t('removedMessage')}</p>
        ) : editing ? (
          <div className="chat-message__editing">
            <RichEditor
              initialContent={message.content}
              sendLabel={t('save')}
              onCancel={() => {
                setEditing(false);
                onEditingChange?.(message.id, false);
              }}
              onSend={(content) => {
                onEdit?.(message.id, content);
                setEditing(false);
                onEditingChange?.(message.id, false);
              }}
            />
          </div>
        ) : (
          <RichMessage content={message.content} />
        )}

        {!message.removed && message.status === 'failed' && (
          <div className="chat-message__error" role="alert">
            <span>{t('failed')}</span>

            {onRetry && (
              <Button
                size="compact"
                variant="ghost"
                type="button"
                onClick={() => onRetry(message.id)}
              >
                <IconRefresh size={16} />

                {t('retry')}
              </Button>
            )}
          </div>
        )}
      </div>
    </article>
  );
};
