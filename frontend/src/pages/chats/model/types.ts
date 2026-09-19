import type { JSONContent } from '@tiptap/react';

export type ChatAdapterContext = {
  currentUserUid: string;
  locale: string;
  ownerNames: Readonly<Record<string, string>>;
};

export type ChatIdentityPersona = {
  id: string;
  kind: 'persona';
  name: string;
  owner: string;
  own: boolean;
  initials: string;
  avatarUid?: string | null;
  previewUrl?: string;
};

export type ChatIdentityCharacter = {
  id: string;
  kind: 'character';
  name: string;
  initials: string;
  avatarUid?: string | null;
  previewUrl?: string;
  ownerId?: string;
  description?: string;
};

export type ChatIdentity = ChatIdentityPersona | ChatIdentityCharacter;

export type ChatMessagePresentationState = {
  edited?: boolean;
  status?: 'sent' | 'failed';
};

export type ChatMessageData = {
  id: string;
  author: ChatIdentity | null;
  removed?: boolean;
  content: JSONContent;
  time: string;
} & ChatMessagePresentationState;

export type ChatCharacter = Extract<ChatIdentity, { kind: 'character' }>;

export type ChatUser = { id: string; name: string };

export type ChatPersona = Extract<ChatIdentity, { kind: 'persona' }>;

export type EditChatMessage = (id: string, content: JSONContent) => void;

export type RetryChatMessage = (id: string) => void;

export type LoadChatHistory = (pageSize: number) => Promise<void>;
