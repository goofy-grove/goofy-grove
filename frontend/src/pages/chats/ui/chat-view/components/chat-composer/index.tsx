import { useTranslation } from 'react-i18next';

import { PersonaSelect } from '@pages/chats/ui/chat-view/components/persona-select';
import { RichEditor } from '@pages/chats/ui/chat-view/components/rich-editor';

import type { ChatComposerProps } from './types';
import type { FC } from 'react';

import '@pages/chats/ui/chat-view/components/role.scss';
import './styles.scss';

export const ChatComposer: FC<ChatComposerProps> = ({
  personas,
  selectedId,
  onPersonaChange,
  onSend,
  blocked = false,
  initialContent,
}) => {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });
  const available = personas.filter(
    (identity) => identity.kind === 'persona' && identity.own,
  );
  const selected = available.find((identity) => identity.id === selectedId);

  return (
    <section className="chat-composer" aria-label={t('editor')}>
      <div className="chat-composer__identity">
        <span>{t('writeAs')}</span>

        <PersonaSelect
          personas={available}
          selectedId={selectedId}
          onChange={onPersonaChange}
        />

        <span className="chat-role chat-role--persona">{t('persona')}</span>
      </div>

      <RichEditor
        initialContent={initialContent}
        blocked={blocked || !selected}
        onSend={onSend}
      />
    </section>
  );
};
