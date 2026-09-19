import type {
  ChatSchema,
  ChatMemberSchema,
  ChatCharacterSchema,
} from './schema';
import type { z } from 'zod';

export type Chat = z.infer<typeof ChatSchema>;

export type ChatMember = z.infer<typeof ChatMemberSchema>;

export type ChatCharacterDto = z.infer<typeof ChatCharacterSchema>;
