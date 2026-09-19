import { useEffect, useMemo, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { CharacterForm } from '@pages/characters/ui/character-form';
import { ChatView } from '@pages/chats';
import {
  CHAT_PAGE_SIZE,
  type ChatCharacter,
  type ChatMessageData,
} from '@pages/chats/model';

import { useObjectUrl } from '@shared/ui/hooks';

import forest from './assets/forest.svg';
import { createScene } from './model';

import type { JSONContent } from '@tiptap/react';

import './demo.scss';

export type ChatPreviewProps = {
  state?:
    | 'conversation'
    | 'empty'
    | 'generating'
    | 'failed'
    | 'read-only'
    | 'history'
    | 'history-error';
  background?: boolean;
};

export function ChatPreview({
  state = 'conversation',
  background = false,
}: ChatPreviewProps) {
  return (
    <ChatScene
      key={state}
      state={state}
      background={background}
    />
  );
}

function CharacterEditor({
  character,
  onClose,
  onSave,
}: {
  character: ChatCharacter;
  onClose: () => void;
  onSave: (changes: Partial<ChatCharacter>) => void;
}) {
  const { t } = useTranslation('translation', { keyPrefix: 'chat' });
  const [name, setName] = useState(character.name);
  const [description, setDescription] = useState(character.description ?? '');
  const [file, setFile] = useState<File | null>(null);
  const preview = useObjectUrl(file);
  const dialog = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    dialog.current?.showModal();
  }, []);

  return (
    <dialog
      ref={dialog}
      className="chat-demo-dialog"
      onCancel={onClose}
      aria-label={t('editCharacter')}
    >
      <h2>{t('editCharacter')}</h2>

      <CharacterForm
        name={name}
        description={description}
        avatarUid={character.avatarUid ?? undefined}
        avatarPreviewUrl={preview ?? character.previewUrl}
        isPending={false}
        submitLabel={t('save')}
        onNameChange={setName}
        onDescriptionChange={setDescription}
        onAvatarChange={setFile}
        onCancel={onClose}
        onSubmit={() => {
          if (file) {
            const reader = new FileReader();

            reader.onload = () => {
              if (typeof reader.result !== 'string') return;

              onSave({ name, description, previewUrl: reader.result });
              onClose();
            };
            reader.readAsDataURL(file);
          } else {
            onSave({ name, description });
            onClose();
          }
        }}
      />
    </dialog>
  );
}

function ChatScene({ state, background }: Required<ChatPreviewProps>) {
  const { t, i18n } = useTranslation('translation', { keyPrefix: 'chat' });
  const isEnglish = i18n.resolvedLanguage?.startsWith('en') ?? false;
  const scene = useMemo(() => createScene(isEnglish), [isEnglish]);
  const history = useMemo(
    () =>
      Array.from(
        { length: 250 },
        (_, index): ChatMessageData => ({
          ...scene.messages[index % scene.messages.length],
          id: `history-${index}`,
          time: `${String(10 + Math.floor(index / 60)).padStart(2, '0')}:${String(index % 60).padStart(2, '0')}`,
        }),
      ),
    [scene],
  );
  const historyMode = state === 'history' || state === 'history-error';
  const [oldest, setOldest] = useState(historyMode ? 200 : 0);
  const [messages, setMessages] = useState<ChatMessageData[]>(() =>
    historyMode
      ? history.slice(-CHAT_PAGE_SIZE)
      : state === 'empty'
        ? []
        : state === 'generating'
          ? scene.messages.slice(0, 3)
          : state === 'failed'
            ? [
                ...scene.messages.slice(0, 1),
                { ...scene.messages[1], status: 'failed' },
              ]
            : scene.messages,
  );
  const [characters, setCharacters] = useState<ChatCharacter[]>([
    { ...scene.character, kind: 'character', ownerId: 'me' },
    {
      id: 'owl',
      kind: 'character',
      name: isEnglish ? 'Owl' : 'Сова',
      initials: isEnglish ? 'O' : 'С',
      ownerId: 'sasha',
    },
  ]);
  const [editingId, setEditingId] = useState<string>();
  const [selectedId, setSelectedId] = useState(scene.persona.id);
  const [generatingId, setGeneratingId] = useState<string | null>(
    state === 'generating' ? scene.character.id : null,
  );
  const [notice, setNotice] = useState('');
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const failOnce = useRef(state === 'history-error');
  const sequence = useRef(1000);
  const active = useRef(true);

  useEffect(() => {
    active.current = true;

    return () => {
      active.current = false;
      if (timer.current) clearTimeout(timer.current);
    };
  }, []);
  const trigger = (id: string) => {
    const character = characters.find((item) => item.id === id);

    if (!character || generatingId || state === 'read-only') return;
    setGeneratingId(id);
    setNotice('');
    timer.current = setTimeout(() => {
      setMessages((current) => [
        ...current,
        {
          id: `reply-${sequence.current++}`,
          author: character,
          content: scene.reply,
          time: '21:08',
        },
      ]);
      setGeneratingId(null);
    }, 2200);
  };
  const send = (content: JSONContent) => {
    const author = [scene.persona, scene.alternate].find(
      (persona) => persona.id === selectedId,
    );

    if (!author || generatingId) return;
    setMessages((current) => [
      ...current,
      { id: `local-${sequence.current++}`, author, content, time: '21:08' },
    ]);
    trigger(characters[0].id);
  };
  const loadOlder = async (pageSize: number) => {
    await new Promise((resolve) => setTimeout(resolve, 900));
    if (!active.current) return;
    if (failOnce.current) {
      failOnce.current = false;
      throw new Error('Demo loading error');
    }
    const next = Math.max(0, oldest - pageSize);

    setMessages((current) => [...history.slice(next, oldest), ...current]);
    setOldest(next);
  };

  const editing = characters.find((character) => character.id === editingId);

  return (
    <>
      <ChatView
        title={t('title')}
        subtitle={t('subtitle')}
        users={[
          { id: 'me', name: t('you') },
          { id: 'sasha', name: isEnglish ? 'Sasha' : 'Саша' },
        ]}
        currentUserId="me"
        characters={characters}
        myPersonas={[scene.persona, scene.alternate]}
        selectedPersonaId={selectedId}
        onPersonaChange={setSelectedId}
        messages={messages}
        onSend={send}
        onTriggerCharacter={trigger}
        onEditCharacter={setEditingId}
        generatingCharacterId={generatingId}
        onStop={() => {
          if (timer.current) clearTimeout(timer.current);
          setGeneratingId(null);
          setNotice(t('stopped'));
        }}
        onEditMessage={(id, content) => {
          setMessages((current) =>
            current.map((message) =>
              message.id === id
                ? { ...message, content, edited: true }
                : message,
            ),
          );
          setNotice(t('saved'));
        }}
        onRetry={(id) => {
          setMessages((current) =>
            current.map((message) =>
              message.id === id ? { ...message, status: 'sent' } : message,
            ),
          );
          trigger(characters[0].id);
        }}
        readOnly={state === 'read-only'}
        notice={notice}
        hasOlder={oldest > 0}
        loadOlder={loadOlder}
        backgroundUrl={background ? forest : undefined}
      />

      {editing && (
        <CharacterEditor
          character={editing}
          onClose={() => setEditingId(undefined)}
          onSave={(changes) =>
            setCharacters((current) =>
              current.map((character) =>
                character.id === editing.id
                  ? { ...character, ...changes }
                  : character,
              ),
            )
          }
        />
      )}
    </>
  );
}
