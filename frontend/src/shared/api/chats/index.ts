import { create } from './create';
import { getAll } from './get-all';
import { putAvatar } from './put-avatar';
import { remove } from './remove';
import { update } from './update';

export const chats = { getAll, create, putAvatar, remove, update };

export type { Chat, ChatMember, ChatCharacterDto } from './types';
