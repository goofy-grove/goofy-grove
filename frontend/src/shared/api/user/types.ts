import type { UserSchema } from './schema';
import type { z } from 'zod';

export type User = z.infer<typeof UserSchema>;
