import z from 'zod';

import { CharacterSchema } from '@shared/api/characters';
import { PersonaSchema } from '@shared/api/personas';

const timestamp = z.iso
  .datetime({ offset: true })
  .transform((value) => new Date(value));

export const MessageAuthorUidSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('persona'), uid: z.string() }),
  z.object({ kind: z.literal('character'), uid: z.string() }),
]);

export const MessageAuthorSchema = z.discriminatedUnion('kind', [
  PersonaSchema.extend({ kind: z.literal('persona') }),
  CharacterSchema.extend({ kind: z.literal('character') }),
]);

export const MessageInfoSchema = z.object({
  uid: z.string(),
  author_uid: MessageAuthorUidSchema.nullable(),
  content: z.string(),
  created_at: timestamp,
  chat_uid: z.string(),
  reply_to_message_uid: z.string().nullable(),
  is_removed: z.boolean(),
});

export const MessageSchema = z.object({
  uid: z.string(),
  author: MessageAuthorSchema.nullable(),
  content: z.string(),
  created_at: timestamp,
  chat_uid: z.string(),
  reply_to_message: MessageInfoSchema.nullable(),
  is_removed: z.boolean(),
});

export const MessagesPageSchema = z.object({
  messages: z.array(MessageSchema),
  next_page: z.string().nullable(),
});

export const SendMessageSchema = z.object({
  content: z.string(),
  author: MessageAuthorUidSchema,
  reply_to_message_uid: z.string().nullish(),
});
