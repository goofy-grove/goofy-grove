import { IconPlayerStop } from '@tabler/icons-react';
import { useTranslation } from 'react-i18next';

import { IdentityAvatar } from '@pages/chats/ui/chat-view/components/identity-avatar';

import { Button } from '@shared/ui';

import type { CharacterThinkingProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const CharacterThinking: FC<CharacterThinkingProps> = ({
  character,
  onStop,
}) => {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });

  return (
    <div className="chat-thinking">
      <IdentityAvatar identity={character} small />

      <span role="status">{`${character.name} ${t('thinking')}`}</span>

      <span className="chat-thinking__dots" aria-hidden="true">
        <i />

        <i />

        <i />
      </span>

      {onStop && (
        <Button
          size="compact"
          variant="ghost"
          type="button"
          onClick={onStop}
        >
          <IconPlayerStop size={14} />

          {t('stop')}
        </Button>
      )}
    </div>
  );
};
