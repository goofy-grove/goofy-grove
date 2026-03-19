import z from 'zod';

export const UserResponseSchema = z.object({
  id: z.string(),
  username: z.string(),
});
