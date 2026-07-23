import type { ApiErrorDataSchema } from './api-error-data';
import type { z } from 'zod';

export type ApiFunction<Args = unknown> = (...args: Args[]) => Promise<unknown>;

export type ApiErrorData = z.infer<typeof ApiErrorDataSchema>;
export type ApiErrorCode = ApiErrorData['code'];
