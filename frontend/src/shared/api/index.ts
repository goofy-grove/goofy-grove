import { auth } from './auth';
import { characters } from './characters';
import { personas } from './personas';
import { users } from './user';

export * from './socket';

export const api = { auth, users, personas, characters };
