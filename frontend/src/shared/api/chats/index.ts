import { create } from './create';
import { getAll } from './get-all';
import { putAvatar } from './put-avatar';

export const chats = { getAll, create, putAvatar };

export type { Chat, ChatMember, ChatCharacterDto } from './types';
