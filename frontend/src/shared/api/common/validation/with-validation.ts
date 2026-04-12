import * as z from 'zod';

import type { ApiFunction } from '@shared/api/common/types';

import { ResponseErrorSchema, ResponseOkSchema } from './schemas';

export const withValidation = <
  ApiFn extends ApiFunction<Args>,
  Schema extends z.ZodType,
  // NOTE: For better type inference of the result function arguments
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  Args = any,
>(
  schema: Schema,
  fn: ApiFn,
) => {
  const ResponseSchema = z.discriminatedUnion('error', [
    ResponseOkSchema.extend({ data: schema }),
    ResponseErrorSchema,
  ]);

  return async (
    ...args: Parameters<ApiFn>
  ): Promise<z.infer<typeof ResponseSchema>> => {
    const fnResult = await fn(...args);

    const result = ResponseSchema.parse(fnResult);

    return result;
  };
};
