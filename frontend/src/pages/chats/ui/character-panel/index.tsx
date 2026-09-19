import { IconPencil, IconPlayerPlay } from '@tabler/icons-react';
import { useTranslation } from 'react-i18next';

import { IdentityAvatar } from '@pages/chats/ui/identity-avatar';

import { Button } from '@shared/ui';

import type { CharacterPanelProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const CharacterPanel: FC<CharacterPanelProps> = ({
  characters,
  currentUserId,
  generatingId,
  disabled,
  onTrigger,
  onEdit,
}) => {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });

  return (
    <section className="chat-characters" aria-label={t('characters')}>
      <span className="chat-characters__label">{t('characters')}</span>

      <div className="chat-characters__list">
        {characters.map((character) => (
          <div
            key={character.id}
            className={`chat-character ${generatingId === character.id ? 'chat-character--active' : ''}`}
          >
            <IdentityAvatar identity={character} small />

            <strong>{character.name}</strong>

            <Button
              size="compact"
              variant="ghost"
              type="button"
              disabled={disabled || !!generatingId}
              aria-label={`${t('trigger')}: ${character.name}`}
              onClick={() => onTrigger(character.id)}
            >
              <IconPlayerPlay size={15} />

              {t('trigger')}
            </Button>

            {character.ownerId === currentUserId && !disabled && (
              <Button
                size="compact"
                variant="ghost"
                type="button"
                className="chat-character__edit"
                aria-label={`${t('editCharacter')}: ${character.name}`}
                title={t('editCharacter')}
                disabled={!!generatingId}
                onClick={() => onEdit(character.id)}
              >
                <IconPencil size={17} />
              </Button>
            )}
          </div>
        ))}
      </div>

      {!characters.length && <span>{t('noCharacters')}</span>}
    </section>
  );
};
