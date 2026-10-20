import z from 'zod';

import { CharacterSchema } from '@shared/api/characters';
import { MessageSchema } from '@shared/api/messages';
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

export const ChatInfoSchema = z.object({
  uid: z.string(),
  name: z.string(),
  created_at: z.iso
    .datetime({ offset: true })
    .transform((date) => new Date(date)),
  creator_uid: z.string(),
  avatar_uid: z.string().nullable(),
  last_message: MessageSchema.nullable(),
});

export const ChatSchema = ChatInfoSchema.extend({
  members: z.array(ChatMemberSchema),
  characters: z.array(ChatCharacterSchema),
});

// pub uid: String,
// pub name: String,
// pub created_at: DateTime<Utc>,
// pub creator_uid: String,
// pub avatar_uid: Option<String>,
