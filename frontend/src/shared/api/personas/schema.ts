import z from 'zod';

export const PersonaSchema = z.object({
  uid: z.string(),
  name: z.string(),
  description: z.string(),
  creator_uid: z.string(),
});
