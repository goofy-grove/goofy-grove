import z from 'zod';

export const UserSchema = z.object({
  uid: z.string(),
  username: z.string(),
});
