import z from 'zod';

export const CharacterSchema = z.object({
  uid: z.string(),
  name: z.string(),
  description: z.string(),
  creator_uid: z.string(),
});
