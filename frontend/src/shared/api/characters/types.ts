import type { CharacterSchema } from './schema';
import type { z } from 'zod';

export type Character = z.infer<typeof CharacterSchema>;

export type UpdateCharacterPayload = {
  name?: string;
  description?: string;
};
