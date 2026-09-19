import { auth } from './auth';
import { characters } from './characters';
import { chats } from './chats';
import { files } from './files';
import { personas } from './personas';
import { users } from './user';

export * from './common';
export * from './socket';
export type { Character } from './characters';
export type { Persona } from './personas';
export type { User } from './user';
export type { Chat } from './chats';
export type { MessageDto, SendMessageDto } from './messages';

export const api = { auth, users, personas, characters, files, chats };
