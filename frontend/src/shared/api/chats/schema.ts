import z from 'zod';

import { CharacterSchema } from '@shared/api/characters';
import { UserSchema } from '@shared/api/user';

export const ChatMemberSchema = UserSchema.extend({
  joined_at: z.iso
    .datetime({ offset: true })
    .transform((date) => new Date(date)),
  chat_uid: z.string(),
});

export const ChatCharacterSchema = CharacterSchema.extend({
  connected_at: z.iso
    .datetime({ offset: true })
    .transform((date) => new Date(date)),
  chat_uid: z.string(),
});

export const ChatSchema = z.object({
  uid: z.string(),
  name: z.string(),
  created_at: z.iso
    .datetime({ offset: true })
    .transform((date) => new Date(date)),
  creator_uid: z.string(),
  avatar_uid: z.string().nullable(),
  members: z.array(ChatMemberSchema),
  characters: z.array(ChatCharacterSchema),
});
