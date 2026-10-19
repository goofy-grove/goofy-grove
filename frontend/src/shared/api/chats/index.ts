import { create } from './create';
import { getAll } from './get-all';
import { putAvatar } from './put-avatar';
import { remove } from './remove';

export const chats = { getAll, create, putAvatar, remove };

export type { Chat, ChatMember, ChatCharacterDto } from './types';
