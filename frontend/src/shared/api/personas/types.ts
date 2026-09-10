import type { PersonaSchema } from './schema';
import type { z } from 'zod';

export type Persona = z.infer<typeof PersonaSchema>;

export type UpdatePersonaPayload = {
  name?: string;
  description?: string;
};
