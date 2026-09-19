import { IconTrees, IconUsers } from '@tabler/icons-react';
import { useEffect, useRef } from 'react';
import { useTranslation } from 'react-i18next';

import { CharacterPanel } from '@pages/chats/ui/character-panel';
import { CharacterThinking } from '@pages/chats/ui/character-thinking';
import { ChatComposer } from '@pages/chats/ui/chat-composer';
import { VirtualTimeline } from '@pages/chats/ui/virtual-timeline';

import type { ChatViewProps } from './types';
import type { CSSProperties, FC } from 'react';

import './styles.scss';

export const ChatView: FC<ChatViewProps> = (props) => {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });
  const container = useRef<HTMLElement>(null);

  useEffect(() => {
    const viewport = window.visualViewport;

    const resize = () =>
      container.current?.style.setProperty(
        '--chat-viewport-height',
        `${viewport?.height ?? window.innerHeight}px`,
      );

    resize();
    viewport?.addEventListener('resize', resize);

    return () => viewport?.removeEventListener('resize', resize);
  }, []);
  const activeCharacter = props.characters.find(
    (character) => character.id === props.generatingCharacterId,
  );
  const background: CSSProperties | undefined = props.backgroundUrl
    ? {
        backgroundImage: `linear-gradient(rgba(16,29,23,.68), rgba(16,29,23,.86)), url(${JSON.stringify(props.backgroundUrl)})`,
      }
    : undefined;

  return (
    <section
      ref={container}
      className={`chat-view ${props.backgroundUrl ? 'chat-view--background' : ''}`}
      style={background}
    >
      <header className="chat-header">
        <div className="chat-header__mark">
          <IconTrees size={27} stroke={1.5} />
        </div>

        <div className="chat-header__title">
          <h1>{props.title}</h1>

          {props.subtitle && <p>{props.subtitle}</p>}
        </div>

        <details className="chat-users">
          <summary aria-label={t('users')}>
            <IconUsers size={17} />

            {props.users.length}
          </summary>

          <ul>
            {props.users.map((user) => (
              <li key={user.id}>{user.name}</li>
            ))}
          </ul>
        </details>
      </header>

      <CharacterPanel
        characters={props.characters}
        currentUserId={props.currentUserId}
        generatingId={props.generatingCharacterId}
        disabled={props.readOnly}
        onTrigger={props.onTriggerCharacter}
        onEdit={props.onEditCharacter}
      />

      <VirtualTimeline
        messages={props.messages}
        hasOlder={props.hasOlder}
        loadOlder={props.loadOlder}
        pageSize={props.pageSize}
        onEdit={props.readOnly ? undefined : props.onEditMessage}
        onRetry={props.readOnly ? undefined : props.onRetry}
        footer={
          activeCharacter && (
            <CharacterThinking
              character={activeCharacter}
              onStop={props.onStop}
            />
          )
        }
        empty={
          <div className="chat-empty">
            <IconTrees size={54} stroke={1} />

            <h2>{t('emptyTitle')}</h2>

            <p>{t('emptyText')}</p>
          </div>
        }
      />

      {props.notice && (
        <p className="chat-notice" role="status">
          {props.notice}
        </p>
      )}

      {props.readOnly ? (
        <p className="chat-read-only">{t('readOnly')}</p>
      ) : (
        <ChatComposer
          personas={props.myPersonas}
          selectedId={props.selectedPersonaId}
          onPersonaChange={props.onPersonaChange}
          onSend={props.onSend}
          blocked={!!props.generatingCharacterId}
        />
      )}
    </section>
  );
};
