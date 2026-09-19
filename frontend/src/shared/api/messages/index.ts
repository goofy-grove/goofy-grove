import type {
  MessageInfoSchema,
  MessageAuthorUidSchema,
  MessageSchema,
  MessageAuthorSchema,
  MessagesPageSchema,
  SendMessageSchema,
} from './schema';
import type { z } from 'zod';

export {
  MessageSchema,
  MessageInfoSchema,
  MessageAuthorSchema,
  MessageAuthorUidSchema,
  MessagesPageSchema,
  SendMessageSchema,
} from './schema';

export type MessageDto = z.infer<typeof MessageSchema>;

export type MessageAuthorDto = z.infer<typeof MessageAuthorSchema>;

export type MessagesPageDto = z.infer<typeof MessagesPageSchema>;

export type SendMessageDto = z.infer<typeof SendMessageSchema>;

export type MessageInfoDto = z.infer<typeof MessageInfoSchema>;

export type MessageAuthorUidDto = z.infer<typeof MessageAuthorUidSchema>;
