import z from 'zod';

export const AuthResponseSchema = z.object({
  exp: z.number(),
  token: z.string(),
});
