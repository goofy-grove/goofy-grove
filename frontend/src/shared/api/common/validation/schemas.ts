import * as z from 'zod';

import { ApiErrorDataSchema } from './api-error-data';

export const ResponseErrorSchema = z.object({
  error: z.literal(true),
  data: ApiErrorDataSchema,
});

export const ResponseOkSchema = z.object({
  data: z.unknown(),
  error: z.literal(false),
});
