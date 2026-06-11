import z from 'zod';

export const UserResponseSchema = z.object({
  uid: z.string(),
  username: z.string(),
});
