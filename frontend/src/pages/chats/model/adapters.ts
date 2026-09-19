import { parseMessageContent, serializeMessageContent } from '@pages/chats/lib';

import type { Character } from '@shared/api/characters';
import type { Chat } from '@shared/api/chats';
import type { MessageDto, SendMessageDto } from '@shared/api/messages';
import type { Persona } from '@shared/api/personas';

import type {
  ChatAdapterContext,
  ChatCharacter,
  ChatMessageData,
  ChatPersona,
} from './types';
import type { JSONContent } from '@tiptap/react';

const initials = (name: string) =>
  name
    .trim()
    .split(/\s+/)
    .slice(0, 2)
    .map((part) => part[0] ?? '')
    .join('')
    .toLocaleUpperCase();

export const toChatCharacter = (character: Character): ChatCharacter => ({
  id: character.uid,
  kind: 'character',
  name: character.name,
  description: character.description,
  ownerId: character.creator_uid,
  initials: initials(character.name),
  avatarUid: character.avatar_uid,
});

export const toChatPersona = (
  persona: Persona,
  context: ChatAdapterContext,
): ChatPersona => ({
  id: persona.uid,
  kind: 'persona',
  name: persona.name,
  owner: context.ownerNames[persona.creator_uid] ?? persona.creator_uid,
  own: persona.creator_uid === context.currentUserUid,
  initials: initials(persona.name),
  avatarUid: persona.avatar_uid,
});

export const toChatMessage = (
  message: MessageDto,
  context: ChatAdapterContext,
): ChatMessageData => ({
  id: message.uid,
  author:
    message.author === null
      ? null
      : message.author.kind === 'persona'
        ? toChatPersona(message.author, context)
        : toChatCharacter(message.author),
  content: parseMessageContent(message.is_removed ? '' : message.content),
  removed: message.is_removed,
  time: new Intl.DateTimeFormat(context.locale, {
    hour: '2-digit',
    minute: '2-digit',
  }).format(message.created_at),
});

export const toChatSummary = (chat: Chat) => ({
  uid: chat.uid,
  title: chat.name,
  avatarUid: chat.avatar_uid,
  users: chat.members.map((user) => ({ id: user.uid, name: user.username })),
  characters: chat.characters.map(toChatCharacter),
});

export const toSendMessage = (
  content: JSONContent,
  author: SendMessageDto['author'],
  replyToUid: string | null = null,
): SendMessageDto => ({
  content: serializeMessageContent(content),
  author,
  reply_to_message_uid: replyToUid,
});
